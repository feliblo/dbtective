use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NodeDocs {
    pub show: Option<bool>,
    pub node_color: Option<String>,
}
