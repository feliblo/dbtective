use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

use super::code_max_lines::HasCode;
use super::utils::strip_sql_comments;

use regex::Regex;
use std::sync::LazyLock;

/// Regex that matches hardcoded table references after FROM or JOIN keywords.
///
/// Detects patterns like:
/// - `FROM schema.table`
/// - `JOIN schema.table`
/// - `FROM "schema"."table"`
/// - `FROM [schema].[table]`
/// - `FROM db.schema.table`
/// - `FROM db."schema"."table"`
///
/// Does NOT match:
/// - CTE references (single unqualified identifiers like `FROM my_cte`)
/// - Column selects (e.g., `a.column_name` in SELECT)
///
/// The regex handles optional whitespace/newlines between FROM/JOIN and the table ref.
static HARDCODED_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    // An identifier is either: quoted ("x" or [x]), or unquoted word
    let ident = r#"(?:"[^"]+"|`[^`]+`|\[[^\]]+\]|\w+)"#;

    // Match FROM/JOIN (with optional prefix like LEFT/INNER/etc) followed by schema.table
    // Supports 2-part (schema.table) or 3-part (db.schema.table) references
    // (?i) = case-insensitive
    // \b = word boundary to avoid matching inside other words
    let pattern = format!(r"(?i)\b(?:FROM|JOIN)\s+{ident}\.{ident}(?:\.{ident})?");

    Regex::new(&pattern).expect("hardcoded ref regex must compile")
});

/// Find all hardcoded table references in SQL code.
/// Returns the matched reference strings.
fn find_hardcoded_refs(code: &str) -> Vec<String> {
    let stripped = strip_sql_comments(code);
    HARDCODED_REF_RE
        .find_iter(&stripped)
        .map(|m| {
            // Extract just the table reference (everything after FROM/JOIN + whitespace)
            let matched = m.as_str();
            let lower = matched.to_lowercase();
            let keyword_end = if lower.starts_with("from") {
                4
            } else {
                // find "join" position
                lower.find("join").map_or(0, |pos| pos + 4)
            };
            matched[keyword_end..].trim().to_string()
        })
        .collect()
}

/// Check if code contains hardcoded table references (schema.table patterns after FROM/JOIN).
///
/// Strips SQL comments before checking to avoid false positives from commented-out code.
/// Detection is case-insensitive.
/// Returns `None` if no hardcoded references are found (pass).
/// Returns `Some(RuleResult)` if hardcoded references are found (fail).
/// Gracefully skips objects with no code (returns `None`).
pub fn code_no_hardcoded_refs<T: HasCode>(
    object_with_code: &T,
    rule: &ManifestRule,
) -> Option<RuleResult> {
    let raw_code = match object_with_code.get_raw_code() {
        Some(code) if !code.trim().is_empty() => code,
        Some(_) | None => {
            return None;
        }
    };

    let refs = find_hardcoded_refs(raw_code);

    if refs.is_empty() {
        return None;
    }

    let refs_display = refs
        .iter()
        .map(|r| format!("'{r}'"))
        .collect::<Vec<_>>()
        .join(", ");

    let message = format!(
        "{} contains hardcoded table reference(s): {}. Use ref() or source() instead.",
        object_with_code.get_object_string(),
        refs_display
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

    fn create_test_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::CodeNoHardcodedRefs {},
            Severity::Warning,
        )
    }

    // === SHOULD TRIGGER ===

    #[test]
    fn test_from_schema_dot_table() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id, name FROM analytics.orders WHERE active = true".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("analytics.orders"));
    }

    #[test]
    fn test_join_schema_dot_table() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT a.id FROM {{ ref('users') }} a JOIN analytics.orders b ON a.id = b.order_id".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("analytics.orders"));
    }

    #[test]
    fn test_quoted_schema_dot_table() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some(r#"SELECT id FROM "analytics"."orders" WHERE 1=1"#.to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_bracket_quoted_schema_dot_table() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id FROM [analytics].[orders] WHERE 1=1".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_three_part_ref() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id FROM db.analytics.orders".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_left_join_hardcoded() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT a.id FROM {{ ref('users') }} a LEFT JOIN raw.customers c ON a.id = c.user_id".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("raw.customers"));
    }

    #[test]
    fn test_from_on_next_line() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id\nFROM\n  analytics.orders\nWHERE 1=1".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("analytics.orders"));
    }

    #[test]
    fn test_join_on_next_line() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT a.id\nFROM {{ ref('users') }} a\nJOIN\n  analytics.orders b ON a.id = b.user_id".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_case_insensitive() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("select id from Analytics.Orders where 1=1".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
    }

    #[test]
    fn test_multiple_hardcoded_refs() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some(
                "SELECT a.id FROM analytics.orders a JOIN raw.customers b ON a.id = b.id"
                    .to_string(),
            ),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("analytics.orders"));
        assert!(msg.contains("raw.customers"));
    }

    // === SHOULD NOT TRIGGER ===

    #[test]
    fn test_cte_reference_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some(
                "WITH my_cte AS (SELECT * FROM {{ ref('users') }})\nSELECT id FROM my_cte"
                    .to_string(),
            ),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_ref_function_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id FROM {{ ref('orders') }} WHERE active = true".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_source_function_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT id FROM {{ source('raw', 'orders') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_commented_hardcoded_ref_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("-- FROM analytics.orders\nSELECT id FROM {{ ref('orders') }}".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_block_commented_hardcoded_ref_passes() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some(
                "/* FROM analytics.orders */\nSELECT id FROM {{ ref('orders') }}".to_string(),
            ),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
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
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_code_returns_none() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "empty_model".to_string(),
            code: Some(String::new()),
            relative_path: Some("models/empty_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_column_select_not_flagged() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "my_model".to_string(),
            code: Some("SELECT a.id, b.name FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id".to_string()),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_none());
    }

    #[test]
    fn test_severity_and_metadata() {
        let rule = create_test_rule();
        let node = TestNode {
            name: "test_model".to_string(),
            code: Some("SELECT 1 FROM analytics.orders".to_string()),
            relative_path: Some("models/test_model.sql".to_string()),
        };
        let result = code_no_hardcoded_refs(&node, &rule);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.severity, "WARN");
        assert_eq!(rule_result.object_type, "Model");
        assert_eq!(rule_result.rule_name, "code_no_hardcoded_refs");
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    // === find_hardcoded_refs unit tests ===

    #[test]
    fn test_find_hardcoded_refs_basic() {
        let refs = find_hardcoded_refs("SELECT * FROM schema.table");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "schema.table");
    }

    #[test]
    fn test_find_hardcoded_refs_none() {
        let refs = find_hardcoded_refs("SELECT * FROM my_cte WHERE 1=1");
        assert!(refs.is_empty());
    }

    #[test]
    fn test_find_hardcoded_refs_ignores_comments() {
        let refs = find_hardcoded_refs("-- FROM schema.table\nSELECT 1");
        assert!(refs.is_empty());
    }

    #[test]
    fn test_find_hardcoded_refs_multiline() {
        let refs = find_hardcoded_refs("SELECT id\nFROM\n  schema.table\nWHERE 1=1");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "schema.table");
    }
}
