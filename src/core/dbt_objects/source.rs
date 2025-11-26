// Sources aren't being treated as nodes in dbt. They have their own structure.
// use super::column::Column;
// use super::{Meta, Tags};
use serde::Deserialize;

use crate::core::traits::Descriptable;
// use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Source {
    // Required fields
    pub database: Option<String>,
    // pub schema: String,
    pub name: String,
    pub description: Option<String>,
    // pub resource_type: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub source_name: String,
    // pub loader: String,
    // pub identifier: String,

    // // Optional fields
    // pub quoting: Option<serde_json::Value>,
    // pub loaded_at_field: Option<String>,
    // pub loaded_at_query: Option<String>,
    // pub freshness: Option<serde_json::Value>,
    // pub external: Option<serde_json::Value>,
    // pub columns: Option<HashMap<String, Column>>,
    // pub meta: Option<Meta>,
    // pub source_meta: Option<Meta>,
    // pub tags: Option<Tags>,
    // pub config: Option<serde_json::Value>,
    // pub patch_path: Option<String>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub relation_name: Option<String>,
    // pub created_at: Option<f64>,
    // pub unrendered_database: Option<String>,
    // pub unrendered_schema: Option<String>,
    // pub doc_blocks: Option<Vec<String>>,
}

impl Source {
    pub const fn get_name(&self) -> &String {
        &self.name
    }
}

impl Descriptable for Source {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn get_object_type(&self) -> &'static str {
        "Source"
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }
}
