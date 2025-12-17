use regex::Regex;

use crate::{
    cli::table::RuleResult,
    core::{config::catalog_rule::CatalogRule, rules::common_traits::Columnable},
};

/// C => Catalog object only in this test
/// # Errors
/// Returns an `anyhow::Error` if the provided pattern is an invalid regex
pub fn column_name_convention<C: Columnable>(
    catalog_object: &C,
    pattern: &str,
    rule: &CatalogRule,
    _verbose: bool,
) -> anyhow::Result<Option<RuleResult>> {
    let (regex, convention) = match pattern {
        "snake_case" | "snakecase" => (r"^[a-z][a-z0-9_]*$", "snake_case"),
        "kebab_case" | "kebabcase" | "kebab-case" => (r"^[a-z][a-z0-9-]*$", "kebab-case"),
        "camelCase" | "camel_case" | "camelcase" => (r"^[a-z][a-zA-Z0-9]*$", "camelCase"),
        "pascal_case" | "pascalcase" | "pascal-case" | "PascalCase" => {
            (r"^[A-Z][a-zA-Z0-9]*$", "PascalCase")
        }
        _ => (pattern, pattern),
    };

    let re = Regex::new(regex)
        .map_err(|e| anyhow::anyhow!("Invalid regex for '{}'. {}", rule.get_name(), e))?;

    let Some(columns) = catalog_object.get_column_names() else {
        return Ok(None);
    };

    let invalid_columns: Vec<&String> = columns
        .into_iter()
        .filter(|col_name| !re.is_match(col_name))
        .collect();

    if invalid_columns.is_empty() {
        return Ok(None);
    }

    let invalid_column_list = invalid_columns
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(", ");

    Ok(Some(RuleResult::new(
        &rule.severity,
        C::get_object_type(catalog_object),
        rule.get_name(),
        format!(
            "{} has columns that do not follow the {} naming convention: {}.",
            catalog_object.get_object_string(),
            convention,
            invalid_column_list
        ),
        catalog_object.get_relative_path().cloned(),
    )))
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

        fn get_object_type(&self) -> &'static str {
            "TestItem"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_column_name_convention_passes_snake_case() {
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                pattern: "snake_case".to_string(),
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["first_column".to_string(), "second_column".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, "snake_case", &rule, false).unwrap(),
            None
        );
    }

    #[test]
    fn test_column_name_convention_fails_snake_case() {
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                pattern: "snake_case".to_string(),
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["FirstColumn".to_string(), "second_column".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, "snake_case", &rule, false).unwrap(),
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
            let rule = CatalogRule {
                name: Some("columns_name_convention".to_string()),
                severity: Severity::Warning,
                applies_to: None,
                description: None,
                includes: None,
                excludes: None,
                rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                    pattern: (*pattern).to_string(),
                },
            };
            let item = TestItem {
                name: "test_item".to_string(),
                columns: test_columns[i].iter().map(|s| (*s).to_string()).collect(),
            };
            assert_eq!(
                column_name_convention(&item, pattern, &rule, false).unwrap(),
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
            let rule = CatalogRule {
                name: Some("columns_name_convention".to_string()),
                severity: Severity::Warning,
                applies_to: None,
                description: None,
                includes: None,
                excludes: None,
                rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                    pattern: (*pattern).to_string(),
                },
            };
            let item = TestItem {
                name: "test_item".to_string(),
                columns: test_columns[i].iter().map(|s| (*s).to_string()).collect(),
            };
            assert!(
                column_name_convention(&item, pattern, &rule, false)
                    .unwrap()
                    .is_some(),
                "Failed for pattern: {pattern}",
            );
        }
    }

    #[test]
    fn test_column_name_convention_passes_custom_regex() {
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                pattern: r"^[a-z]{3}[0-9]{2}$".to_string(), // custom pattern: 3 letters followed by 2 digits
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["abc12".to_string(), "def34".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, r"^[a-z]{3}[0-9]{2}$", &rule, false).unwrap(),
            None
        );
    }

    #[test]
    fn test_column_name_convention_fails_custom_regex() {
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                pattern: r"^[a-z]{3}[0-9]{2}$".to_string(), // custom pattern: 3 letters followed by 2 digits
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["ab12".to_string(), "defg34".to_string()],
        };
        assert_eq!(
            column_name_convention(&item, r"^[a-z]{3}[0-9]{2}$", &rule, false).unwrap(),
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
    fn test_column_name_convention_invalid_regex() {
        let rule = CatalogRule {
            name: Some("columns_name_convention".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule: CatalogSpecificRuleConfig::ColumnsNameConvention {
                pattern: r"*[a-z".to_string(), // invalid regex
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
            columns: vec!["valid_column".to_string()],
        };
        let result = column_name_convention(&item, r"*[a-z", &rule, false);
        assert!(result.is_err());
    }
}
