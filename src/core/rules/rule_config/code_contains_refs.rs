use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

use super::code_max_lines::HasCode;
use super::utils::strip_sql_comments;

/// Check if code contains `ref()` or `source()` function calls.
/// Strips SQL comments before checking to avoid false positives from commented-out code.
/// Detection is case-insensitive.
pub fn code_contains_refs<T: HasCode>(
    object_with_code: &T,
    rule: &ManifestRule,
) -> Option<RuleResult> {
    let raw_code = match object_with_code.get_raw_code() {
        Some(code) if !code.trim().is_empty() => code,
        Some(_) | None => {
            // No code available — skip gracefully
            return None;
        }
    };

    let stripped = strip_sql_comments(raw_code);
    let lowered = stripped.to_lowercase();

    if lowered.contains("ref(") || lowered.contains("source(") {
        return None;
    }

    let message = format!(
        "{} does not contain any ref() or source() function calls in its code",
        object_with_code.get_object_string()
    );

    Some(RuleResult::new(
        &rule.severity,
        object_with_code.get_object_type(),
        rule.get_name(),
        message,
        object_with_code
            .get_problematic_path(true)
            .map(str::to_owned),
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
        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }

    impl HasCode for TestNode {
        fn get_raw_code(&self) -> Option<&str> {
            self.code.as_deref()
        }
    }

    fn create_test_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeContainsRefs {},
            Severity::Warning,
        )
    }

    #[test]
    fn test_code_with_ref_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT * FROM {{ ref('other_model') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_code_with_source_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT * FROM {{ source('raw', 'users') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_code_without_refs_fails() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "hardcoded_model".to_string(),
            code: Some("SELECT * FROM raw_schema.users WHERE active = true".to_string()),
            relative_path: Some("models/hardcoded_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("hardcoded_model"));
        assert!(msg.contains("does not contain any ref() or source()"));
    }

    #[test]
    fn test_commented_out_ref_fails() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "commented_model".to_string(),
            code: Some(
                "-- SELECT * FROM {{ ref('model') }}\nSELECT * FROM raw_schema.users".to_string(),
            ),
            relative_path: Some("models/commented_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_multiline_commented_ref_fails() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "commented_model".to_string(),
            code: Some("/* {{ ref('model') }} */\nSELECT * FROM raw_schema.users".to_string()),
            relative_path: Some("models/commented_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_case_insensitive_ref_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT * FROM {{ REF('other_model') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_case_insensitive_source_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT * FROM {{ SOURCE('raw', 'users') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_no_code_returns_none() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "no_code_model".to_string(),
            code: None,
            relative_path: Some("models/no_code_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_ref_after_comment_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "mixed_model".to_string(),
            code: Some(
                "-- old code\nSELECT * FROM {{ ref('valid_model') }}\nWHERE 1=1".to_string(),
            ),
            relative_path: Some("models/mixed_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_both_ref_and_source_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "multi_ref_model".to_string(),
            code: Some(
                "SELECT * FROM {{ ref('model_a') }} JOIN {{ source('raw', 'users') }}".to_string(),
            ),
            relative_path: Some("models/multi_ref_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_severity_and_metadata() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "test_model".to_string(),
            code: Some("SELECT 1 FROM hardcoded_table".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };

        let result = code_contains_refs(&node, &rule);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.severity, "WARN");
        assert_eq!(rule_result.object_type, "Model");
        assert_eq!(rule_result.rule_name, "code_contains_refs");
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }
}
