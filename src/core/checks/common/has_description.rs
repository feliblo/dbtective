use crate::cli::table::CheckRow;
use crate::core::config::parse_config::ManifestRule;
use crate::core::traits::Descriptable;

pub fn has_description<T: Descriptable>(
    descriptable: &T,
    rule: &ManifestRule,
) -> Result<CheckRow, ()> {
    let description = descriptable.description();
    match description {
        Some(desc) if !desc.trim().is_empty() => Err(()),
        _ => Ok(CheckRow::new(
            &rule.severity,
            Descriptable::get_object_type(descriptable),
            rule.get_name(),
            format!(
                "{} is missing a description.",
                Descriptable::get_object_string(descriptable)
            ),
        )),
    }
}
#[cfg(test)]
mod tests {
    use crate::core::config::{
        applies_to::AppliesTo, parse_config::SpecificRuleConfig, severity::Severity,
    };

    use super::*;

    struct TestNode {
        name: String,
        description: Option<String>,
    }
    impl Descriptable for TestNode {
        fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }
        #[allow(clippy::unnecessary_literal_bound)]
        fn get_object_type(&self) -> &str {
            "TestNode"
        }

        fn get_object_string(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_check_warning() {
        let rule = ManifestRule {
            name: Some("has_description".to_string()),
            severity: Severity::Warning,
            description: None,
            applies_to: Some(AppliesTo::empty()),
            includes: None,
            excludes: None,
            rule: SpecificRuleConfig::HasDescription {},
        };
        let node_with_desc = TestNode {
            name: "TestNode1".to_string(),
            description: Some("This is a test node.".to_string()),
        };
        let node_without_desc = TestNode {
            name: "TestNode2".to_string(),
            description: None,
        };
        assert_eq!(has_description(&node_with_desc, &rule), Err(()));
        assert_eq!(
            has_description(&node_without_desc, &rule),
            Ok(CheckRow::new(
                &Severity::Warning,
                "TestNode",
                "has_description",
                "TestNode2 is missing a description."
            ))
        );
    }
    #[test]
    fn test_check_error() {
        let rule = ManifestRule {
            name: Some("has_description".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: SpecificRuleConfig::HasDescription {},
        };
        let node_with_desc = TestNode {
            name: "TestNode3".to_string(),
            description: Some("This is another test node.".to_string()),
        };
        let node_without_desc = TestNode {
            name: "TestNode4".to_string(),
            description: None,
        };
        assert_eq!(has_description(&node_with_desc, &rule), Err(()));
        assert_eq!(
            has_description(&node_without_desc, &rule),
            Ok(CheckRow::new(
                &Severity::Error,
                "TestNode",
                "has_description",
                "TestNode4 is missing a description."
            ))
        );
    }

    #[test]
    fn test_check_nonempty_description() {
        let rule = ManifestRule {
            name: Some("has_description".to_string()),
            severity: Severity::Warning,
            description: None,
            includes: None,
            excludes: None,
            applies_to: None,
            rule: SpecificRuleConfig::HasDescription {},
        };
        let node_with_empty_desc = TestNode {
            name: "TestNode5".to_string(),
            description: Some("This is a valid description".to_string()),
        };
        assert_eq!(has_description(&node_with_empty_desc, &rule), Err(()));
    }
}
