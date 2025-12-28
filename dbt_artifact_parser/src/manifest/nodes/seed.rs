use super::node::NodeBase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Seed {
    #[serde(flatten)]
    pub base: NodeBase,
}
