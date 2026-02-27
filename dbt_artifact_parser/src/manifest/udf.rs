use super::dbt_objects::column::Column;
use super::dbt_objects::{Meta, NodeDocs, Tags};
use super::nodes::node::{Contract, DependsOn, FileHash};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// resource_type is always "function"
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UDF {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub columns: Option<HashMap<String, Column>>,
    pub meta: Meta,
    pub group: Option<String>,
    pub returns: FunctionReturns,
    pub database: Option<String>,
    pub schema: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: FileHash,
    pub config: UDFConfig,
    #[serde(default)]
    pub tags: Option<Tags>,
    pub docs: Option<NodeDocs>,
    pub patch_path: Option<String>,
    pub build_path: Option<String>,
    pub unrendered_config: UnrenderedConfig,
    pub created_at: f64,
    pub config_call_dict: Option<Value>,
    pub unrendered_config_call_dict: Option<Value>,
    pub relation_name: Option<String>,
    #[serde(default)]
    pub raw_code: String,
    #[serde(default)]
    pub doc_blocks: Vec<String>,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub refs: Vec<RefArg>,
    #[serde(default)]
    pub sources: Vec<Vec<String>>,
    #[serde(default)]
    pub metrics: Vec<Vec<String>>,
    #[serde(default)]
    pub functions: Vec<Vec<String>>,
    #[serde(default)]
    pub depends_on: DependsOn,
    pub compiled_path: Option<String>,
    #[serde(default)]
    pub compiled: Option<bool>,
    pub compiled_code: Option<String>,
    #[serde(default)]
    pub extra_ctes_injected: Option<bool>,
    #[serde(default)]
    pub extra_ctes: Vec<InjectedCTE>,
    pub pre_injected_sql: Option<String>,
    pub contract: Option<Contract>,
    #[serde(default)]
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FunctionReturns {
    pub data_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct InjectedCTE {
    pub id: String,
    pub sql: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RefArg {
    pub name: String,
    pub package: Option<String>,
    pub version: Option<RefArgsVersion>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FunctionArgument {
    pub name: String,
    pub data_type: String,
    pub description: Option<String>,
    pub default_value: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UDFConfig {
    pub enabled: bool,
    pub alias: Option<String>,
    pub schema: Option<String>,
    pub database: Option<String>,
    #[serde(default)]
    pub tags: Option<Tags>,
    pub meta: Option<Meta>,
    pub group: Option<String>,
    #[serde(default)]
    pub materialized: String,
    pub incremental_strategy: Option<String>,
    pub batch_size: Option<Value>,
    pub lookback: Option<Value>,
    pub begin: Option<Value>,
    pub persist_docs: Option<Value>,
    #[serde(rename = "post-hook", default)]
    pub post_hook: Vec<Hook>,
    #[serde(rename = "pre-hook", default)]
    pub pre_hook: Vec<Hook>,
    pub quoting: Option<Value>,
    pub column_types: Option<Value>,
    pub full_refresh: Option<bool>,
    pub unique_key: Option<UniqueKey>,
    #[serde(default)]
    pub on_schema_change: String,
    pub on_configuration_change: Option<OnConfigurationChange>,
    pub grants: Option<Value>,
    #[serde(default)]
    pub packages: Vec<String>,
    pub docs: Option<NodeDocs>,
    pub contract: Option<Contract>,
    pub event_time: Option<Value>,
    pub concurrent_batches: Option<Value>,
    // UDF-specific config fields
    #[serde(rename = "type")]
    pub udf_type: Option<UDFType>,
    pub volatility: Option<Volatility>,
    pub runtime_version: Option<String>,
    pub entry_point: Option<String>,
    // Catch any other fields we don't explicitly model
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnConfigurationChange {
    Apply,
    Continue,
    Fail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Volatility {
    Deterministic,
    Stable,
    NonDeterministic,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UDFType {
    Scalar,
    Aggregate,
    Table,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UniqueKey {
    Single(String),
    Multiple(Vec<String>),
    Null,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RefArgsVersion {
    VersionString(String),
    VersionInt(i64),
}

#[derive(Debug, Deserialize)]
pub struct UnrenderedConfig(pub Value);

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Hook {
    pub sql: String,
    pub transaction: bool,
    pub index: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_function_returns() {
        let json = r#"{"data_type": "integer", "description": null}"#;
        let returns: FunctionReturns = serde_json::from_str(json).unwrap();
        assert_eq!(returns.data_type, "integer");
        assert!(returns.description.is_none());
    }

    #[test]
    fn test_deserialize_function_returns_with_description() {
        let json = r#"{"data_type": "boolean", "description": "Returns true if positive"}"#;
        let returns: FunctionReturns = serde_json::from_str(json).unwrap();
        assert_eq!(returns.data_type, "boolean");
        assert_eq!(
            returns.description.as_deref(),
            Some("Returns true if positive")
        );
    }

    #[test]
    fn test_deserialize_function_argument() {
        let json = r#"{
            "name": "a_string",
            "data_type": "text",
            "description": "The input string",
            "default_value": "'1'"
        }"#;
        let arg: FunctionArgument = serde_json::from_str(json).unwrap();
        assert_eq!(arg.name, "a_string");
        assert_eq!(arg.data_type, "text");
        assert_eq!(arg.description.as_deref(), Some("The input string"));
        assert!(arg.default_value.is_some());
    }

    #[test]
    fn test_deserialize_function_argument_minimal() {
        let json = r#"{"name": "x", "data_type": "int"}"#;
        let arg: FunctionArgument = serde_json::from_str(json).unwrap();
        assert_eq!(arg.name, "x");
        assert_eq!(arg.data_type, "int");
        assert!(arg.description.is_none());
        assert!(arg.default_value.is_none());
    }

    #[test]
    fn test_deserialize_function_argument_default_value_types() {
        // String default
        let json = r#"{"name": "x", "data_type": "text", "default_value": "'hello'"}"#;
        let arg: FunctionArgument = serde_json::from_str(json).unwrap();
        assert!(arg.default_value.unwrap().is_string());

        // Integer default
        let json = r#"{"name": "x", "data_type": "int", "default_value": 42}"#;
        let arg: FunctionArgument = serde_json::from_str(json).unwrap();
        assert!(arg.default_value.unwrap().is_i64());

        // Boolean default
        let json = r#"{"name": "x", "data_type": "bool", "default_value": true}"#;
        let arg: FunctionArgument = serde_json::from_str(json).unwrap();
        assert!(arg.default_value.unwrap().is_boolean());
    }

    #[test]
    fn test_deserialize_udf_type() {
        let json = r#""scalar""#;
        let t: UDFType = serde_json::from_str(json).unwrap();
        assert!(matches!(t, UDFType::Scalar));

        let json = r#""aggregate""#;
        let t: UDFType = serde_json::from_str(json).unwrap();
        assert!(matches!(t, UDFType::Aggregate));

        let json = r#""table""#;
        let t: UDFType = serde_json::from_str(json).unwrap();
        assert!(matches!(t, UDFType::Table));
    }

    #[test]
    fn test_deserialize_volatility() {
        let json = r#""deterministic""#;
        let v: Volatility = serde_json::from_str(json).unwrap();
        assert!(matches!(v, Volatility::Deterministic));

        let json = r#""stable""#;
        let v: Volatility = serde_json::from_str(json).unwrap();
        assert!(matches!(v, Volatility::Stable));

        let json = r#""non-deterministic""#;
        let v: Volatility = serde_json::from_str(json).unwrap();
        assert!(matches!(v, Volatility::NonDeterministic));
    }

    #[test]
    fn test_deserialize_on_configuration_change() {
        let json = r#""apply""#;
        let v: OnConfigurationChange = serde_json::from_str(json).unwrap();
        assert!(matches!(v, OnConfigurationChange::Apply));

        let json = r#""continue""#;
        let v: OnConfigurationChange = serde_json::from_str(json).unwrap();
        assert!(matches!(v, OnConfigurationChange::Continue));

        let json = r#""fail""#;
        let v: OnConfigurationChange = serde_json::from_str(json).unwrap();
        assert!(matches!(v, OnConfigurationChange::Fail));
    }

    #[test]
    fn test_deserialize_full_udf() {
        let json = r#"{
            "returns": {"data_type": "integer", "description": null},
            "database": "dbt",
            "schema": "public_udf_schema",
            "name": "is_positive_int",
            "resource_type": "function",
            "package_name": "test_project",
            "path": "is_positive_int.sql",
            "original_file_path": "functions/is_positive_int.sql",
            "unique_id": "function.test_project.is_positive_int",
            "fqn": ["test_project", "is_positive_int"],
            "alias": "is_positive_int",
            "checksum": {"name": "sha256", "checksum": "abc123"},
            "config": {
                "enabled": true,
                "alias": null,
                "schema": "udf_schema",
                "database": "dbt",
                "tags": [],
                "meta": {},
                "group": null,
                "materialized": "function",
                "incremental_strategy": null,
                "batch_size": null,
                "lookback": 1,
                "begin": null,
                "persist_docs": {},
                "post-hook": [],
                "pre-hook": [],
                "quoting": {},
                "column_types": {},
                "full_refresh": null,
                "unique_key": null,
                "on_schema_change": "ignore",
                "on_configuration_change": "apply",
                "grants": {},
                "packages": [],
                "docs": {"show": true, "node_color": null},
                "contract": {"enforced": false, "alias_types": true},
                "event_time": null,
                "concurrent_batches": null,
                "type": "scalar",
                "volatility": "deterministic",
                "runtime_version": null,
                "entry_point": null
            },
            "tags": [],
            "description": "A test UDF",
            "columns": {},
            "meta": {},
            "group": null,
            "docs": {"show": true, "node_color": null},
            "patch_path": "test_project://functions/schema.yml",
            "build_path": null,
            "unrendered_config": {"schema": "udf_schema"},
            "created_at": 1772188537.320818,
            "relation_name": null,
            "raw_code": "SELECT 1",
            "doc_blocks": [],
            "language": "sql",
            "refs": [],
            "sources": [],
            "metrics": [],
            "functions": [],
            "depends_on": {"macros": [], "nodes": []},
            "compiled_path": null,
            "contract": {"enforced": false, "alias_types": true, "checksum": null},
            "arguments": [
                {
                    "name": "x",
                    "data_type": "text",
                    "description": "input",
                    "default_value": "'1'"
                }
            ]
        }"#;

        let udf: UDF = serde_json::from_str(json).unwrap();
        assert_eq!(udf.name, "is_positive_int");
        assert_eq!(udf.package_name, "test_project");
        assert_eq!(udf.returns.data_type, "integer");
        assert_eq!(udf.language, "sql");
        assert_eq!(udf.raw_code, "SELECT 1");
        assert_eq!(udf.arguments.len(), 1);
        assert_eq!(udf.arguments[0].name, "x");
        assert!(matches!(udf.config.udf_type, Some(UDFType::Scalar)));
        assert!(matches!(
            udf.config.volatility,
            Some(Volatility::Deterministic)
        ));
        assert!(udf.config.full_refresh.is_none());
        assert!(udf.config.enabled);
    }

    #[test]
    fn test_deserialize_udf_without_optional_fields() {
        let json = r#"{
            "returns": {"data_type": "int"},
            "database": null,
            "schema": "main",
            "name": "my_func",
            "resource_type": "function",
            "package_name": "proj",
            "path": "my_func.sql",
            "original_file_path": "functions/my_func.sql",
            "unique_id": "function.proj.my_func",
            "fqn": ["proj", "my_func"],
            "alias": "my_func",
            "checksum": {"name": "sha256", "checksum": "def456"},
            "config": {
                "enabled": true,
                "alias": null,
                "schema": null,
                "database": null,
                "materialized": "function",
                "on_schema_change": "ignore",
                "post-hook": [],
                "pre-hook": [],
                "packages": []
            },
            "meta": {},
            "group": null,
            "docs": null,
            "patch_path": null,
            "build_path": null,
            "unrendered_config": {},
            "created_at": 1772188537.0,
            "relation_name": null,
            "compiled_path": null,
            "compiled_code": null,
            "contract": null,
            "_pre_injected_sql": null
        }"#;

        let udf: UDF = serde_json::from_str(json).unwrap();
        assert_eq!(udf.name, "my_func");
        assert!(udf.database.is_none());
        assert!(udf.docs.is_none());
        assert!(udf.patch_path.is_none());
        assert!(udf.contract.is_none());
        assert!(udf.arguments.is_empty());
        assert!(udf.description.is_empty());
        assert!(udf.config.udf_type.is_none());
        assert!(udf.config.volatility.is_none());
    }

    #[test]
    fn test_deserialize_hook() {
        let json = r#"{"sql": "GRANT SELECT", "transaction": true, "index": null}"#;
        let hook: Hook = serde_json::from_str(json).unwrap();
        assert_eq!(hook.sql, "GRANT SELECT");
        assert!(hook.transaction);
        assert!(hook.index.is_none());
    }

    #[test]
    fn test_deserialize_unique_key_variants() {
        let json = r#""id""#;
        let uk: UniqueKey = serde_json::from_str(json).unwrap();
        assert!(matches!(uk, UniqueKey::Single(s) if s == "id"));

        let json = r#"["id", "name"]"#;
        let uk: UniqueKey = serde_json::from_str(json).unwrap();
        assert!(matches!(uk, UniqueKey::Multiple(v) if v.len() == 2));

        let json = r"null";
        let uk: UniqueKey = serde_json::from_str(json).unwrap();
        assert!(matches!(uk, UniqueKey::Null));
    }

    #[test]
    fn test_deserialize_ref_args_version() {
        let json = r#""v1""#;
        let v: RefArgsVersion = serde_json::from_str(json).unwrap();
        assert!(matches!(v, RefArgsVersion::VersionString(s) if s == "v1"));

        let json = r"2";
        let v: RefArgsVersion = serde_json::from_str(json).unwrap();
        assert!(matches!(v, RefArgsVersion::VersionInt(2)));
    }
}
