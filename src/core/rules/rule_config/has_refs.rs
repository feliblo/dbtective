use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

// Characterizes objects that can have upstream references.
// Indicated by {{ ref(model_name) }} statements in dbt models or {{ source(...) }} statements.
pub trait CanReference {
    fn get_depends_on_nodes(&self) -> &[String];
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

pub fn has_refs<T: CanReference>(item: &T, rule: &ManifestRule) -> Option<RuleResult> {
    let referenced_objects = item.get_depends_on_nodes();

    if referenced_objects.is_empty() {
        let error_msg = format!(
            "{} does not have any upstream references (refs/sources)",
            item.get_object_string()
        );

        return Some(RuleResult::new(
            &rule.severity,
            item.get_object_type(),
            rule.get_name(),
            error_msg,
            item.get_relative_path().cloned(),
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    struct TestCanReference {
        name: String,
        depends_on: Vec<String>,
        relative_path: String,
    }
    impl CanReference for TestCanReference {
        fn get_depends_on_nodes(&self) -> &[String] {
            &self.depends_on
        }

        fn get_object_type(&self) -> &'static str {
            "TestObject"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }

        fn get_relative_path(&self) -> Option<&String> {
            Some(&self.relative_path)
        }
    }

    #[test]
    fn test_has_refs_with_no_refs() {
        let test_item = TestCanReference {
            name: "test_model".to_string(),
            depends_on: vec![],
            relative_path: "models/test_model.sql".to_string(),
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasRefs {},
            Severity::Warning,
        );
        let result = has_refs(&test_item, &rule);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.severity, "WARN");
        assert_eq!(rule_result.object_type, "TestObject");
        assert_eq!(
            rule_result.message,
            "test_model does not have any upstream references (refs/sources)"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    #[test]
    fn test_has_refs_with_refs() {
        let test_item = TestCanReference {
            name: "test_model".to_string(),
            depends_on: vec!["ref_model".to_string()],
            relative_path: "models/test_model.sql".to_string(),
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasRefs {},
            Severity::Warning,
        );
        let result = has_refs(&test_item, &rule);
        assert!(result.is_none());
    }
}
