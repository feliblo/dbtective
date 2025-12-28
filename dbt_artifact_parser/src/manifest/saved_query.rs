use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SavedQueryDependsOn {
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SavedQuery {
    pub name: String,
}
