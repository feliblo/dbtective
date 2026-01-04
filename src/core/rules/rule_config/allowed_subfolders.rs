use crate::cli::table::RuleResult;
use crate::core::config::applies_to::RuleTarget;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

/// Trait for objects that can be checked for allowed subfolders.
/// Extends Identifiable to provide path information.
pub trait PathCheckable: Identifiable {
    fn get_rule_target(&self) -> RuleTarget;
}

pub fn check_allowed_subfolders<T: PathCheckable>(
    item: &T,
    rule: &ManifestRule,
    allowed_subfolders: &[String],
    path_prefix: Option<&String>,
    path_postfix: Option<&String>,
) -> Option<RuleResult> {
    // Get the relative path from the object
    let Some(raw_path) = item.get_relative_path() else {
        return Some(RuleResult::new(
            &rule.severity,
            item.get_object_type(),
            rule.get_name(),
            format!(
                "{} does not have a relative path; cannot check allowed subfolders.",
                item.get_object_string()
            ),
            None,
        ));
    };

    let normalized_path = normalize_path(raw_path);
    let resource_type = item.get_rule_target().as_snake_case();
    let base_path = build_base_path(resource_type, path_prefix, path_postfix);

    // Check if the path starts with the base path
    // If it doesn't, this rule doesn't apply to this file - pass (return None)
    // e.g. /models/bronze/ doesnt apply to /models/silver/
    if !normalized_path.starts_with(&base_path) {
        return None;
    }

    // Extract the part after the base path
    let remaining_path = &normalized_path[base_path.len()..];

    // If there's no '/', the file is directly in the base path
    if !remaining_path.contains('/') || remaining_path.is_empty() {
        return Some(RuleResult::new(
            &rule.severity,
            item.get_object_type(),
            rule.get_name(),
            format!(
                "{} is directly in '{}' but should be in one of the allowed subfolders: {}",
                item.get_object_string(),
                base_path,
                allowed_subfolders.join(", ")
            ),
            item.get_relative_path().cloned(),
        ));
    }

    // Split by '/' to get the immediate subfolder
    let parts: Vec<&str> = remaining_path.split('/').collect();
    let immediate_subfolder = parts[0];

    if allowed_subfolders.contains(&immediate_subfolder.to_string()) {
        None // Pass
    } else {
        Some(RuleResult::new(
            &rule.severity,
            item.get_object_type(),
            rule.get_name(),
            format!(
                "{} is in subfolder '{}' these subfolders are allowed: {}",
                item.get_object_string(),
                immediate_subfolder,
                allowed_subfolders.join(", ")
            ),
            item.get_relative_path().cloned(),
        ))
    }
}

/// Normalizes a path by converting backslashes to forward slashes
/// and collapsing multiple consecutive slashes into single slashes.
fn normalize_path(path: &str) -> String {
    let mut normalized = path.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized
}

// Constructs the base path string based on resource type, optional prefix, and optional postfix.
// Example: /{path_prefix/}models/{path_postfix/}bronze/
fn build_base_path(
    resource_type: &str,
    path_prefix: Option<&String>,
    path_postfix: Option<&String>,
) -> String {
    let mut base_path = String::new();

    if let Some(prefix) = path_prefix {
        base_path.push_str(&normalize_path(prefix));
        if !base_path.ends_with('/') {
            base_path.push('/');
        }
    }

    base_path.push_str(resource_type);

    if let Some(postfix) = path_postfix {
        if !base_path.ends_with('/') {
            base_path.push('/');
        }
        base_path.push_str(&normalize_path(postfix));
    }

    if !base_path.ends_with('/') {
        base_path.push('/');
    }

    base_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    struct TestObject {
        name: String,
        path: String,
        rule_target: RuleTarget,
    }

    impl Identifiable for TestObject {
        fn get_object_type(&self) -> &'static str {
            "TestObject"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }

        fn get_relative_path(&self) -> Option<&String> {
            Some(&self.path)
        }
    }

    impl PathCheckable for TestObject {
        fn get_rule_target(&self) -> RuleTarget {
            self.rule_target.clone()
        }
    }

    #[test]
    fn test_allowed_subfolder_pass() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/bronze/source1/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string(), "source2".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string(), "source2".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(result, None, "Should pass - in allowed subfolder");
    }

    #[test]
    fn test_allowed_subfolder_fail() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/bronze/source3/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string(), "source2".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string(), "source2".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert!(result.is_some(), "Should fail - not in allowed subfolder");
        let result = result.unwrap();
        assert!(
            result.message.contains("source3"),
            "Message should mention the actual subfolder"
        );
        assert!(
            result.message.contains("source1, source2"),
            "Message should list allowed subfolders"
        );
    }

    #[test]
    fn test_with_path_prefix() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "dbt/models/bronze/source1/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: Some("dbt".to_string()),
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            Some(&"dbt".to_string()),
            Some(&"bronze".to_string()),
        );

        assert_eq!(result, None, "Should pass with prefix");
    }

    #[test]
    fn test_windows_backslash_paths() {
        // Test that Windows-style backslash paths are normalized
        let obj = TestObject {
            name: "test_model".to_string(),
            path: r"models\bronze\source1\file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - Windows backslashes should be normalized"
        );
    }

    #[test]
    fn test_windows_mixed_slashes() {
        // Test mixed slashes (Windows)
        let obj = TestObject {
            name: "test_model".to_string(),
            path: r"models/bronze\source1/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - mixed slashes should be normalized"
        );
    }

    #[test]
    fn test_double_slashes() {
        // Test double slashes in path
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models//bronze//source1//file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );
        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );
        assert_eq!(
            result, None,
            "Should pass - double slashes should be handled correctly"
        );
    }

    #[test]
    fn test_no_prefix_or_postfix() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/bronze/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["bronze".to_string(), "silver".to_string()],
                path_prefix: None,
                path_postfix: None,
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &[
                "bronze".to_string(),
                "silver".to_string(),
                "gold".to_string(),
            ],
            None,
            None,
        );

        assert_eq!(result, None, "Should pass - models/bronze is allowed");
    }

    #[test]
    fn test_different_resource_types() {
        let seed = TestObject {
            name: "test_seed".to_string(),
            path: "seeds/bronze/source1/file.csv".to_string(),
            rule_target: RuleTarget::Seeds,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &seed,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(result, None, "Should pass - seeds have their own path");
    }

    #[test]
    fn test_deep_nested_path() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/bronze/source1/subdir/deeper/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - deep nesting under allowed subfolder is ok"
        );
    }

    #[test]
    fn test_outside_base_path_ignored() {
        // Files outside the base path should be ignored (pass)
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/silver/source1/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - file is not in base path (models/bronze/), so rule doesn't apply"
        );
    }

    #[test]
    fn test_intermediate_folder_ignored() {
        // Test that models/intermediate/ is ignored when rule is for models/staging/
        let obj = TestObject {
            name: "int_model".to_string(),
            path: "models/intermediate/finance/int_model.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![
                    "bronze".to_string(),
                    "silver".to_string(),
                    "gold".to_string(),
                ],
                path_prefix: None,
                path_postfix: Some("staging".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &[
                "bronze".to_string(),
                "silver".to_string(),
                "gold".to_string(),
            ],
            None,
            Some(&"staging".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - models/intermediate/ is not in base path (models/staging/)"
        );
    }

    #[test]
    fn test_marts_folder_ignored() {
        // Test that models/marts/ is ignored when rule is for models/staging/
        let obj = TestObject {
            name: "customers".to_string(),
            path: "models/marts/finance/customers.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![
                    "bronze".to_string(),
                    "silver".to_string(),
                    "gold".to_string(),
                ],
                path_prefix: None,
                path_postfix: Some("staging".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &[
                "bronze".to_string(),
                "silver".to_string(),
                "gold".to_string(),
            ],
            None,
            Some(&"staging".to_string()),
        );

        assert_eq!(
            result, None,
            "Should pass - models/marts/ is not in base path (models/staging/)"
        );
    }

    #[test]
    fn test_directly_in_base_path_no_subfolder() {
        let obj = TestObject {
            name: "test_model".to_string(),
            path: "models/bronze/file.sql".to_string(),
            rule_target: RuleTarget::Models,
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec!["source1".to_string(), "source2".to_string()],
                path_prefix: None,
                path_postfix: Some("bronze".to_string()),
            },
            Severity::Error,
        );

        let result = check_allowed_subfolders(
            &obj,
            &rule,
            &["source1".to_string(), "source2".to_string()],
            None,
            Some(&"bronze".to_string()),
        );

        assert!(
            result.is_some(),
            "Should fail - file is directly in bronze without source1 or source2 subfolder"
        );
        let result = result.unwrap();
        assert!(
            result.message.contains("directly in"),
            "Message should mention it's directly in the base path"
        );
    }
}
