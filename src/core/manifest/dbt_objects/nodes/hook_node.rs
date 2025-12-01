use super::node::NodeBase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HookNode {
    #[serde(flatten)]
    pub base: NodeBase,
    pub index: Option<i64>,
}
