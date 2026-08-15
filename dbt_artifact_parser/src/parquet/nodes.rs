//! `dbt.nodes` -> `Node` / `Source`. One flat table carries every resource type.

use super::reader::IndexRow;
use crate::manifest::dbt_objects::column::Column;
use crate::manifest::dbt_objects::Meta;
use crate::manifest::materialization::Materialization;
use crate::manifest::nodes::{
    Analysis, CompiledNodeFields, Contract, DependsOn, FileHash, HookNode, Model, Node, NodeBase,
    NodeConfig, Seed, Snapshot, SqlOperation, Test, TestMetadata,
};
use crate::manifest::source::{FreshnessThreshold, Source, SourceFreshness};
use std::collections::HashMap;

pub struct NodeRow {
    pub unique_id: String,
    pub name: String,
    pub resource_type: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub patch_path: Option<String>,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub raw_code: Option<String>,
    pub database: Option<String>,
    pub schema: String,
    pub materialized: Option<String>,
    pub contract_enforced: Option<bool>,
    pub access: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
    pub primary_key: Option<Vec<String>>,
    pub version: Option<String>,
    pub latest_version: Option<String>,
    pub deprecation_date: Option<String>,
    pub time_spine: Option<String>,
    // source-only
    pub source_name: Option<String>,
    pub loader: Option<String>,
    pub freshness: Option<serde_json::Value>,
    pub source_meta: Option<serde_json::Value>,
}

impl NodeRow {
    pub fn read(row: &IndexRow) -> Option<Self> {
        Some(Self {
            unique_id: row.non_empty_str("unique_id")?,
            name: row.str("name").unwrap_or_default(),
            resource_type: row.non_empty_str("resource_type")?,
            package_name: row.str("package_name").unwrap_or_default(),
            path: row.str("file_path").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            patch_path: row.non_empty_str("patch_path"),
            fqn: row.list("fqn").unwrap_or_default(),
            alias: row.str("alias").unwrap_or_default(),
            checksum: row.non_empty_str("checksum"),
            description: row.non_empty_str("description"),
            language: row.non_empty_str("node_language"),
            raw_code: row.non_empty_str("raw_code"),
            database: row.non_empty_str("database_name"),
            schema: row.str("schema_name").unwrap_or_default(),
            materialized: row.non_empty_str("materialized"),
            contract_enforced: row.bool("contract_enforced"),
            access: row.non_empty_str("access_level"),
            tags: row.list("tags"),
            meta: row.json("meta"),
            config: row.json("config"),
            primary_key: row.list("primary_key"),
            version: row.non_empty_str("version"),
            latest_version: row.non_empty_str("latest_version"),
            deprecation_date: row.non_empty_str("deprecation_date"),
            time_spine: row.non_empty_str("time_spine"),
            source_name: row.non_empty_str("source_name"),
            loader: row.non_empty_str("loader"),
            freshness: row.json("freshness"),
            source_meta: row.json("source_meta"),
        })
    }

    pub fn is_source(&self) -> bool {
        self.resource_type == "source"
    }

    pub fn fill_gaps(&mut self, raw_code: Option<&String>, loader: Option<&String>) {
        if self.raw_code.is_none() {
            self.raw_code = raw_code.cloned();
        }
        if self.loader.is_none() {
            self.loader = loader.cloned();
        }
    }

    /// dbt v2 moves `meta` and `tags` under `config`; the top-level columns are
    /// left null.
    fn config_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.config
            .as_ref()?
            .get(key)
            .filter(|v| !v.is_null() && !is_empty_collection(v))
    }

    /// Defaults to `{}`: the JSON manifest emits `meta: {}` rather than omitting
    /// it, and rules distinguish "no metadata at all" from "metadata without the key".
    fn effective_meta(&self) -> serde_json::Value {
        self.meta
            .clone()
            .filter(|m| !is_empty_collection(m))
            .or_else(|| self.config_value("meta").cloned())
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()))
    }

    fn effective_loader(&self) -> Option<String> {
        self.loader.clone().or_else(|| {
            self.config_value("loader")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
    }

    fn effective_freshness(&self) -> Option<serde_json::Value> {
        self.freshness
            .clone()
            .or_else(|| self.config_value("freshness").cloned())
    }

    /// Defaults to `[]`, for the same reason as `effective_meta`.
    fn effective_tags(&self) -> Vec<String> {
        if let Some(tags) = self.tags.clone().filter(|t| !t.is_empty()) {
            return tags;
        }
        self.config_value("tags")
            .and_then(serde_json::Value::as_array)
            .map(|tags| {
                tags.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn node_config(&self) -> Option<NodeConfig> {
        let contract = self.contract_enforced.map(|enforced| Contract {
            enforced,
            alias_types: true,
        });
        let materialized = self.materialized.as_deref().map(Materialization::from);
        if contract.is_none() && materialized.is_none() {
            return None;
        }
        Some(NodeConfig {
            contract,
            materialized,
        })
    }

    fn base(&self, columns: Option<HashMap<String, Column>>, depends_on: DependsOn) -> NodeBase {
        NodeBase {
            database: self.database.clone(),
            schema: self.schema.clone(),
            name: self.name.clone(),
            package_name: self.package_name.clone(),
            path: self.path.clone(),
            original_file_path: self.original_file_path.clone(),
            patch_path: self.patch_path.clone(),
            unique_id: self.unique_id.clone(),
            fqn: self.fqn.clone(),
            alias: self.alias.clone(),
            checksum: FileHash {
                name: "sha256".to_string(),
                checksum: self.checksum.clone().unwrap_or_default(),
            },
            tags: Some(self.effective_tags()),
            description: self.description.clone(),
            meta: Some(Meta(self.effective_meta())),
            columns,
            config: self.node_config(),
            depends_on,
            raw_code: self.raw_code.clone(),
        }
    }

    fn compiled(&self) -> CompiledNodeFields {
        CompiledNodeFields {
            language: self.language.clone(),
        }
    }

    /// `None` for a resource type with no `Node` counterpart.
    pub fn into_node(
        self,
        columns: Option<HashMap<String, Column>>,
        depends_on: DependsOn,
        test_metadata: Option<TestRowMetadata>,
    ) -> Option<Node> {
        let base = self.base(columns, depends_on);
        let compiled = self.compiled();

        Some(match self.resource_type.as_str() {
            "model" => Node::Model(Model {
                base,
                compiled,
                access: self.access,
                constraints: None,
                version: self.version.map(serde_json::Value::String),
                latest_version: self.latest_version.map(serde_json::Value::String),
                deprecation_date: self.deprecation_date,
                defer_relation: None,
                primary_key: self.primary_key,
                time_spine: self.time_spine.map(serde_json::Value::String),
            }),
            "seed" => Node::Seed(Seed { base }),
            "snapshot" => Node::Snapshot(Snapshot { base, compiled }),
            "analysis" => Node::Analysis(Analysis { base, compiled }),
            "operation" => Node::HookNode(HookNode { base, index: None }),
            "sql_operation" => Node::SqlOperation(SqlOperation { base, compiled }),
            "test" => {
                let meta = test_metadata.unwrap_or_default();
                Node::Test(Test {
                    base,
                    compiled,
                    column_name: meta.column_name,
                    file_key_name: None,
                    attached_node: meta.attached_node,
                    test_metadata: meta.name.map(|name| TestMetadata {
                        name,
                        kwargs: meta.kwargs,
                        namespace: meta.namespace,
                    }),
                })
            }
            _ => return None,
        })
    }

    pub fn into_source(self, columns: Option<HashMap<String, Column>>) -> Source {
        Source {
            database: self.database.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            package_name: self.package_name.clone(),
            original_file_path: self.original_file_path.clone(),
            patch_path: self.patch_path.clone(),
            unique_id: self.unique_id.clone(),
            columns,
            meta: Some(Meta(
                self.source_meta
                    .clone()
                    .filter(|m| !is_empty_collection(m))
                    .unwrap_or_else(|| self.effective_meta()),
            )),
            tags: Some(self.effective_tags()),
            loader: self.effective_loader(),
            freshness: self
                .effective_freshness()
                .as_ref()
                .and_then(parse_freshness),
        }
    }
}

/// `dbt.test_metadata`, keyed by test `unique_id`.
#[derive(Default)]
pub struct TestRowMetadata {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub kwargs: Option<serde_json::Value>,
    pub column_name: Option<String>,
    pub attached_node: Option<String>,
}

impl TestRowMetadata {
    pub fn read(row: &IndexRow) -> Option<(String, Self)> {
        let unique_id = row.non_empty_str("unique_id")?;
        Some((
            unique_id,
            Self {
                name: row.non_empty_str("test_name"),
                namespace: row.non_empty_str("test_namespace"),
                kwargs: row.json("kwargs"),
                column_name: row.non_empty_str("column_name"),
                attached_node: row.non_empty_str("attached_node"),
            },
        ))
    }
}

fn is_empty_collection(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(o) => o.is_empty(),
        serde_json::Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

fn parse_freshness(value: &serde_json::Value) -> Option<SourceFreshness> {
    let threshold = |key: &str| -> Option<FreshnessThreshold> {
        let obj = value.get(key)?;
        Some(FreshnessThreshold {
            count: obj.get("count").and_then(serde_json::Value::as_u64),
            period: obj
                .get("period")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string),
        })
    };
    let freshness = SourceFreshness {
        warn_after: threshold("warn_after"),
        error_after: threshold("error_after"),
    };
    if freshness.warn_after.is_none() && freshness.error_after.is_none() {
        return None;
    }
    Some(freshness)
}

#[cfg(test)]
mod tests {
    use super::NodeRow;
    use crate::manifest::nodes::DependsOn;
    use crate::parquet::test_writer::{with_rows, Cell, ColumnKind};
    use ColumnKind::Utf8;

    const SOURCE_CONFIG: &str = r#"{"enabled":true,"loader":"fivetran","loaded_at_field":"_synced","freshness":{"warn_after":{"count":12,"period":"hour"},"error_after":{"count":24,"period":"hour"}},"meta":{"owner":"data"},"tags":["daily"]}"#;

    fn source_columns() -> &'static [(&'static str, ColumnKind)] {
        &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
            ("loader", Utf8),
            ("freshness", Utf8),
            ("meta", Utf8),
            ("tags", Utf8),
            ("config", Utf8),
        ]
    }

    /// dbt v2 leaves the dedicated source columns NULL and puts everything in config.
    #[test]
    fn a_source_reads_freshness_and_loader_from_config() {
        let row = &[
            Cell::Str("source.my_project.raw.orders"),
            Cell::Str("orders"),
            Cell::Str("source"),
            Cell::Str("my_project"),
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Str(SOURCE_CONFIG),
        ];

        with_rows(source_columns(), &[row], |r| {
            let node_row = NodeRow::read(r).expect("source row should map");
            assert!(node_row.is_source());

            let source = node_row.into_source(None);
            assert_eq!(source.loader.as_deref(), Some("fivetran"));

            let freshness = source.freshness.expect("freshness comes from config");
            assert!(freshness.is_configured());
            assert_eq!(freshness.warn_after.unwrap().count, Some(12));
            assert_eq!(
                freshness.error_after.unwrap().period.as_deref(),
                Some("hour")
            );

            assert_eq!(
                source.tags.as_deref(),
                Some(["daily".to_string()].as_slice())
            );
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn a_source_without_freshness_configured_has_none() {
        let row = &[
            Cell::Str("source.my_project.raw.orders"),
            Cell::Str("orders"),
            Cell::Str("source"),
            Cell::Str("my_project"),
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Str(r#"{"enabled":true}"#),
        ];

        with_rows(source_columns(), &[row], |r| {
            let source = NodeRow::read(r).unwrap().into_source(None);
            assert!(source.freshness.is_none());
            assert!(source.loader.is_none());
            Ok(())
        })
        .unwrap();
    }

    /// dbt writes rows for resource types dbtective has no node for.
    #[test]
    fn an_unsupported_resource_type_is_skipped() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
        ];
        let row = &[
            Cell::Str("doc.my_project.overview"),
            Cell::Str("overview"),
            Cell::Str("doc"),
            Cell::Str("my_project"),
        ];

        with_rows(columns, &[row], |r| {
            let node_row = NodeRow::read(r).expect("row should read");
            assert!(node_row
                .into_node(None, DependsOn::default(), None)
                .is_none());
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn a_seed_and_a_snapshot_map_to_their_node_types() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("resource_type", Utf8),
            ("package_name", Utf8),
        ];
        let seed = &[
            Cell::Str("seed.my_project.raw_orders"),
            Cell::Str("raw_orders"),
            Cell::Str("seed"),
            Cell::Str("my_project"),
        ];
        let snapshot = &[
            Cell::Str("snapshot.my_project.snapshot_orders"),
            Cell::Str("snapshot_orders"),
            Cell::Str("snapshot"),
            Cell::Str("my_project"),
        ];

        let mut kinds = Vec::new();
        with_rows(columns, &[seed, snapshot], |r| {
            let node = NodeRow::read(r)
                .unwrap()
                .into_node(None, DependsOn::default(), None)
                .expect("both map to nodes");
            kinds.push(node.as_str().to_string());
            Ok(())
        })
        .unwrap();

        assert_eq!(kinds, vec!["Seed", "Snapshot"]);
    }
}
