use super::meta::Meta;
use super::tags::Tags;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ColumnLevelConstraint {
    #[serde(rename = "type")]
    type_: String,
    name: Option<String>,
    expression: Option<String>,
    warn_unenforced: Option<bool>,
    warn_unsupported: Option<bool>,
    to: Option<String>,
    to_columns: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ColumnConfig {
    _extra: Option<serde_json::Value>,
    meta: Option<Meta>,
    tags: Option<Tags>,
    #[serde(rename = "Additional Properties")]
    additional_properties: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Column {
    pub name: String,
    pub description: Option<String>,
    pub data_type: Option<String>,
    pub tests: Option<Vec<String>>,
    pub meta: Option<Meta>,
    pub datatype: Option<String>,
    pub constraints: Option<Vec<String>>,
    pub quoted: Option<bool>,
    pub config: Option<ColumnConfig>,
    pub tags: Tags,
    #[allow(clippy::pub_underscore_fields)]
    pub _extra: Option<serde_json::Value>,
    pub granularity: Option<String>,
    pub doc_blocks: Option<Vec<String>>,
    #[serde(rename = "Additional Properties")]
    pub additional_properties: Option<serde_json::Value>,
}
