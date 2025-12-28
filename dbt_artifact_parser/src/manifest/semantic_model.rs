use serde::Deserialize;

use super::dbt_objects::Meta;

#[derive(Debug, Deserialize)]
pub struct SemanticModelDependsOn {
    #[allow(dead_code)]
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SemanticModel {
    pub name: String,
    pub package_name: String,
    pub original_file_path: String,
    pub description: Option<String>,
    pub metadata: Option<Meta>,
    pub depends_on: SemanticModelDependsOn,
}

impl SemanticModel {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "SemanticModel"
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.original_file_path
    }
}
