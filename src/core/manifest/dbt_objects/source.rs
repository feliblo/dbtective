use std::collections::HashMap;

// Sources aren't being treated as nodes in dbt. They have their own structure.
// use super::column::Column;
// use super::{Meta, Tags};
use super::tags::Tags;
use crate::core::{
    checks::{
        common::{
            child_map::ChildMappable, has_description::Descriptable, has_tags::Tagable,
            has_unique_test::TestAble, name_convention::NameAble,
        },
        common_traits::Columnable,
    },
    config::{applies_to::RuleTarget, includes_excludes::IncludeExcludable},
    manifest::{dbt_objects::column::Column, Manifest},
};
use serde::Deserialize;
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
    pub package_name: String,
    // pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
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
    pub columns: Option<HashMap<String, Column>>,
    // pub meta: Option<Meta>,
    // pub source_meta: Option<Meta>,
    pub tags: Option<Tags>,
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

    #[allow(clippy::unused_self)]
    pub const fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Sources
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

impl IncludeExcludable for Source {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &Source {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for Source {
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

impl NameAble for Source {
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

impl Tagable for Source {
    fn get_tags(&self) -> Option<&Tags> {
        self.tags.as_ref()
    }

    fn get_object_type(&self) -> &'static str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl Columnable for Source {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.columns
            .as_ref()
            .map(|cols| cols.keys().collect::<Vec<&String>>())
    }

    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl ChildMappable for Source {
    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }

    fn get_childs<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str> {
        let unique_id = self.get_unique_id();
        manifest
            .child_map
            .get(unique_id)
            .map(|children| children.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

impl TestAble for Source {
    fn get_unique_id(&self) -> &String {
        self.get_unique_id()
    }

    fn get_object_string(&self) -> &String {
        self.get_name()
    }

    fn get_object_type(&self) -> String {
        Self::get_object_type().to_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}
