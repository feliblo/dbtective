use serde::Deserialize;

use crate::core::{
    config::{applies_to::RuleTarget, includes_excludes::IncludeExcludable},
    manifest::dbt_objects::Meta,
    rules::rule_config::{
        has_description::Descriptable, has_metadata_keys::HasMetadata, name_convention::NameAble,
    },
};

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct MacroDependsOn {
//     pub macros: Vec<String>,
// }

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct MacroArgument {
//     pub name: String,
//     #[serde(rename = "type")]
//     pub arg_type: Option<String>,
//     pub description: Option<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Macro {
    pub name: String,
    pub package_name: String,
    // pub path: String,
    pub original_file_path: String,
    // pub unique_id: String,
    // pub macro_sql: String,
    // pub depends_on: MacroDependsOn,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    // pub docs: Option<serde_json::Value>,
    // pub patch_path: Option<String>,
    // pub arguments: Option<Vec<MacroArgument>>,
    // pub created_at: Option<f64>,
    // pub supported_languages: Option<Vec<String>>,
}

impl Macro {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    #[allow(clippy::unused_self)]
    pub const fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Macros
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

impl IncludeExcludable for Macro {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &Macro {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for Macro {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
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

impl NameAble for Macro {
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

impl HasMetadata for Macro {
    fn get_metadata(&self) -> Option<&Meta> {
        self.meta.as_ref()
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
