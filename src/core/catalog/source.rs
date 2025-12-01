use serde::Deserialize;
use std::collections::HashMap;

use crate::core::catalog::{
    columns::CatalogColumn, resource_metadata::CatalogResourceMetadata, stats::CatalogStat,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogSource {
    pub unique_id: String,
    pub metadata: CatalogResourceMetadata,
    pub columns: HashMap<String, CatalogColumn>,
    pub stats: HashMap<String, CatalogStat>,
}
