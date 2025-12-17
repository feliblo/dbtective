pub trait NameAble {
    fn name(&self) -> &str;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};
use regex::Regex;

/// Check if the item's name follows the specified naming convention pattern
/// # Errors
/// Returns an `anyhow::Error` if the provided pattern is an invalid regex
pub fn check_name_convention<T: NameAble>(
    item: &T,
    rule: &ManifestRule,
    pattern: &str,
) -> anyhow::Result<Option<RuleResult>> {
    // Map common naming conventions to regex patterns or use the provided pattern directly
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

    if re.is_match(item.name()) {
        Ok(None)
    } else {
        Ok(Some(RuleResult::new(
            &rule.severity,
            NameAble::get_object_type(item),
            rule.get_name(),
            format!(
                "{} does not follow the {} naming convention.",
                NameAble::get_object_string(item),
                convention
            ),
            item.get_relative_path().cloned(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::config::{
        applies_to::AppliesTo, manifest_rule::ManifestSpecificRuleConfig, severity::Severity,
    };

    use super::*;
    struct TestItem {
        name: String,
    }
    impl NameAble for TestItem {
        fn name(&self) -> &str {
            &self.name
        }

        fn get_object_type(&self) -> &'static str {
            "TestItem"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_name_convention_snake_case() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: "snake_case".to_string(),
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item, &rule, "snake_case").unwrap(),
            None
        );
        let item_invalid = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, "snake_case").unwrap(),
            Some(RuleResult::new(
                &rule.severity,
                NameAble::get_object_type(&item_invalid),
                rule.rule.as_str(),
                "TestItem does not follow the snake_case naming convention.".to_string(),
                item_invalid.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_name_convention_pascal_case() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: "PascalCase".to_string(),
            },
        };
        let item = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item, &rule, "PascalCase").unwrap(),
            None
        );
        let item_invalid = TestItem {
            name: "test_item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, "PascalCase").unwrap(),
            Some(RuleResult::new(
                &rule.severity,
                NameAble::get_object_type(&item_invalid),
                rule.rule.as_str(),
                "test_item does not follow the PascalCase naming convention.".to_string(),
                item_invalid.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_name_convention_kebab_case() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: "kebab-case".to_string(),
            },
        };
        let item = TestItem {
            name: "test-item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item, &rule, "kebab-case").unwrap(),
            None
        );
        let item_invalid = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, "kebab-case").unwrap(),
            Some(RuleResult::new(
                &rule.severity,
                NameAble::get_object_type(&item_invalid),
                rule.rule.as_str(),
                "TestItem does not follow the kebab-case naming convention.".to_string(),
                item_invalid.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_name_convention_camel_case() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: "camelCase".to_string(),
            },
        };
        let item = TestItem {
            name: "testItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item, &rule, "camelCase").unwrap(),
            None
        );
        let item_invalid = TestItem {
            name: "Test_Item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, "camelCase").unwrap(),
            Some(RuleResult::new(
                &rule.severity,
                NameAble::get_object_type(&item_invalid),
                rule.rule.as_str(),
                "Test_Item does not follow the camelCase naming convention.".to_string(),
                item_invalid.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn test_name_convention_custom_regex() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: r"^[A-Z]{3}-[0-9]{4}$".to_string(),
            },
        };
        let item = TestItem {
            name: "ABC-1234".to_string(),
        };
        assert_eq!(
            check_name_convention(&item, &rule, r"^[A-Z]{3}-[0-9]{4}$").unwrap(),
            None,
        );
        let item_invalid = TestItem {
            name: "AB-123".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, r"^[A-Z]{3}-[0-9]{4}$").unwrap(),
            Some(RuleResult::new(
                &rule.severity,
                NameAble::get_object_type(&item_invalid),
                rule.rule.as_str(),
                "AB-123 does not follow the ^[A-Z]{3}-[0-9]{4}$ naming convention.".to_string(),
                item_invalid.get_relative_path().cloned(),
            ))
        );
    }

    #[test]
    fn bad_regex_causes_anyhow_error() {
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                pattern: "(*invalid_regex".to_string(),
            },
        };
        let item = TestItem {
            name: "SomeName".to_string(),
        };
        let result = check_name_convention(&item, &rule, "(*invalid_regex");
        assert!(result.is_err());
    }
}
