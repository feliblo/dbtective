use super::node::{CompiledNodeFields, NodeBase};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Test {
    #[serde(flatten)]
    pub base: NodeBase,
    #[serde(flatten)]
    pub compiled: CompiledNodeFields,

    // GenericTest-specific fields (will be None for SingularTests)
    pub column_name: Option<String>,
    pub file_key_name: Option<String>,
    pub attached_node: Option<String>,
    pub test_metadata: Option<serde_json::Value>,
}
