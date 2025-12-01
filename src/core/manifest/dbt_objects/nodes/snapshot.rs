use super::node::{CompiledNodeFields, NodeBase};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Snapshot {
    #[serde(flatten)]
    pub base: NodeBase,
    #[serde(flatten)]
    pub compiled: CompiledNodeFields,
}
