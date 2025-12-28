use std::collections::HashMap;

use super::dbt_objects::column::Column;
use super::dbt_objects::{Meta, Tags};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Source {
    // Required fields
    pub database: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub package_name: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub columns: Option<HashMap<String, Column>>,
    pub meta: Option<Meta>,
    pub tags: Option<Tags>,
}

impl Source {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "Source"
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.original_file_path
    }

    pub const fn get_unique_id(&self) -> &String {
        &self.unique_id
    }
}
