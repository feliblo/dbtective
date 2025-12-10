use serde::Deserialize;

use crate::core::{
    checks::rules::{has_description::Descriptable, name_convention::NameAble},
    config::{applies_to::RuleTarget, includes_excludes::IncludeExcludable},
};

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct SemanticModelDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SemanticModel {
    pub name: String,
    // pub resource_type: String,
    // pub path: String,
    pub package_name: String,
    pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub model: String,
    // pub node_relation: serde_json::Value,
    pub description: Option<String>,
    // pub label: Option<String>,
    // pub defaults: Option<serde_json::Value>,
    // pub entities: Option<Vec<serde_json::Value>>,
    // pub measures: Option<Vec<serde_json::Value>>,
    // pub dimensions: Option<Vec<serde_json::Value>>,
    // pub metadata: Option<serde_json::Value>,
    // pub depends_on: Option<SemanticModelDependsOn>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub created_at: Option<f64>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub primary_entity: Option<String>,
    // pub group: Option<String>,
}

impl SemanticModel {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    #[allow(clippy::unused_self)]
    pub const fn ruletarget(&self) -> RuleTarget {
        RuleTarget::SemanticModels
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

impl IncludeExcludable for SemanticModel {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &SemanticModel {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for SemanticModel {
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

impl NameAble for SemanticModel {
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
