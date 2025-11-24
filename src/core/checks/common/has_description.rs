use crate::core::config::ManifestRule;
use crate::core::config::Severity;
use crate::core::traits::Descriptable;
use tabled::Tabled;

#[derive(Tabled, PartialEq, Eq, Debug)]
pub struct CheckRow {
    #[tabled(rename = "Severity")]
    pub severity: String,
    #[tabled(rename = "Object")]
    pub object_type: String,
    #[tabled(rename = "Message")]
    pub message: String,
}

impl CheckRow {
    pub fn new(
        severity: &Severity,
        object_type: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let sev_str = severity.as_str().to_string();
        Self {
            severity: sev_str,
            object_type: object_type.into(),
            message: message.into(),
        }
    }
}

pub fn check_node_description<T: Descriptable>(
    descriptable: &T,
    rule: &ManifestRule,
) -> Result<CheckRow, ()> {
    match descriptable.description() {
        Some(desc) if !desc.trim().is_empty() => Err(()),
        _ => Ok(CheckRow::new(
            &rule.severity,
            descriptable.get_object_type(),
            format!(
                "{} is missing a description.",
                descriptable.get_object_string()
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{ManifestRule, Severity, SpecificRuleConfig};
    use crate::core::traits::Descriptable;
    struct TestNode {
        name: String,
        description: Option<String>,
    }
    impl Descriptable for TestNode {
        fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }

        fn get_object_type(&self) -> String {
            "TestNode".to_string()
        }

        fn get_object_string(&self) -> String {
            format!("node '{}'", self.name)
        }
    }
    #[test]
    fn test_check_warning() {
        let rule = ManifestRule {
            name: "has_description".to_string(),
            severity: Severity::Warning,
            description: None,
            applies_to: None,
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
        assert_eq!(check_node_description(&node_with_desc, &rule), Err(()));
        assert_eq!(
            check_node_description(&node_without_desc, &rule),
            Ok(CheckRow::new(
                &Severity::Warning,
                "TestNode",
                "node 'TestNode2' is missing a description."
            ))
        );
    }
    #[test]
    fn test_check_error() {
        let rule = ManifestRule {
            name: "has_description".to_string(),
            severity: Severity::Error,
            description: None,
            applies_to: None,
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
        assert_eq!(check_node_description(&node_with_desc, &rule), Err(()));
        assert_eq!(
            check_node_description(&node_without_desc, &rule),
            Ok(CheckRow::new(
                &Severity::Error,
                "TestNode",
                "node 'TestNode4' is missing a description."
            ))
        );
    }
}
