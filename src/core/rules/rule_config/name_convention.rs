pub trait NameAble {
    fn name(&self) -> &str;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

use crate::{
    cli::table::RuleResult,
    core::config::{manifest_rule::ManifestRule, naming_convention::NamingConvention},
};

/// Check if the item's name follows the specified naming convention pattern
pub fn check_name_convention<T: NameAble>(
    item: &T,
    rule: &ManifestRule,
    convention: &NamingConvention,
) -> Option<RuleResult> {
    if convention.is_match(item.name()) {
        None
    } else {
        Some(RuleResult::new(
            &rule.severity,
            NameAble::get_object_type(item),
            rule.get_name(),
            format!(
                "{} does not follow the {} naming convention.",
                NameAble::get_object_string(item),
                convention.name()
            ),
            item.get_relative_path().cloned(),
        ))
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
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::from_pattern("snake_case").unwrap(),
            },
        };
        let item = TestItem {
            name: "test_item".to_string(),
        };
        assert_eq!(check_name_convention(&item, &rule, &convention), None);
        let item_invalid = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, &convention),
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
        let convention = NamingConvention::from_pattern("PascalCase").unwrap();
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::from_pattern("PascalCase").unwrap(),
            },
        };
        let item = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(check_name_convention(&item, &rule, &convention), None);
        let item_invalid = TestItem {
            name: "test_item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, &convention),
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
        let convention = NamingConvention::from_pattern("kebab-case").unwrap();
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::from_pattern("kebab-case").unwrap(),
            },
        };
        let item = TestItem {
            name: "test-item".to_string(),
        };
        assert_eq!(check_name_convention(&item, &rule, &convention), None);
        let item_invalid = TestItem {
            name: "TestItem".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, &convention),
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
        let convention = NamingConvention::from_pattern("camelCase").unwrap();
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::from_pattern("camelCase").unwrap(),
            },
        };
        let item = TestItem {
            name: "testItem".to_string(),
        };
        assert_eq!(check_name_convention(&item, &rule, &convention), None);
        let item_invalid = TestItem {
            name: "Test_Item".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, &convention),
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
        let convention = NamingConvention::from_pattern(r"^[A-Z]{3}-[0-9]{4}$").unwrap();
        let rule = ManifestRule {
            name: Some("name_convention".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::from_pattern(r"^[A-Z]{3}-[0-9]{4}$").unwrap(),
            },
        };
        let item = TestItem {
            name: "ABC-1234".to_string(),
        };
        assert_eq!(check_name_convention(&item, &rule, &convention), None);
        let item_invalid = TestItem {
            name: "AB-123".to_string(),
        };
        assert_eq!(
            check_name_convention(&item_invalid, &rule, &convention),
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
    fn bad_regex_caught_at_parse() {
        // Invalid regex patterns are now caught during NamingConvention::from_pattern
        let result = NamingConvention::from_pattern("(*invalid_regex");
        assert!(result.is_err());
    }
}
