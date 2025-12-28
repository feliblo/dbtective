use serde::Deserialize;

use super::dbt_objects::{Meta, Tags};

#[derive(Debug, Deserialize)]
pub struct ExposureDependsOn {
    #[allow(dead_code)]
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Exposure {
    pub name: String,
    pub package_name: String,
    pub original_file_path: String,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    pub tags: Option<Tags>,
    pub depends_on: ExposureDependsOn,
}

impl Exposure {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "Exposure"
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.original_file_path
    }
}
