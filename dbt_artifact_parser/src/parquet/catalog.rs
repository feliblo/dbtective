use super::columns::ColumnRow;
use super::IndexLayout;
use crate::catalog::columns::CatalogColumn;
use crate::catalog::nodes::{CatalogNode, CatalogNodeBase};
use crate::catalog::parse_catalog::{Catalog, CatalogMetadata};
use crate::catalog::resource_metadata::CatalogResourceMetadata;
use crate::catalog::source::CatalogSource;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

const V1_SCHEMA_VERSION: &str = "https://schemas.getdbt.com/dbt/catalog/v1.json";

struct Relation {
    resource_type: String,
    database: String,
    schema: String,
    name: String,
    table_type: String,
    owner: Option<String>,
    comment: Option<String>,
}

impl Catalog {
    /// # Errors
    /// If the index is missing or unreadable.
    pub fn from_index<P: AsRef<Path>>(index_dir: P) -> Result<Self> {
        let dir = index_dir.as_ref();
        let layout = IndexLayout::discover(dir)?.ok_or_else(|| {
            anyhow::anyhow!(
                "No dbt Parquet index found at {}.\n\
                 Generate one with a dbt v2 command such as 'dbt build --write-index'.",
                dir.display()
            )
        })?;
        Self::from_layout(&layout)
    }

    /// Only relations that actually carry warehouse column types are included,
    /// mirroring `catalog.json`, which only lists relations that exist.
    ///
    /// # Errors
    /// If a required table is missing or a row cannot be decoded.
    pub fn from_layout(layout: &IndexLayout) -> Result<Self> {
        let metadata = read_metadata(layout)?;
        let relations = read_relations(layout, read_project_name(layout)?.as_deref())?;
        let mut columns = read_typed_columns(layout)?;

        let mut catalog = Self {
            metadata,
            nodes: HashMap::new(),
            sources: HashMap::new(),
            errors: None,
        };

        for (unique_id, relation) in relations {
            let Some(cols) = columns.remove(&unique_id) else {
                continue;
            };
            let metadata = CatalogResourceMetadata {
                type_: relation.table_type,
                database: relation.database,
                schema: relation.schema,
                name: relation.name,
                comment: relation.comment,
                owner: relation.owner,
            };

            if relation.resource_type == "source" {
                catalog.sources.insert(
                    unique_id.clone(),
                    CatalogSource {
                        unique_id,
                        metadata,
                        columns: cols,
                        stats: HashMap::new(),
                    },
                );
            } else {
                let base = CatalogNodeBase {
                    unique_id: unique_id.clone(),
                    metadata,
                    columns: cols,
                    stats: HashMap::new(),
                };
                if let Ok(node) = CatalogNode::from_base(base) {
                    catalog.nodes.insert(unique_id, node);
                }
            }
        }

        Ok(catalog)
    }
}

fn read_metadata(layout: &IndexLayout) -> Result<CatalogMetadata> {
    let mut metadata = CatalogMetadata {
        dbt_schema_version: V1_SCHEMA_VERSION.to_string(),
        dbt_version: String::new(),
        generated_at: String::new(),
        invocation_id: None,
        invocation_started_at: None,
        env: HashMap::new(),
    };
    if let Some(table) = layout.optional("dbt.project")? {
        table.for_each_row(|row| {
            metadata.dbt_version = row.str("dbt_version").unwrap_or_default();
            metadata.generated_at = row.str("ingested_at").unwrap_or_default();
            Ok(())
        })?;
    }
    Ok(metadata)
}

fn read_project_name(layout: &IndexLayout) -> Result<Option<String>> {
    let Some(table) = layout.optional("dbt.project")? else {
        return Ok(None);
    };
    let mut name = None;
    table.for_each_row(|row| {
        name = row.non_empty_str("project_name");
        Ok(())
    })?;
    Ok(name)
}

/// Package models are dropped so catalog nodes always have a manifest counterpart.
fn read_relations(
    layout: &IndexLayout,
    project_name: Option<&str>,
) -> Result<HashMap<String, Relation>> {
    let mut out = HashMap::new();
    let table = layout.require("dbt.nodes")?;
    table.for_each_row(|row| {
        let Some(unique_id) = row.non_empty_str("unique_id") else {
            return Ok(());
        };
        if let Some(project) = project_name {
            if row.str("package_name").as_deref() != Some(project) {
                return Ok(());
            }
        }
        let materialized = row.str("materialized").unwrap_or_default();
        let table_type = row.non_empty_str("table_role").unwrap_or_else(|| {
            if materialized.eq_ignore_ascii_case("view") {
                "VIEW".to_string()
            } else {
                "BASE TABLE".to_string()
            }
        });
        out.insert(
            unique_id,
            Relation {
                resource_type: row.str("resource_type").unwrap_or_default(),
                database: row.str("database_name").unwrap_or_default(),
                schema: row.str("schema_name").unwrap_or_default(),
                name: row.str("name").unwrap_or_default(),
                table_type,
                owner: row.non_empty_str("owner"),
                comment: row.non_empty_str("description"),
            },
        );
        Ok(())
    })?;
    Ok(out)
}

/// A relation is in the catalog only if the warehouse reported at least one of
/// its columns, mirroring `catalog.json`, which lists only relations that exist.
/// All of that relation's columns are then included, typed or not.
fn read_typed_columns(
    layout: &IndexLayout,
) -> Result<HashMap<String, HashMap<String, CatalogColumn>>> {
    let mut out: HashMap<String, HashMap<String, CatalogColumn>> = HashMap::new();
    let mut built: HashSet<String> = HashSet::new();
    let Some(table) = layout.optional("dbt.node_columns")? else {
        return Ok(out);
    };
    table.for_each_row(|row| {
        if let Some(col) = ColumnRow::read(row) {
            if col.has_warehouse_type() {
                built.insert(col.unique_id.clone());
            }
            out.entry(col.unique_id.clone())
                .or_default()
                .insert(col.name.clone(), col.to_catalog_column());
        }
        Ok(())
    })?;
    out.retain(|id, _| built.contains(id));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::catalog::parse_catalog::Catalog;
    use crate::parquet::test_writer::{Cell, ColumnKind, IndexBuilder, TableBuilder};
    use tempfile::TempDir;
    use ColumnKind::{Int64, Utf8};

    const PROJECT: &str = "my_project";

    fn project_table() -> TableBuilder {
        TableBuilder::new(&[
            ("project_name", Utf8),
            ("dbt_version", Utf8),
            ("adapter_type", Utf8),
        ])
        .row(&[
            Cell::Str(PROJECT),
            Cell::Str("2.0.0-beta.1"),
            Cell::Str("duckdb"),
        ])
    }

    fn nodes_table() -> TableBuilder {
        TableBuilder::new(&[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
            ("database_name", Utf8),
            ("schema_name", Utf8),
            ("materialized", Utf8),
            ("description", Utf8),
        ])
        .row(&[
            Cell::Str("model.my_project.orders"),
            Cell::Str("orders"),
            Cell::Str("model"),
            Cell::Str(PROJECT),
            Cell::Str("dbt"),
            Cell::Str("marts"),
            Cell::Str("table"),
            Cell::Str("All orders"),
        ])
        .row(&[
            Cell::Str("model.my_project.stg_orders"),
            Cell::Str("stg_orders"),
            Cell::Str("model"),
            Cell::Str(PROJECT),
            Cell::Str("dbt"),
            Cell::Str("staging"),
            Cell::Str("view"),
        ])
        .row(&[
            Cell::Str("source.my_project.raw.orders"),
            Cell::Str("orders"),
            Cell::Str("source"),
            Cell::Str(PROJECT),
            Cell::Str("dbt"),
            Cell::Str("raw"),
        ])
        .row(&[
            Cell::Str("model.dbt_utils.vendored"),
            Cell::Str("vendored"),
            Cell::Str("model"),
            Cell::Str("dbt_utils"),
            Cell::Str("dbt"),
            Cell::Str("marts"),
            Cell::Str("table"),
        ])
    }

    fn node_columns_table() -> TableBuilder {
        TableBuilder::new(&[
            ("unique_id", Utf8),
            ("column_name", Utf8),
            ("column_index", Int64),
            ("data_type", Utf8),
            ("description", Utf8),
        ])
        .row(&[
            Cell::Str("model.my_project.orders"),
            Cell::Str("order_id"),
            Cell::Int(0),
            Cell::Str("INTEGER"),
            Cell::Str("Primary key"),
        ])
        .row(&[
            Cell::Str("model.my_project.orders"),
            Cell::Str("status"),
            Cell::Int(1),
            Cell::Null,
            Cell::Null,
        ])
        .row(&[
            Cell::Str("model.my_project.stg_orders"),
            Cell::Str("order_id"),
            Cell::Int(0),
            Cell::Null,
            Cell::Null,
        ])
        .row(&[
            Cell::Str("source.my_project.raw.orders"),
            Cell::Str("id"),
            Cell::Int(0),
            Cell::Str("BIGINT"),
            Cell::Null,
        ])
        .row(&[
            Cell::Str("model.dbt_utils.vendored"),
            Cell::Str("id"),
            Cell::Int(0),
            Cell::Str("BIGINT"),
            Cell::Null,
        ])
    }

    /// `orders` and the source were built; `stg_orders` was not, and the
    /// `dbt_utils` model belongs to a package.
    fn index() -> TempDir {
        let dir = TempDir::new().unwrap();
        IndexBuilder::new(&dir.path().join("index"))
            .unwrap()
            .table("dbt.project", project_table())
            .unwrap()
            .table("dbt.nodes", nodes_table())
            .unwrap()
            .table("dbt.node_columns", node_columns_table())
            .unwrap();
        dir
    }

    fn catalog(dir: &TempDir) -> Catalog {
        Catalog::from_index(dir.path().join("index")).expect("index should build a catalog")
    }

    #[test]
    fn only_built_relations_are_in_the_catalog() {
        let dir = index();
        let catalog = catalog(&dir);
        assert!(catalog.nodes.contains_key("model.my_project.orders"));
        assert!(
            !catalog.nodes.contains_key("model.my_project.stg_orders"),
            "a relation the warehouse never reported is not in the catalog"
        );
    }

    #[test]
    fn a_built_relation_keeps_all_its_columns() {
        let dir = index();
        let catalog = catalog(&dir);
        let orders = &catalog.nodes["model.my_project.orders"];
        let columns = &orders.get_base().columns;
        assert_eq!(columns.len(), 2, "untyped columns are still listed");
        assert_eq!(columns["order_id"].type_, "INTEGER");
        assert_eq!(columns["order_id"].comment.as_deref(), Some("Primary key"));
        assert_eq!(columns["status"].type_, "");
    }

    #[test]
    fn sources_land_in_sources() {
        let dir = index();
        let catalog = catalog(&dir);
        let source = &catalog.sources["source.my_project.raw.orders"];
        assert_eq!(source.get_name(), "orders");
        assert_eq!(source.metadata.schema, "raw");
        assert!(catalog.nodes.keys().all(|k| !k.starts_with("source.")));
    }

    /// Package models have no manifest counterpart after project filtering, and
    /// would otherwise produce "no matching manifest node" warnings for each one.
    #[test]
    fn package_models_are_excluded() {
        let dir = index();
        let catalog = catalog(&dir);
        assert!(!catalog.nodes.contains_key("model.dbt_utils.vendored"));
    }

    #[test]
    fn relation_metadata_comes_from_the_node_row() {
        let dir = index();
        let catalog = catalog(&dir);
        let metadata = &catalog.nodes["model.my_project.orders"].get_base().metadata;
        assert_eq!(metadata.database, "dbt");
        assert_eq!(metadata.schema, "marts");
        assert_eq!(metadata.name, "orders");
        assert_eq!(metadata.type_, "BASE TABLE");
        assert_eq!(metadata.comment.as_deref(), Some("All orders"));
    }

    #[test]
    fn a_view_is_typed_as_a_view() {
        let dir = TempDir::new().unwrap();
        let nodes = TableBuilder::new(&[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
            ("materialized", Utf8),
        ])
        .row(&[
            Cell::Str("model.my_project.stg_orders"),
            Cell::Str("stg_orders"),
            Cell::Str("model"),
            Cell::Str(PROJECT),
            Cell::Str("view"),
        ]);
        let node_columns = TableBuilder::new(&[
            ("unique_id", Utf8),
            ("column_name", Utf8),
            ("data_type", Utf8),
        ])
        .row(&[
            Cell::Str("model.my_project.stg_orders"),
            Cell::Str("order_id"),
            Cell::Str("INTEGER"),
        ]);
        IndexBuilder::new(&dir.path().join("index"))
            .unwrap()
            .table("dbt.nodes", nodes)
            .unwrap()
            .table("dbt.node_columns", node_columns)
            .unwrap();

        let catalog = catalog(&dir);
        assert_eq!(
            catalog.nodes["model.my_project.stg_orders"]
                .get_base()
                .metadata
                .type_,
            "VIEW"
        );
    }

    #[test]
    fn missing_index_is_an_error() {
        let dir = TempDir::new().unwrap();
        let err = Catalog::from_index(dir.path().join("nope"))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("No dbt Parquet index"),
            "unexpected error: {err}"
        );
    }
}
