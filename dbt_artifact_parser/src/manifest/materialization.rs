use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// <https://docs.getdbt.com/docs/build/materializations>
/// Represents dbt model materialization types.
/// Supports the 5 built-in types plus custom materializations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Materialization {
    Table,
    View,
    Incremental,
    Ephemeral,
    MaterializedView,
    /// Custom materialization (e.g., from dbt packages or user-defined)
    Custom(String),
}

impl Materialization {
    /// Returns the string representation of the materialization.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Table => "table",
            Self::View => "view",
            Self::Incremental => "incremental",
            Self::Ephemeral => "ephemeral",
            Self::MaterializedView => "materialized_view",
            Self::Custom(s) => s,
        }
    }
}

impl fmt::Display for Materialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> Deserialize<'de> for Materialization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s.as_str()))
    }
}

impl From<&str> for Materialization {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "table" => Self::Table,
            "view" => Self::View,
            "incremental" => Self::Incremental,
            "ephemeral" => Self::Ephemeral,
            "materialized_view" | "materializedview" | "materialized-view" => {
                Self::MaterializedView
            }
            _ => Self::Custom(s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_built_in_materializations() {
        assert_eq!(Materialization::from("table"), Materialization::Table);
        assert_eq!(Materialization::from("view"), Materialization::View);
        assert_eq!(
            Materialization::from("incremental"),
            Materialization::Incremental
        );
        assert_eq!(
            Materialization::from("ephemeral"),
            Materialization::Ephemeral
        );
        assert_eq!(
            Materialization::from("materialized_view"),
            Materialization::MaterializedView
        );
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(Materialization::from("TABLE"), Materialization::Table);
        assert_eq!(Materialization::from("View"), Materialization::View);
        assert_eq!(
            Materialization::from("INCREMENTAL"),
            Materialization::Incremental
        );
    }

    #[test]
    fn test_materialized_view_aliases() {
        assert_eq!(
            Materialization::from("materialized_view"),
            Materialization::MaterializedView
        );
        assert_eq!(
            Materialization::from("materializedview"),
            Materialization::MaterializedView
        );
        assert_eq!(
            Materialization::from("materialized-view"),
            Materialization::MaterializedView
        );
    }

    #[test]
    fn test_custom_materialization() {
        let custom = Materialization::from("my_custom_mat");
        assert_eq!(custom, Materialization::Custom("my_custom_mat".to_string()));
        assert_eq!(custom.as_str(), "my_custom_mat");
    }

    #[test]
    fn test_display() {
        assert_eq!(Materialization::Table.to_string(), "table");
        assert_eq!(Materialization::View.to_string(), "view");
        assert_eq!(
            Materialization::Custom("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_deserialize() {
        let json = r#""table""#;
        let mat: Materialization = serde_json::from_str(json).unwrap();
        assert_eq!(mat, Materialization::Table);

        let json_custom = r#""my_custom""#;
        let mat_custom: Materialization = serde_json::from_str(json_custom).unwrap();
        assert_eq!(mat_custom, Materialization::Custom("my_custom".to_string()));
    }
}
