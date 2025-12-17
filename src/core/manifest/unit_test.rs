use serde::Deserialize;

use crate::core::{
    config::{applies_to::RuleTarget, includes_excludes::IncludeExcludable},
    rules::rule_config::{has_description::Descriptable, name_convention::NameAble},
};

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct UnitTestDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UnitTest {
    pub name: String,
    pub model: String,
    // pub given: Vec<serde_json::Value>,
    // pub expect: serde_json::Value,
    // pub resource_type: String,
    pub package_name: String,
    // pub path: String,
    pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    pub description: Option<String>,
    // pub overrides: Option<serde_json::Value>,
    // pub depends_on: Option<UnitTestDependsOn>,
    // pub config: Option<serde_json::Value>,
    // pub checksum: Option<String>,
    // pub schema: Option<String>,
    // pub created_at: Option<f64>,
    // pub versions: Option<serde_json::Value>,
    // pub version: Option<serde_json::Value>,
}

impl UnitTest {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    #[allow(clippy::unused_self)]
    pub const fn ruletarget(&self) -> RuleTarget {
        RuleTarget::UnitTests
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

impl IncludeExcludable for UnitTest {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &UnitTest {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for UnitTest {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn get_object_type(&self) -> &'static str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }
}

impl NameAble for UnitTest {
    fn name(&self) -> &str {
        self.get_name()
    }

    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}
