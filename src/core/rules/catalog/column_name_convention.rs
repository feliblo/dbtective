use std::collections::HashSet;
use std::str::FromStr;

use crate::{
    cli::table::RuleResult,
    core::{
        config::{
            catalog_rule::{CatalogRule, DataTypes},
            naming_convention::NamingConvention,
        },
        rules::common_traits::Columnable,
    },
};

/// Tries to match a database data type string to a `DataTypes` enum variant.
/// Returns None if no match is found.
fn parse_data_type(db_type: &str) -> Option<DataTypes> {
    // Convert to lowercase and remove common prefixes/suffixes and parameters
    let base_type = db_type
        .to_lowercase()
        .split('(')
        .next()
        .unwrap_or(db_type)
        .trim()
        .to_string();

    // Handle array types first (e.g., "ARRAY<INT>", "ARRAY")
    if base_type.starts_with("array") {
        return Some(DataTypes::Array);
    }

    if let Ok(data_type) = DataTypes::from_str(&base_type) {
        return Some(data_type);
    }

    // Not quite sure if this is needed, since I'm quite sure the dbt datatypes are already covered above
    // But just in case, lets keep some common aliases here (I think think they're used in the catalog.json, but just in case)

    match base_type.as_str() {
        "number" => Some(DataTypes::Decimal),

        // Common short aliases
        "int" | "int4" => Some(DataTypes::Integer),
        "bool" => Some(DataTypes::Boolean),

        // PostgreSQL specific
        "int2" => Some(DataTypes::SmallInt),
        "int8" => Some(DataTypes::BigInt),
        "float8" | "double precision" => Some(DataTypes::Double),
        "float4" => Some(DataTypes::Float),
        "bytea" => Some(DataTypes::Binary),

        // Redshift specific
        "super" => Some(DataTypes::Variant),

        // SQL Server specific
        "datetime2" => Some(DataTypes::DateTime),
        "character varying" | "nvarchar" => Some(DataTypes::Varchar),
        "character" | "nchar" => Some(DataTypes::Char),

        "struct" => Some(DataTypes::Object),

        _ => None,
    }
}

/// C => Catalog object only in this test
pub fn column_name_convention<C: Columnable>(
    catalog_object: &C,
    convention: &NamingConvention,
    data_types_filter: Option<&Vec<DataTypes>>,
    rule: &CatalogRule,
    _verbose: bool,
) -> Option<RuleResult> {
    // Get columns to check - either all columns or filtered by data type
    let columns_to_check: Vec<&String> = if let Some(data_types) = data_types_filter {
        let type_set: HashSet<_> = data_types.iter().collect();

        let columns_with_types = catalog_object.get_columns_with_types()?;

        columns_with_types
            .into_iter()
            .filter_map(|(col_name, col_type)| {
                parse_data_type(col_type)
                    .filter(|parsed_type| type_set.contains(parsed_type))
                    .map(|_| col_name)
            })
            .collect()
    } else {
        catalog_object.get_column_names()?
    };

    let invalid_columns: Vec<&String> = columns_to_check
        .into_iter()
        .filter(|col_name| !convention.is_match(col_name))
        .collect();

    if invalid_columns.is_empty() {
        return None;
    }

    let invalid_column_list = invalid_columns
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(", ");

    Some(RuleResult::new(
        &rule.severity,
        C::get_object_type(catalog_object),
        rule.get_name(),
        format!(
            "{} has columns that do not follow the {} naming convention: {}.",
            catalog_object.get_object_string(),
            convention.name(),
            invalid_column_list
        ),
        catalog_object.get_relative_path().cloned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{catalog_rule::CatalogSpecificRuleConfig, severity::Severity};

    struct TestItem {
        name: String,
        columns: Vec<String>,
    }
    impl Columnable for TestItem {
        fn get_column_names(&self) -> Option<Vec<&String>> {
            Some(self.columns.iter().collect())
        }

        fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
            None
        }

        fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
            None
        }

        fn get_object_type(&self) -> &'static str {
            "TestItem"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_column_name_convention_passes_snake_case() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::from_pattern("snake_case").unwrap(),
                data_types: None,
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["first_column".to_string(), "second_column".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, &convention, None, &rule, false),
            None
        );
    }

    #[test]
    fn test_column_name_convention_fails_snake_case() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::from_pattern("snake_case").unwrap(),
                data_types: None,
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["FirstColumn".to_string(), "second_column".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, &convention, None, &rule, false),
            Some(RuleResult::new(
                &rule.severity,
                "TestItem",
                rule.get_name(),
                "test_item has columns that do not follow the snake_case naming convention: FirstColumn.".to_string(),
                item.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_colum_name_convention_passes_all_presets() {
        let patterns = ["snake_case", "kebab-case", "camelCase", "PascalCase"];
        let test_columns = [
            vec!["first_column", "second_column"], // snake_case
            vec!["first-column", "second-column"], // kebab-case
            vec!["firstColumn", "secondColumn"],   // camelCase
            vec!["FirstColumn", "SecondColumn"],   // PascalCase
        ];

        for (i, pattern) in patterns.iter().enumerate() {
            let convention = NamingConvention::from_pattern(pattern).unwrap();
            let rule = CatalogRule {
                name: Some("columns_name_convention".to_string()),
                severity: Severity::Warning,
                applies_to: None,
                description: None,
                includes: None,
                excludes: None,
                rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::from_pattern(pattern).unwrap(),
                    data_types: None,
                },
            };
            let item = TestItem {
                name: "test_item".to_string(),
                columns: test_columns[i].iter().map(|s| (*s).to_string()).collect(),
            };
            assert_eq!(
                column_name_convention(&item, &convention, None, &rule, false),
                None,
                "Failed for pattern: {pattern}",
            );
        }
    }

    #[test]
    fn test_column_name_convention_fails_all_presets() {
        let patterns = ["snake_case", "kebab-case", "camelCase", "PascalCase"];
        let test_columns = [
            vec!["FirstColumn", "secondColumn"],   // fails snake_case
            vec!["First-Column", "second-Column"], // fails kebab-case
            vec!["first_column", "Second_column"], // fails camelCase
            vec!["firstColumn", "secondColumn"],
        ];

        for (i, pattern) in patterns.iter().enumerate() {
            let convention = NamingConvention::from_pattern(pattern).unwrap();
            let rule = CatalogRule {
                name: Some("columns_name_convention".to_string()),
                severity: Severity::Warning,
                applies_to: None,
                description: None,
                includes: None,
                excludes: None,
                rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::from_pattern(pattern).unwrap(),
                    data_types: None,
                },
            };
            let item = TestItem {
                name: "test_item".to_string(),
                columns: test_columns[i].iter().map(|s| (*s).to_string()).collect(),
            };
            assert!(
                column_name_convention(&item, &convention, None, &rule, false).is_some(),
                "Failed for pattern: {pattern}",
            );
        }
    }

    #[test]
    fn test_column_name_convention_passes_custom_regex() {
        let convention = NamingConvention::from_pattern(r"^[a-z]{3}[0-9]{2}$").unwrap();
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::from_pattern(r"^[a-z]{3}[0-9]{2}$").unwrap(),
                data_types: None,
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["abc12".to_string(), "def34".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, &convention, None, &rule, false),
            None
        );
    }

    #[test]
    fn test_column_name_convention_fails_custom_regex() {
        let convention = NamingConvention::from_pattern(r"^[a-z]{3}[0-9]{2}$").unwrap();
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::from_pattern(r"^[a-z]{3}[0-9]{2}$").unwrap(),
                data_types: None,
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["ab12".to_string(), "defg34".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, &convention, None, &rule, false),
            Some(RuleResult::new(
                &rule.severity,
                "TestItem",
                rule.get_name(),
                "test_item has columns that do not follow the ^[a-z]{3}[0-9]{2}$ naming convention: ab12, defg34.".to_string(),
                item.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_column_name_convention_invalid_regex_caught_at_parse() {
        // Invalid regex patterns are now caught during NamingConvention::from_pattern
        let result = NamingConvention::from_pattern(r"*[a-z");
        assert!(result.is_err());
    }
}
