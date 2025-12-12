use serde::Deserialize;

use crate::core::{
    checks::rules::{
        has_description::Descriptable, has_metadata_keys::HasMetadata, has_tags::Tagable,
        name_convention::NameAble,
    },
    config::{applies_to::RuleTarget, includes_excludes::IncludeExcludable},
    manifest::dbt_objects::{Meta, Tags},
};

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct ExposureOwner {
//     pub email: Option<String>,
//     pub name: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct ExposureDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Exposure {
    pub name: String,
    // pub reExposure_type: String,
    pub package_name: String,
    // pub path: String,
    pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // #[serde(rename = "type")]
    // pub exposure_type: String,
    // pub owner: ExposureOwner,
    pub description: Option<String>,
    // pub label: Option<String>,
    // pub maturity: Option<String>,
    pub meta: Option<Meta>,
    pub tags: Option<Tags>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub url: Option<String>,
    // pub depends_on: Option<ExposureDependsOn>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub Exposures: Option<Vec<serde_json::Value>>,
    // pub metrics: Option<Vec<serde_json::Value>>,
    // pub created_at: Option<f64>,
}

impl Exposure {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    #[allow(clippy::unused_self)]
    pub const fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Exposures
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

impl IncludeExcludable for Exposure {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &Exposure {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for Exposure {
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

impl NameAble for Exposure {
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

impl Tagable for Exposure {
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

impl HasMetadata for Exposure {
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
