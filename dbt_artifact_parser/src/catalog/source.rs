use serde::Deserialize;
use std::collections::HashMap;

use super::columns::CatalogColumn;
use super::resource_metadata::CatalogResourceMetadata;
use super::stats::CatalogStat;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogSource {
    pub unique_id: String,
    pub metadata: CatalogResourceMetadata,
    pub columns: HashMap<String, CatalogColumn>,
    pub stats: HashMap<String, CatalogStat>,
}

impl CatalogSource {
    pub fn get_name(&self) -> &str {
        &self.metadata.name
    }

    pub fn get_unique_id(&self) -> &str {
        &self.unique_id
    }

    pub const fn get_object_type() -> &'static str {
        "Source"
    }
}
