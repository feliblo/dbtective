use regex::Regex;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize, Serializer};
use strum_macros::{AsRefStr, EnumString};

use super::Materialization;

// PropertyFileColocation
#[derive(EnumString, Debug, Clone, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColocationMode {
    #[default]
    SameDirectory,
    RelativeSubdirectory,
}

// HasTags
#[derive(EnumString, Debug, Clone, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HasTagsCriteria {
    #[default]
    All,
    Any,
    OneOf,
}

// IsNotOrphaned
#[derive(EnumString, Debug, Clone, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize, serde::Serialize)]
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

// NodeDependency
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParentNodeType {
    Model,
    Source,
    Seed,
    Snapshot,
}

impl ParentNodeType {
    pub fn matches(&self, type_str: &str) -> bool {
        match self {
            Self::Model => type_str == "model",
            Self::Source => type_str == "source",
            Self::Seed => type_str == "seed",
            Self::Snapshot => type_str == "snapshot",
        }
    }
}

// HasContractEnforced
#[derive(EnumString, Debug, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    Public,
    Protected,
    Private,
}

pub fn default_allowed_test_names() -> Vec<String> {
    vec![
        "dbt_expectations.expect_compound_columns_to_be_unique".to_string(),
        "dbt_utils.unique_combination_of_columns".to_string(),
        "unique".to_string(),
    ]
}

// HasRequiredTests
/// A single required-test entry. Can be deserialized from either:
/// - a plain string: `"not_null"` (matches exactly that test name)
/// - an object with alternatives: `{ name: "uniqueness", allowed_names: ["unique", "dbt_utils.unique_combination_of_columns"] }`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequiredTest {
    WithAlternatives {
        name: String,
        allowed_names: Vec<String>,
    },
    Simple(String),
}

impl RequiredTest {
    /// The display name used in violation messages.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Simple(name) | Self::WithAlternatives { name, .. } => name,
        }
    }

    /// The list of test metadata names that satisfy this requirement.
    pub fn allowed_names(&self) -> Vec<&str> {
        match self {
            Self::Simple(name) => vec![name.as_str()],
            Self::WithAlternatives { allowed_names, .. } => {
                allowed_names.iter().map(String::as_str).collect()
            }
        }
    }
}

pub const fn default_code_max_lines() -> usize {
    150
}

pub const fn default_code_max_joins() -> usize {
    5
}

// Dependency count rules (max_upstream_dependencies / max_downstream_dependencies)
#[derive(EnumString, Debug, Clone, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Models,
    Seeds,
    Sources,
    Snapshots,
    Tests,
    Macros,
    Exposures,
}

impl DependencyType {
    pub fn matches(&self, resource_type: &str) -> bool {
        match self {
            Self::Models => resource_type == "model",
            Self::Seeds => resource_type == "seed",
            Self::Sources => resource_type == "source",
            Self::Snapshots => resource_type == "snapshot",
            Self::Tests => resource_type == "test",
            Self::Macros => resource_type == "macro",
            Self::Exposures => resource_type == "exposure",
        }
    }
}

pub fn default_excluded_dependency_types() -> Vec<DependencyType> {
    vec![DependencyType::Tests]
}

pub const fn default_max_upstream() -> usize {
    5
}

pub const fn default_max_downstream() -> usize {
    5
}

// MaxMaterializationLineage
pub fn default_included_materializations() -> Vec<Materialization> {
    vec![Materialization::View, Materialization::Ephemeral]
}

pub const fn default_max_materialization_lineage() -> usize {
    4
}

// ExposureParentsMaterialized
pub fn default_allowed_materializations() -> Vec<Materialization> {
    vec![
        Materialization::Table,
        Materialization::Incremental,
        Materialization::MaterializedView,
    ]
}

/// `ColumnNamePattern` for `columns_canonical_name` rule
/// Parse regex if the string looks like a regex pattern
/// Otherwise, treat it as a literal string
#[derive(Debug, Clone)]
pub enum ColumnNamePattern {
    Literal(String),
    Regex(Regex),
}

impl ColumnNamePattern {
    pub fn matches(&self, column_name: &str) -> bool {
        match self {
            Self::Literal(s) => s.eq_ignore_ascii_case(column_name),
            Self::Regex(r) => r.is_match(column_name),
        }
    }
}

impl<'de> Deserialize<'de> for ColumnNamePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('^') || s.ends_with('$') || s.contains(".*") || s.contains(".+") {
            let regex = Regex::new(&s).map_err(de::Error::custom)?;
            Ok(Self::Regex(regex))
        } else {
            Ok(Self::Literal(s))
        }
    }
}

impl Serialize for ColumnNamePattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Literal(s) => serializer.serialize_str(s),
            Self::Regex(r) => serializer.serialize_str(r.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ParentNodeType tests
    #[test]
    fn test_parent_node_type_matches() {
        assert!(ParentNodeType::Model.matches("model"));
        assert!(!ParentNodeType::Model.matches("source"));
        assert!(ParentNodeType::Source.matches("source"));
        assert!(!ParentNodeType::Source.matches("model"));
        assert!(ParentNodeType::Seed.matches("seed"));
        assert!(!ParentNodeType::Seed.matches("model"));
        assert!(ParentNodeType::Snapshot.matches("snapshot"));
        assert!(!ParentNodeType::Snapshot.matches("model"));
    }

    // OrphanedReferenceType tests
    #[test]
    fn test_orphaned_reference_type_matches() {
        assert!(OrphanedReferenceType::Models.matches("model"));
        assert!(!OrphanedReferenceType::Models.matches("snapshot"));
        assert!(OrphanedReferenceType::Snapshots.matches("snapshot"));
        assert!(OrphanedReferenceType::Exposures.matches("exposure"));
        assert!(OrphanedReferenceType::UnitTests.matches("unit_test"));
        assert!(!OrphanedReferenceType::UnitTests.matches("model"));
    }

    #[test]
    fn test_default_allowed_references() {
        let refs = default_allowed_references();
        assert_eq!(refs.len(), 1);
        assert!(refs[0].matches("model"));
    }

    // DependencyType tests
    #[test]
    fn test_dependency_type_matches() {
        assert!(DependencyType::Models.matches("model"));
        assert!(DependencyType::Seeds.matches("seed"));
        assert!(DependencyType::Sources.matches("source"));
        assert!(DependencyType::Snapshots.matches("snapshot"));
        assert!(DependencyType::Tests.matches("test"));
        assert!(DependencyType::Macros.matches("macro"));
        assert!(DependencyType::Exposures.matches("exposure"));
        assert!(!DependencyType::Models.matches("source"));
    }

    #[test]
    fn test_default_excluded_dependency_types() {
        let excluded = default_excluded_dependency_types();
        assert_eq!(excluded.len(), 1);
        assert!(excluded[0].matches("test"));
    }

    // ColumnNamePattern tests
    #[test]
    fn test_literal_pattern_matches_case_insensitive() {
        let pattern = ColumnNamePattern::Literal("user_id".to_string());
        assert!(pattern.matches("user_id"));
        assert!(pattern.matches("USER_ID"));
        assert!(pattern.matches("User_Id"));
        assert!(!pattern.matches("user_name"));
    }

    #[test]
    fn test_regex_pattern_matches() {
        let pattern = ColumnNamePattern::Regex(Regex::new("^id_.*").unwrap());
        assert!(pattern.matches("id_user"));
        assert!(pattern.matches("id_order"));
        assert!(!pattern.matches("user_id"));
    }

    #[test]
    fn test_deserialize_literal() {
        let pattern: ColumnNamePattern = serde_json::from_str(r#""user_id""#).unwrap();
        assert!(matches!(pattern, ColumnNamePattern::Literal(_)));
        assert!(pattern.matches("user_id"));
    }

    #[test]
    fn test_deserialize_regex_caret() {
        let pattern: ColumnNamePattern = serde_json::from_str(r#""^id_.*""#).unwrap();
        assert!(matches!(pattern, ColumnNamePattern::Regex(_)));
        assert!(pattern.matches("id_user"));
    }

    #[test]
    fn test_deserialize_regex_dollar() {
        let pattern: ColumnNamePattern = serde_json::from_str(r#""_id$""#).unwrap();
        assert!(matches!(pattern, ColumnNamePattern::Regex(_)));
        assert!(pattern.matches("user_id"));
    }

    #[test]
    fn test_deserialize_regex_dot_star() {
        let pattern: ColumnNamePattern = serde_json::from_str(r#""user.*name""#).unwrap();
        assert!(matches!(pattern, ColumnNamePattern::Regex(_)));
        assert!(pattern.matches("user_full_name"));
    }

    #[test]
    fn test_deserialize_regex_dot_plus() {
        let pattern: ColumnNamePattern = serde_json::from_str(r#""id_.+""#).unwrap();
        assert!(matches!(pattern, ColumnNamePattern::Regex(_)));
        assert!(pattern.matches("id_user"));
        assert!(!pattern.matches("id_"));
    }

    #[test]
    fn test_serialize_literal() {
        let pattern = ColumnNamePattern::Literal("user_id".to_string());
        let json = serde_json::to_string(&pattern).unwrap();
        assert_eq!(json, r#""user_id""#);
    }

    #[test]
    fn test_serialize_regex() {
        let pattern = ColumnNamePattern::Regex(Regex::new("^id_.*").unwrap());
        let json = serde_json::to_string(&pattern).unwrap();
        assert_eq!(json, r#""^id_.*""#);
    }
}
