use crate::cli::table::RuleResult;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

pub trait Descriptable: Identifiable {
    fn description(&self) -> Option<&String>;
}

pub fn has_description<T: Descriptable>(
    descriptable: &T,
    rule: &ManifestRule,
    min_length: Option<usize>,
    forbidden_substrings: Option<&[String]>,
) -> Option<RuleResult> {
    let desc = match descriptable.description() {
        Some(d) if !d.trim().is_empty() => d,
        _ => {
            return Some(RuleResult::new(
                &rule.severity,
                descriptable.get_object_type(),
                rule.get_name(),
                format!(
                    "{} is missing a description.",
                    descriptable.get_object_string()
                ),
                descriptable.get_problematic_path(false).map(str::to_owned),
            ));
        }
    };

    if let Some(min) = min_length {
        if desc.trim().len() < min {
            return Some(RuleResult::new(
                &rule.severity,
                descriptable.get_object_type(),
                rule.get_name(),
                format!(
                    "{} has a description that is too short ({} chars, minimum is {}).",
                    descriptable.get_object_string(),
                    desc.trim().len(),
                    min
                ),
                descriptable.get_problematic_path(false).map(str::to_owned),
            ));
        }
    }

    forbidden_substrings.and_then(|forbidden| {
        forbidden
            .iter()
            .find(|pattern| desc.contains(pattern.as_str()))
            .map(|pattern| {
                RuleResult::new(
                    &rule.severity,
                    descriptable.get_object_type(),
                    rule.get_name(),
                    format!(
                        "{} has a description containing forbidden substring \"{}\".",
                        descriptable.get_object_string(),
                        pattern
                    ),
                    descriptable.get_problematic_path(false).map(str::to_owned),
                )
            })
    })
}

#[cfg(test)]
mod tests {
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    use super::*;

    struct TestNode {
        name: String,
        description: Option<String>,
    }
    impl Identifiable for TestNode {
        fn get_object_type(&self) -> &'static str {
            "TestNode"
        }
        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            None
        }
        fn get_object_string(&self) -> &str {
            &self.name
        }
    }
    impl Descriptable for TestNode {
        fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }
    }

    fn make_rule(severity: Severity) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            },
            severity,
        )
    }

    #[test]
    fn test_check_warning() {
        let rule = make_rule(Severity::Warning);
        let node_with_desc = TestNode {
            name: "TestNode1".to_string(),
            description: Some("This is a test node.".to_string()),
        };
        let node_without_desc = TestNode {
            name: "TestNode2".to_string(),
            description: None,
        };
        assert_eq!(has_description(&node_with_desc, &rule, None, None), None);
        assert_eq!(
            has_description(&node_without_desc, &rule, None, None),
            Some(RuleResult::new(
                &Severity::Warning,
                "TestNode",
                "has_description",
                "TestNode2 is missing a description.",
                node_without_desc
                    .get_problematic_path(false)
                    .map(str::to_owned),
            ))
        );
    }

    #[test]
    fn test_check_error() {
        let rule = make_rule(Severity::Error);
        let node_with_desc = TestNode {
            name: "TestNode3".to_string(),
            description: Some("This is another test node.".to_string()),
        };
        let node_without_desc = TestNode {
            name: "TestNode4".to_string(),
            description: None,
        };
        assert_eq!(has_description(&node_with_desc, &rule, None, None), None);
        assert_eq!(
            has_description(&node_without_desc, &rule, None, None),
            Some(RuleResult::new(
                &Severity::Error,
                "TestNode",
                "has_description",
                "TestNode4 is missing a description.",
                node_without_desc
                    .get_problematic_path(false)
                    .map(str::to_owned),
            ))
        );
    }

    #[test]
    fn test_check_nonempty_description() {
        let rule = make_rule(Severity::Warning);
        let node = TestNode {
            name: "TestNode5".to_string(),
            description: Some("This is a valid description".to_string()),
        };
        assert_eq!(has_description(&node, &rule, None, None), None);
    }

    #[test]
    fn test_min_length_passes() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("A valid long enough description".to_string()),
        };
        assert_eq!(has_description(&node, &rule, Some(10), None), None);
    }

    #[test]
    fn test_min_length_fails() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("Short".to_string()),
        };
        let result = has_description(&node, &rule, Some(10), None);
        assert!(result.is_some());
        assert!(result
            .unwrap()
            .message
            .contains("too short (5 chars, minimum is 10)"));
    }

    #[test]
    fn test_min_length_trims_whitespace() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("  ab  ".to_string()),
        };
        let result = has_description(&node, &rule, Some(5), None);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("too short (2 chars"));
    }

    #[test]
    fn test_forbidden_substrings_passes() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("A real production description".to_string()),
        };
        let forbidden = vec!["TODO".to_string(), "test_description".to_string()];
        assert_eq!(has_description(&node, &rule, None, Some(&forbidden)), None);
    }

    #[test]
    fn test_forbidden_substrings_fails() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("This is a test_description value".to_string()),
        };
        let forbidden = vec!["TODO".to_string(), "test_description".to_string()];
        let result = has_description(&node, &rule, None, Some(&forbidden));
        assert!(result.is_some());
        assert!(result
            .unwrap()
            .message
            .contains("forbidden substring \"test_description\""));
    }

    #[test]
    fn test_forbidden_substrings_first_match_wins() {
        let rule = make_rule(Severity::Warning);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("TODO: fix this test_description".to_string()),
        };
        let forbidden = vec!["TODO".to_string(), "test_description".to_string()];
        let result = has_description(&node, &rule, None, Some(&forbidden));
        assert!(result.is_some());
        assert!(result
            .unwrap()
            .message
            .contains("forbidden substring \"TODO\""));
    }

    #[test]
    fn test_min_length_and_forbidden_substrings_combined() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("A proper description of the model".to_string()),
        };
        let forbidden = vec!["TODO".to_string()];
        assert_eq!(
            has_description(&node, &rule, Some(10), Some(&forbidden)),
            None
        );
    }

    #[test]
    fn test_min_length_checked_before_forbidden() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: Some("TODO".to_string()),
        };
        let forbidden = vec!["TODO".to_string()];
        let result = has_description(&node, &rule, Some(10), Some(&forbidden));
        assert!(result.is_some());
        // min_length is checked first, so the message should be about length
        assert!(result.unwrap().message.contains("too short"));
    }

    #[test]
    fn test_missing_description_takes_precedence() {
        let rule = make_rule(Severity::Error);
        let node = TestNode {
            name: "model_a".to_string(),
            description: None,
        };
        let forbidden = vec!["TODO".to_string()];
        let result = has_description(&node, &rule, Some(10), Some(&forbidden));
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("missing a description"));
    }
}
