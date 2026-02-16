use std::borrow::Cow;

use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

use super::max_code_lines::HasCode;

pub fn has_forbidden_code<T: HasCode>(
    object_with_code: &T,
    rule: &ManifestRule,
    forbidden_patterns: &[String],
    case_sensitive: bool,
) -> Option<RuleResult> {
    let raw_code = object_with_code.get_code()?;
    let code: Cow<'_, str> = if case_sensitive {
        Cow::Borrowed(raw_code)
    } else {
        Cow::Owned(raw_code.to_lowercase())
    };

    let matched: Vec<&String> = forbidden_patterns
        .iter()
        .filter(|pattern| {
            let p: Cow<'_, str> = if case_sensitive {
                Cow::Borrowed(pattern.as_str())
            } else {
                Cow::Owned(pattern.to_lowercase())
            };
            code.contains(p.as_ref())
        })
        .collect();

    if matched.is_empty() {
        return None;
    }

    let patterns_str = matched
        .iter()
        .map(|p| format!("'{p}'"))
        .collect::<Vec<_>>()
        .join(", ");

    let message = format!(
        "{} contains forbidden code pattern(s): {}",
        object_with_code.get_object_string(),
        patterns_str
    );

    Some(RuleResult::new(
        &rule.severity,
        object_with_code.get_object_type(),
        rule.get_name(),
        message,
        object_with_code.get_relative_path().map(str::to_owned),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};
    use crate::core::rules::common_traits::Identifiable;

    struct TestNode {
        name: String,
        code: Option<String>,
        relative_path: Option<String>,
    }

    impl Identifiable for TestNode {
        fn get_object_type(&self) -> &'static str {
            "Model"
        }
        fn get_object_string(&self) -> &str {
            &self.name
        }
        fn get_relative_path(&self) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }

    impl HasCode for TestNode {
        fn get_code(&self) -> Option<&str> {
            self.code.as_deref()
        }
    }

    fn create_test_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasForbiddenCode {
                forbidden_patterns: vec!["{{".to_string(), "{%".to_string()],
                case_sensitive: false,
            },
            Severity::Warning,
        )
    }

    #[test]
    fn test_code_with_forbidden_pattern() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT {{ ref('other') }} FROM table".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["{{".to_string(), "{%".to_string()], false);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("my_model"));
        assert!(msg.contains("'{{'"));
    }

    #[test]
    fn test_code_without_forbidden_pattern() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "clean_model".to_string(),
            code: Some("SELECT id, name FROM users WHERE active = true".to_string()),
            relative_path: Some("models/clean_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["{{".to_string(), "{%".to_string()], false);
        assert!(result.is_none());
    }

    #[test]
    fn test_code_with_multiple_forbidden_patterns() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "jinja_model".to_string(),
            code: Some("{% if flag %} SELECT {{ ref('x') }} {% endif %}".to_string()),
            relative_path: Some("models/jinja_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["{{".to_string(), "{%".to_string()], false);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("'{{'"));
        assert!(msg.contains("'{%'"));
    }

    #[test]
    fn test_no_code_returns_none() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "no_code_model".to_string(),
            code: None,
            relative_path: Some("models/no_code_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["{{".to_string(), "{%".to_string()], false);
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_forbidden_patterns_returns_none() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "any_model".to_string(),
            code: Some("SELECT {{ ref('x') }}".to_string()),
            relative_path: Some("models/any_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &[], false);
        assert!(result.is_none());
    }

    #[test]
    fn test_case_insensitive_matches_different_case() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "star_model".to_string(),
            code: Some("select * from users".to_string()),
            relative_path: Some("models/star_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["SELECT *".to_string()], false);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("'SELECT *'"));
    }

    #[test]
    fn test_case_sensitive_does_not_match_different_case() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "star_model".to_string(),
            code: Some("select * from users".to_string()),
            relative_path: Some("models/star_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["SELECT *".to_string()], true);
        assert!(result.is_none());
    }

    #[test]
    fn test_case_sensitive_matches_exact_case() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "star_model".to_string(),
            code: Some("SELECT * FROM users".to_string()),
            relative_path: Some("models/star_model.sql".to_string()),
        };

        let result = has_forbidden_code(&node, &rule, &["SELECT *".to_string()], true);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("'SELECT *'"));
    }
}
