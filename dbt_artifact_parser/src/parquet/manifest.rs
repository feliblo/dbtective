use super::columns::ColumnRow;
use super::nodes::{NodeRow, TestRowMetadata};
use super::objects;
use super::IndexLayout;
use crate::manifest::dbt_objects::column::Column;
use crate::manifest::nodes::DependsOn;
use crate::manifest::parse_manifest::{Manifest, ManifestMetadata, Quoting};
use crate::manifest::udf::UDF;
use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const V2_SCHEMA_VERSION: &str = "https://schemas.getdbt.com/dbt/manifest/v12.json";

/// `raw_code` and `loader` are written to `manifest.json` but omitted from the
/// index. Only those keys are deserialized; the rest of the document is skipped.
pub struct JsonSupplement {
    raw_code: HashMap<String, String>,
    loader: HashMap<String, String>,
    columns: HashMap<String, HashMap<String, Column>>,
    macro_ids: HashSet<String>,
    functions: HashMap<String, UDF>,
}

impl JsonSupplement {
    /// # Errors
    /// If the file cannot be read or is not valid JSON.
    pub fn read(path: &Path) -> Result<Self> {
        #[derive(Deserialize)]
        struct Doc {
            #[serde(default)]
            nodes: HashMap<String, Entry>,
            #[serde(default)]
            sources: HashMap<String, Entry>,
            #[serde(default)]
            #[allow(clippy::zero_sized_map_values)]
            macros: HashMap<String, serde::de::IgnoredAny>,
            #[serde(default)]
            functions: HashMap<String, UDF>,
        }
        #[derive(Deserialize)]
        struct Entry {
            raw_code: Option<String>,
            loader: Option<String>,
            columns: Option<HashMap<String, Column>>,
        }

        let file = File::open(path)?;
        let doc: Doc = serde_json::from_reader(BufReader::new(file))?;

        let mut raw_code = HashMap::new();
        let mut loader = HashMap::new();
        let mut columns = HashMap::new();
        for (id, entry) in doc.nodes.into_iter().chain(doc.sources) {
            if let Some(code) = entry.raw_code.filter(|c| !c.is_empty()) {
                raw_code.insert(id.clone(), code);
            }
            if let Some(l) = entry.loader.filter(|l| !l.is_empty()) {
                loader.insert(id.clone(), l);
            }
            if let Some(c) = entry.columns {
                columns.insert(id, c);
            }
        }
        Ok(Self {
            raw_code,
            loader,
            columns,
            macro_ids: doc.macros.into_keys().collect(),
            functions: doc.functions,
        })
    }
}

impl Manifest {
    /// Reads only `dbt.project`, so it stays cheap on large indexes.
    ///
    /// # Errors
    /// If the index directory cannot be read.
    pub fn peek_index_dbt_version<P: AsRef<Path>>(index_dir: P) -> Result<Option<String>> {
        let Some(layout) = IndexLayout::discover(index_dir)? else {
            return Ok(None);
        };
        let Some(table) = layout.optional("dbt.project")? else {
            return Ok(None);
        };
        let mut version = None;
        table.for_each_row(|row| {
            version = row.non_empty_str("dbt_version");
            Ok(())
        })?;
        Ok(version)
    }

    /// # Errors
    /// If the index is missing, unreadable, or has no `dbt.nodes` table.
    pub fn from_index<P: AsRef<Path>>(index_dir: P) -> Result<Self> {
        Self::from_index_with_json(index_dir, None::<&PathBuf>)
    }

    /// dbt v2 writes `manifest.json` alongside the index but leaves `raw_code` and
    /// `loader` out of the index, so those are read back from the JSON when it is
    /// available. Without it those fields stay empty.
    ///
    /// # Errors
    /// If the index is missing, unreadable, or has no `dbt.nodes` table.
    pub fn from_index_with_json<P: AsRef<Path>, J: AsRef<Path>>(
        index_dir: P,
        manifest_json: Option<&J>,
    ) -> Result<Self> {
        let dir = index_dir.as_ref();
        let layout = IndexLayout::discover(dir)?.ok_or_else(|| {
            anyhow::anyhow!(
                "No dbt Parquet index found at {}.\n\
                 Generate one with a dbt v2 command such as 'dbt build --write-index'.",
                dir.display()
            )
        })?;
        let supplement = manifest_json
            .map(AsRef::as_ref)
            .filter(|p| p.is_file())
            .map(JsonSupplement::read)
            .transpose()?;
        Self::from_layout_with(&layout, supplement)
    }

    /// # Errors
    /// If a required table is missing or a row cannot be decoded.
    pub fn from_layout(layout: &IndexLayout) -> Result<Self> {
        Self::from_layout_with(layout, None)
    }

    /// # Errors
    /// If a required table is missing or a row cannot be decoded.
    pub fn from_layout_with(
        layout: &IndexLayout,
        supplement: Option<JsonSupplement>,
    ) -> Result<Self> {
        let supplement_ref = supplement.as_ref();
        let metadata = read_metadata(layout)?;
        let (parent_map, child_map) = read_edges(layout)?;
        let mut columns = read_columns(layout)?;
        let test_metadata = read_test_metadata(layout)?;

        let mut manifest = Self {
            metadata,
            parent_map,
            child_map,
            ..Self::default()
        };

        read_nodes(
            layout,
            &mut manifest,
            supplement_ref,
            &mut columns,
            &test_metadata,
        )?;

        read_into(
            layout,
            "dbt.macros",
            &mut manifest.macros,
            objects::read_macro,
        )?;
        // dbt generates a macro per snapshot which the index lists but manifest.json does not.
        // The index has no functions table at all, so those come straight from the JSON.
        if let Some(s) = supplement {
            manifest.macros.retain(|id, _| s.macro_ids.contains(id));
            manifest.functions = s.functions;
        }
        read_into(
            layout,
            "dbt.exposures",
            &mut manifest.exposures,
            objects::read_exposure,
        )?;
        read_into(
            layout,
            "dbt.groups",
            &mut manifest.groups,
            objects::read_group,
        )?;
        read_into(
            layout,
            "dbt.unit_tests",
            &mut manifest.unit_tests,
            objects::read_unit_test,
        )?;
        read_into(
            layout,
            "dbt.semantic_models",
            &mut manifest.semantic_models,
            objects::read_semantic_model,
        )?;
        read_into(
            layout,
            "dbt.metrics",
            &mut manifest.metrics,
            objects::read_metric,
        )?;
        read_into(
            layout,
            "dbt.saved_queries",
            &mut manifest.saved_queries,
            objects::read_saved_query,
        )?;

        manifest.filter_to_project();
        Ok(manifest)
    }
}

fn read_nodes(
    layout: &IndexLayout,
    manifest: &mut Manifest,
    supplement: Option<&JsonSupplement>,
    columns: &mut HashMap<String, HashMap<String, Column>>,
    test_metadata: &HashMap<String, TestRowMetadata>,
) -> Result<()> {
    let nodes_table = layout.require("dbt.nodes")?;
    nodes_table.for_each_row(|row| {
        let Some(mut node_row) = NodeRow::read(row) else {
            return Ok(());
        };
        if let Some(s) = supplement {
            let id = &node_row.unique_id;
            node_row.fill_gaps(s.raw_code.get(id), s.loader.get(id));
        }
        let unique_id = node_row.unique_id.clone();
        // Declared columns come from the JSON when available: the index merges
        // warehouse-discovered columns into the same table, which the JSON
        // manifest never lists. Some(empty) rather than None mirrors `columns: {}`.
        let node_columns = Some(
            supplement
                .and_then(|s| s.columns.get(&unique_id).cloned())
                .unwrap_or_else(|| columns.remove(&unique_id).unwrap_or_default()),
        );

        if node_row.is_source() {
            manifest
                .sources
                .insert(unique_id, node_row.into_source(node_columns));
            return Ok(());
        }

        let parents = manifest.parent_map.get(&unique_id);
        let depends_on = DependsOn {
            nodes: parents.map(|p| filter_deps(p, false)),
            macros: parents.map(|p| filter_deps(p, true)),
        };
        let meta = test_metadata.get(&unique_id).map(|m| TestRowMetadata {
            name: m.name.clone(),
            namespace: m.namespace.clone(),
            kwargs: m.kwargs.clone(),
            column_name: m.column_name.clone(),
            attached_node: m.attached_node.clone(),
        });
        if let Some(node) = node_row.into_node(node_columns, depends_on, meta) {
            manifest.nodes.insert(unique_id, node);
        }
        Ok(())
    })
}

fn filter_deps(parents: &[String], macros: bool) -> Vec<String> {
    parents
        .iter()
        .filter(|p| p.starts_with("macro.") == macros)
        .cloned()
        .collect()
}

fn read_into<T, F>(
    layout: &IndexLayout,
    table: &str,
    target: &mut HashMap<String, T>,
    read: F,
) -> Result<()>
where
    F: Fn(&super::IndexRow) -> Option<(String, T)>,
{
    let Some(table) = layout.optional(table)? else {
        return Ok(());
    };
    table.for_each_row(|row| {
        if let Some((id, value)) = read(row) {
            target.insert(id, value);
        }
        Ok(())
    })
}

fn read_metadata(layout: &IndexLayout) -> Result<ManifestMetadata> {
    let mut metadata = ManifestMetadata {
        dbt_schema_version: V2_SCHEMA_VERSION.to_string(),
        ..ManifestMetadata::default()
    };

    let Some(table) = layout.optional("dbt.project")? else {
        return Ok(metadata);
    };
    table.for_each_row(|row| {
        metadata.project_name = row.non_empty_str("project_name");
        metadata.project_id = row.non_empty_str("project_id");
        metadata.adapter_type = row.non_empty_str("adapter_type");
        metadata.dbt_version = row.str("dbt_version").unwrap_or_default();
        if let Some(q) = row.json("quoting") {
            metadata.quoting = Quoting {
                database: q.get("database").and_then(serde_json::Value::as_bool),
                schema: q.get("schema").and_then(serde_json::Value::as_bool),
                identifier: q.get("identifier").and_then(serde_json::Value::as_bool),
                column: q.get("column").and_then(serde_json::Value::as_bool),
            };
        }
        Ok(())
    })?;
    Ok(metadata)
}

type EdgeMaps = (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>);

fn read_edges(layout: &IndexLayout) -> Result<EdgeMaps> {
    let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_map: HashMap<String, Vec<String>> = HashMap::new();

    let Some(table) = layout.optional("dbt.edges")? else {
        return Ok((parent_map, child_map));
    };
    table.for_each_row(|row| {
        let (Some(parent), Some(child)) = (
            row.non_empty_str("parent_unique_id"),
            row.non_empty_str("child_unique_id"),
        ) else {
            return Ok(());
        };
        parent_map
            .entry(child.clone())
            .or_default()
            .push(parent.clone());
        child_map.entry(parent).or_default().push(child);
        Ok(())
    })?;
    Ok((parent_map, child_map))
}

fn read_columns(layout: &IndexLayout) -> Result<HashMap<String, HashMap<String, Column>>> {
    let mut out: HashMap<String, HashMap<String, Column>> = HashMap::new();
    let Some(table) = layout.optional("dbt.node_columns")? else {
        return Ok(out);
    };
    table.for_each_row(|row| {
        if let Some(col) = ColumnRow::read(row) {
            out.entry(col.unique_id.clone())
                .or_default()
                .insert(col.name.clone(), col.to_manifest_column());
        }
        Ok(())
    })?;
    Ok(out)
}

fn read_test_metadata(layout: &IndexLayout) -> Result<HashMap<String, TestRowMetadata>> {
    let mut out = HashMap::new();
    let Some(table) = layout.optional("dbt.test_metadata")? else {
        return Ok(out);
    };
    table.for_each_row(|row| {
        if let Some((id, meta)) = TestRowMetadata::read(row) {
            out.insert(id, meta);
        }
        Ok(())
    })?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{filter_deps, Manifest};
    use crate::parquet::test_writer::{Cell, ColumnKind, IndexBuilder, TableBuilder};
    use std::io::Write;
    use tempfile::TempDir;
    use ColumnKind::Utf8;

    const PROJECT: &str = "my_project";

    fn project_with_index_and_json() -> TempDir {
        let dir = TempDir::new().unwrap();

        let project = TableBuilder::new(&[("project_name", Utf8), ("dbt_version", Utf8)])
            .row(&[Cell::Str(PROJECT), Cell::Str("2.0.0-beta.1")]);

        let nodes = TableBuilder::new(&[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
            ("raw_code", Utf8),
        ])
        .row(&[
            Cell::Str("model.my_project.orders"),
            Cell::Str("orders"),
            Cell::Str("model"),
            Cell::Str(PROJECT),
            Cell::Null,
        ])
        .row(&[
            Cell::Str("source.my_project.raw.orders"),
            Cell::Str("orders"),
            Cell::Str("source"),
            Cell::Str(PROJECT),
            Cell::Null,
        ]);

        let macros = TableBuilder::new(&[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("package_name", Utf8),
            ("macro_sql", Utf8),
        ])
        .row(&[
            Cell::Str("macro.my_project.cents_to_dollars"),
            Cell::Str("cents_to_dollars"),
            Cell::Str(PROJECT),
            Cell::Str("{% macro cents_to_dollars() %}{% endmacro %}"),
        ])
        .row(&[
            Cell::Str("macro.my_project.snapshot_orders"),
            Cell::Str("snapshot_orders"),
            Cell::Str(PROJECT),
            Cell::Str("select * from x"),
        ]);

        IndexBuilder::new(&dir.path().join("index"))
            .unwrap()
            .table("dbt.project", project)
            .unwrap()
            .table("dbt.nodes", nodes)
            .unwrap()
            .table("dbt.macros", macros)
            .unwrap();

        let mut file = std::fs::File::create(dir.path().join("manifest.json")).unwrap();
        write!(
            file,
            r#"{{
              "metadata": {{"dbt_version": "2.0.0-beta.1", "project_name": "my_project"}},
              "nodes": {{
                "model.my_project.orders": {{
                  "raw_code": "select * from {{{{ ref('stg_orders') }}}}",
                  "columns": {{"order_id": {{"name": "order_id", "description": "Primary key", "tags": []}}}}
                }}
              }},
              "sources": {{
                "source.my_project.raw.orders": {{"loader": "fivetran"}}
              }},
              "macros": {{"macro.my_project.cents_to_dollars": {{}}}},
              "functions": {{
                "function.my_project.is_positive_int": {{
                  "name": "is_positive_int", "meta": {{}}, "schema": "marts",
                  "package_name": "my_project", "path": "functions/is_positive_int.sql",
                  "original_file_path": "functions/is_positive_int.sql",
                  "unique_id": "function.my_project.is_positive_int", "fqn": ["my_project"],
                  "alias": "is_positive_int", "checksum": {{}},
                  "returns": {{"data_type": "boolean"}}, "created_at": 0.0,
                  "config": {{"enabled": true}}, "unrendered_config": {{}}
                }}
              }}
            }}"#
        )
        .unwrap();
        dir
    }

    fn load(dir: &TempDir) -> Manifest {
        Manifest::from_index_with_json(
            dir.path().join("index"),
            Some(&dir.path().join("manifest.json")),
        )
        .expect("index plus manifest should load")
    }

    #[test]
    fn raw_code_is_recovered_from_the_json_manifest() {
        let dir = project_with_index_and_json();
        let manifest = load(&dir);
        let raw_code = manifest.nodes["model.my_project.orders"]
            .get_base()
            .raw_code
            .as_deref();
        assert_eq!(raw_code, Some("select * from {{ ref('stg_orders') }}"));
    }

    #[test]
    fn loader_is_recovered_from_the_json_manifest() {
        let dir = project_with_index_and_json();
        let manifest = load(&dir);
        assert_eq!(
            manifest.sources["source.my_project.raw.orders"]
                .loader
                .as_deref(),
            Some("fivetran")
        );
    }

    #[test]
    fn functions_come_from_the_json_manifest() {
        let dir = project_with_index_and_json();
        let manifest = load(&dir);
        assert!(manifest
            .functions
            .contains_key("function.my_project.is_positive_int"));
    }

    #[test]
    fn generated_snapshot_macros_are_dropped() {
        let dir = project_with_index_and_json();
        let manifest = load(&dir);
        assert!(manifest
            .macros
            .contains_key("macro.my_project.cents_to_dollars"));
        assert!(!manifest
            .macros
            .contains_key("macro.my_project.snapshot_orders"));
    }

    #[test]
    fn declared_columns_come_from_the_json_manifest() {
        let dir = project_with_index_and_json();
        let manifest = load(&dir);
        let columns = manifest.nodes["model.my_project.orders"]
            .get_base()
            .columns
            .as_ref()
            .expect("columns should be present");
        assert_eq!(columns.len(), 1);
        assert_eq!(
            columns["order_id"].description.as_deref(),
            Some("Primary key")
        );
    }

    #[test]
    fn the_index_loads_without_a_json_manifest() {
        let dir = project_with_index_and_json();
        let manifest = Manifest::from_index(dir.path().join("index")).unwrap();
        assert!(manifest.nodes.contains_key("model.my_project.orders"));
        assert_eq!(
            manifest.nodes["model.my_project.orders"]
                .get_base()
                .raw_code,
            None
        );
        assert!(manifest.functions.is_empty());
    }

    #[test]
    fn peeks_the_version_without_loading_the_index() {
        let dir = project_with_index_and_json();
        let version = Manifest::peek_index_dbt_version(dir.path().join("index")).unwrap();
        assert_eq!(version.as_deref(), Some("2.0.0-beta.1"));
    }

    #[test]
    fn peeking_a_directory_without_an_index_is_not_an_error() {
        let dir = TempDir::new().unwrap();
        assert_eq!(Manifest::peek_index_dbt_version(dir.path()).unwrap(), None);
    }

    #[test]
    fn macro_dependencies_are_split_from_node_dependencies() {
        let parents = vec![
            "model.my_project.stg_orders".to_string(),
            "macro.dbt.current_timestamp".to_string(),
        ];
        assert_eq!(
            filter_deps(&parents, false),
            vec!["model.my_project.stg_orders"]
        );
        assert_eq!(
            filter_deps(&parents, true),
            vec!["macro.dbt.current_timestamp"]
        );
    }
}
