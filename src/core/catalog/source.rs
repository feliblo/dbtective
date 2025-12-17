use serde::Deserialize;
use std::collections::HashMap;

use crate::core::{
    catalog::{
        columns::CatalogColumn, resource_metadata::CatalogResourceMetadata, stats::CatalogStat,
    },
    rules::common_traits::Columnable,
};

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

impl Columnable for CatalogSource {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.columns.keys().collect::<Vec<&String>>().into()
    }

    // Column descriptions are not available in the catalog.
    // Find them by corresponding with the unique_id to the manifest.
    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        None
    }

    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

impl Columnable for &CatalogSource {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        (*self).get_column_names()
    }

    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        (*self).get_columns_with_descriptions()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_object_string(&self) -> &str {
        (*self).get_object_string()
    }
    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}
