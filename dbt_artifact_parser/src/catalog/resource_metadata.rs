use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogResourceMetadata {
    #[serde(rename = "type")]
    pub type_: String,
    pub database: String,
    pub schema: String,
    pub name: String,
    pub comment: Option<String>,
    pub owner: Option<String>,
}
