use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogStat {
    pub id: String,
    pub label: String,
    pub value: serde_json::Value,
    pub include: bool,
    pub description: Option<String>,
}
