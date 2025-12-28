use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UnitTest {
    pub name: String,
    pub model: String,
    pub package_name: String,
    pub original_file_path: String,
    pub description: Option<String>,
}

impl UnitTest {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "UnitTest"
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.original_file_path
    }
}
