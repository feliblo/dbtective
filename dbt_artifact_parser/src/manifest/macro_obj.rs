use serde::Deserialize;

use super::dbt_objects::Meta;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Macro {
    pub name: String,
    pub package_name: String,
    pub original_file_path: String,
    pub macro_sql: String,
    pub description: Option<String>,
    pub meta: Option<Meta>,
}

impl Macro {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "Macro"
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.original_file_path
    }
}
