use strum_macros::{AsRefStr, EnumString};

// HasTags
#[derive(EnumString, Debug, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HasTagsCriteria {
    #[default]
    All,
    Any,
    OneOf,
}

// IsNotOrphaned
#[derive(EnumString, Debug, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// References that can be made in Orphaned rule
pub enum OrphanedReferenceType {
    Models,
    Snapshots,
    // Analyses,
    Exposures,
    UnitTests,
}

pub fn default_allowed_references() -> Vec<OrphanedReferenceType> {
    vec![OrphanedReferenceType::Models]
}

impl OrphanedReferenceType {
    pub fn matches(&self, resource_type: &str) -> bool {
        match self {
            Self::Models => resource_type == "model",
            Self::Snapshots => resource_type == "snapshot",
            Self::Exposures => resource_type == "exposure",
            Self::UnitTests => resource_type == "unit_test",
        }
    }
}

pub fn default_allowed_test_names() -> Vec<String> {
    vec![
        "dbt_expectations.expect_compound_columns_to_be_unique".to_string(),
        "dbt_utils.unique_combination_of_columns".to_string(),
        "unique".to_string(),
    ]
}

pub const fn default_max_code_lines() -> usize {
    150
}
