use super::node::{CompiledNodeFields, NodeBase};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Model {
    #[serde(flatten)]
    pub base: NodeBase,
    #[serde(flatten)]
    pub compiled: CompiledNodeFields,

    // Model-specific fields
    pub access: Option<String>,
    pub constraints: Option<Vec<serde_json::Value>>,
    pub version: Option<serde_json::Value>,
    pub latest_version: Option<serde_json::Value>,
    pub deprecation_date: Option<String>,
    pub defer_relation: Option<serde_json::Value>,
    pub primary_key: Option<Vec<String>>,
    pub time_spine: Option<serde_json::Value>,
}
