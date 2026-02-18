use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

use super::max_code_lines::HasCode;
use super::utils::strip_sql_comments;

/// Count the number of JOIN keywords in SQL code (case-insensitive).
/// Only counts standalone JOIN keywords (including LEFT JOIN, INNER JOIN, etc.).
fn count_joins(code: &str) -> usize {
    let stripped = strip_sql_comments(code);
    let lowered = stripped.to_lowercase();
    lowered.matches("join").count()
}

/// Check if code exceeds the maximum allowed number of JOINs.
///
/// Strips SQL comments before counting to avoid false positives from commented-out code.
/// Detection is case-insensitive.
/// Returns `None` if the code has no JOINs or is within the limit (pass).
/// Returns `Some(RuleResult)` if the code exceeds the limit (fail).
/// Gracefully skips objects with no code (returns `None`).
pub fn max_joins<T: HasCode>(
    object_with_code: &T,
    rule: &ManifestRule,
    max_joins_allowed: usize,
) -> Option<RuleResult> {
    let raw_code = match object_with_code.get_raw_code() {
        Some(code) if !code.trim().is_empty() => code,
        Some(_) | None => {
            // No raw code available or empty — skip gracefully
            return None;
        }
    };

    let join_count = count_joins(raw_code);

    if join_count <= max_joins_allowed {
        return None;
    }

    let message = format!(
        "{} has {} JOIN(s) which exceeds the maximum allowed of {}.",
        object_with_code.get_object_string(),
        join_count,
        max_joins_allowed
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
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }

    impl HasCode for TestNode {
        fn get_raw_code(&self) -> Option<&str> {
            self.code.as_deref()
        }
    }

    fn create_test_rule(max: usize) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::MaxJoins { max_joins: max },
            Severity::Warning,
        )
    }

    #[test]
    fn test_no_joins_passes() {
        let rule = create_test_rule(3);
        let node = TestNode {
            name: "simple_model".to_string(),
            code: Some("SELECT id, name FROM users WHERE active = true".to_string()),
            relative_path: Some("models/simple_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_within_limit_passes() {
        let rule = create_test_rule(3);
        let node = TestNode {
            name: "join_model".to_string(),
            code: Some(
                "SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id JOIN products c ON b.product_id = c.id".to_string(),
            ),
            relative_path: Some("models/join_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_exceeds_limit_fails() {
        let rule = create_test_rule(2);
        let node = TestNode {
            name: "complex_model".to_string(),
            code: Some(
                "SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id JOIN products c ON b.product_id = c.id JOIN categories d ON c.cat_id = d.id".to_string(),
            ),
            relative_path: Some("models/complex_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 2);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("complex_model"));
        assert!(msg.contains("3 JOIN(s)"));
        assert!(msg.contains("maximum allowed of 2"));
    }

    #[test]
    fn test_case_insensitive_detection() {
        let rule = create_test_rule(1);
        let node = TestNode {
            name: "mixed_case_model".to_string(),
            code: Some(
                "SELECT a.id FROM users a join orders b ON a.id = b.user_id JOIN products c ON b.product_id = c.id".to_string(),
            ),
            relative_path: Some("models/mixed_case_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 1);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("2 JOIN(s)"));
    }

    #[test]
    fn test_commented_joins_not_counted() {
        let rule = create_test_rule(1);
        let node = TestNode {
            name: "commented_model".to_string(),
            code: Some(
                "-- JOIN old_table ON ...\nSELECT a.id FROM users a JOIN orders b ON a.id = b.user_id".to_string(),
            ),
            relative_path: Some("models/commented_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_block_commented_joins_not_counted() {
        let rule = create_test_rule(1);
        let node = TestNode {
            name: "block_commented_model".to_string(),
            code: Some(
                "/* JOIN old_table ON ... \n LEFT JOIN another ON ... */ SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id".to_string(),
            ),
            relative_path: Some("models/block_commented_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_no_code_returns_none() {
        let rule = create_test_rule(3);
        let node = TestNode {
            name: "no_code_model".to_string(),
            code: None,
            relative_path: Some("models/no_code_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_left_join_counted() {
        let rule = create_test_rule(0);
        let node = TestNode {
            name: "left_join_model".to_string(),
            code: Some(
                "SELECT a.id FROM users a LEFT JOIN orders b ON a.id = b.user_id".to_string(),
            ),
            relative_path: Some("models/left_join_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 0);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("1 JOIN(s)"));
    }

    #[test]
    fn test_cross_join_counted() {
        let rule = create_test_rule(0);
        let node = TestNode {
            name: "cross_join_model".to_string(),
            code: Some("SELECT a.id FROM users a CROSS JOIN orders b".to_string()),
            relative_path: Some("models/cross_join_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_exact_limit_passes() {
        let rule = create_test_rule(2);
        let node = TestNode {
            name: "exact_limit_model".to_string(),
            code: Some(
                "SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id JOIN products c ON b.product_id = c.id".to_string(),
            ),
            relative_path: Some("models/exact_limit_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_severity_and_metadata() {
        let rule = create_test_rule(0);
        let node = TestNode {
            name: "test_model".to_string(),
            code: Some("SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };

        let result = max_joins(&node, &rule, 0);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.severity, "WARN");
        assert_eq!(rule_result.object_type, "Model");
        assert_eq!(rule_result.rule_name, "max_joins");
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    #[test]
    fn test_count_joins_basic() {
        assert_eq!(count_joins("SELECT * FROM users"), 0);
        assert_eq!(
            count_joins("SELECT * FROM users JOIN orders ON users.id = orders.user_id"),
            1
        );
        assert_eq!(
            count_joins("SELECT * FROM a JOIN b ON a.id = b.id JOIN c ON b.id = c.id"),
            2
        );
    }

    #[test]
    fn test_count_joins_with_prefixes() {
        assert_eq!(
            count_joins("SELECT * FROM a LEFT JOIN b ON a.id = b.id INNER JOIN c ON b.id = c.id"),
            2
        );
        assert_eq!(
            count_joins("SELECT * FROM a FULL OUTER JOIN b ON a.id = b.id"),
            1
        );
    }

    #[test]
    fn test_count_joins_ignores_comments() {
        assert_eq!(count_joins("-- JOIN old\nSELECT * FROM users"), 0);
        assert_eq!(
            count_joins("/* JOIN old */ SELECT * FROM users JOIN orders ON 1=1"),
            1
        );
    }
}
