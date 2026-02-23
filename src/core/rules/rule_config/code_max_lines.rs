use crate::{
    cli::table::RuleResult,
    core::{config::manifest_rule::ManifestRule, rules::common_traits::Identifiable},
};

pub trait HasCode: Identifiable {
    fn get_raw_code(&self) -> Option<&str>;
}

// Models, Macros, Snapshots can all contain code.
pub fn code_max_lines<T: HasCode>(
    object_with_code: &T,
    rule: &ManifestRule,
    max_length: usize,
) -> Option<RuleResult> {
    let code_lines = object_with_code
        .get_raw_code()
        .map_or(0, |code| code.lines().count());

    if code_lines > 0 && code_lines <= max_length {
        return None;
    }

    let message = if code_lines == 0 {
        format!(
            "Code for '{}' is empty. ",
            object_with_code.get_object_string()
        )
    } else {
        format!(
            "{} has {} lines of code which exceeds the maximum allowed of {} lines.",
            object_with_code.get_object_string(),
            code_lines,
            max_length
        )
    };
    Some(RuleResult::new(
        &rule.severity,
        object_with_code.get_object_type(),
        rule.get_name(),
        message,
        object_with_code
            .get_problematic_path(false)
            .map(str::to_owned),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

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
        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }
    impl HasCode for TestNode {
        fn get_raw_code(&self) -> Option<&str> {
            self.code.as_deref()
        }
    }

    #[test]
    fn test_code_max_lines() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );

        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: Some("line1\nline2\nline3\nline4\nline5".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };

        // Test with max_length = 3 (should trigger the rule)
        let result = code_max_lines(&test_node, &rule, 3);
        assert!(result.is_some());

        // Test with max_length = 5 (should not trigger the rule)
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_max_code_lines_no_code() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );
        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: None,
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("is empty"));
    }

    #[test]
    fn test_max_code_lines_empty_code() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );
        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: Some(String::new()),
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("is empty"));
    }
    #[test]
    fn test_max_code_lines_below_limit() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );
        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: Some("line1\nline2".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_max_code_lines_exact_limit() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );
        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: Some("line1\nline2\nline3\nline4\nline5   ".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_max_code_lines_above_limit() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 5 },
            Severity::Warning,
        );
        let test_node = TestNode {
            name: "Test Model".to_string(),
            code: Some("line1\nline2\nline3\nline4\nline5\nline6".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_max_lines(&test_node, &rule, 5);
        assert!(result.is_some());
        assert!(result
            .unwrap()
            .message
            .contains("exceeds the maximum allowed"));
    }
}
