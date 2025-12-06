use crate::{
    cli::table::RuleResult,
    core::{
        config::manifest_rule::ManifestRule,
        manifest::{dbt_objects::nodes, Manifest},
    },
};

pub trait TestAble {
    fn get_tests<'a>(&'a self, manifest: &'a Manifest) -> Vec<&'a nodes::test::Test> {
        let unique_id = self.get_unique_id();
        manifest.get_tests_by_parent(unique_id)
    }
    fn get_unique_id(&self) -> &String;
    fn get_object_string(&self) -> &String;
    fn get_object_type(&self) -> String;
    fn get_relative_path(&self) -> Option<&String>;
}

// Check if the testable has atleast one test that tests for uniqueness
pub fn has_unique_test<T: TestAble>(
    testable: &T,
    rule: &ManifestRule,
    manifest: &Manifest,
    allowed_test_names: &[String],
) -> Option<RuleResult> {
    let unique_tests_found = testable
        .get_tests(manifest)
        .into_iter()
        .filter(|test| {
            // If there is a name and it matches => count it as a unique test
            test.get_metadata_name()
                .is_some_and(|test_name| allowed_test_names.iter().any(|name| name == &test_name))
        })
        .count();

    if unique_tests_found == 0 {
        Some(RuleResult::new(
            &rule.severity,
            testable.get_object_type(),
            rule.get_name(),
            format!(
                "{} does should have a unique test",
                testable.get_object_string(),
            ),
            testable.get_relative_path().cloned(),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::{
            check_config_options::default_allowed_test_names,
            manifest_rule::ManifestSpecificRuleConfig, severity::Severity,
        },
        manifest::{
            dbt_objects::{
                nodes::test::{Test, TestMetadata},
                Node,
            },
            Manifest,
        },
    };

    struct MockTestable {
        unique_id: String,
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
    }
    impl TestAble for MockTestable {
        fn get_unique_id(&self) -> &String {
            &self.unique_id
        }

        fn get_object_string(&self) -> &String {
            &self.object_string
        }

        fn get_object_type(&self) -> String {
            self.object_type.clone()
        }

        fn get_relative_path(&self) -> Option<&String> {
            self.relative_path.as_ref()
        }
    }

    fn create_test_manifest(tests: Vec<Test>) -> Manifest {
        let mut manifest = Manifest::default();
        for test in tests {
            manifest
                .nodes
                .insert(test.base.unique_id.clone(), Node::Test(test));
        }
        manifest
    }

    fn create_mock_test(name: &str, attached_node: &str) -> Test {
        let mut test = Test::default();
        test.base.name = name.to_string();
        test.test_metadata = Some(TestMetadata {
            name: name.to_string(),
            kwargs: None,
            namespace: None,
        });
        test.attached_node = Some(attached_node.to_string());
        test.base.unique_id = format!("test.{name}");
        test
    }

    #[test]
    fn test_has_unique_test_found() {
        let manifest = create_test_manifest(vec![create_mock_test("unique", "model.my_model")]);
        let rule = ManifestRule {
            name: Some(String::new()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
        };
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "model.my_model".to_string(),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_none());
    }

    #[test]
    fn test_has_unique_test_not_found() {
        let manifest = create_test_manifest(vec![]);
        let rule = ManifestRule {
            name: Some(String::new()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
        };
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "model.my_model".to_string(),

            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };

        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "model");
        assert_eq!(rule_result.rule_name, "");
        assert_eq!(
            rule_result.message,
            "my_model does should have a unique test"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/my_model.sql".to_string())
        );
    }

    #[test]
    fn test_has_unique_test_with_not_allowed_name() {
        let manifest = create_test_manifest(vec![create_mock_test("not_unique", "model.my_model")]);
        let rule = ManifestRule {
            name: Some(String::new()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
        };
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "model.my_model".to_string(),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().message,
            "my_model does should have a unique test"
        );
    }

    #[test]
    fn test_custom_allowed_test_names() {
        let manifest = create_test_manifest(vec![create_mock_test(
            "custom_unique_test",
            "model.my_model",
        )]);
        let rule = ManifestRule {
            name: Some(String::new()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: vec!["custom_unique_test".to_string()],
            },
        };
        let allowed = vec!["custom_unique_test".to_string()];
        let testable = MockTestable {
            unique_id: "model.my_model".to_string(),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_none());
    }

    #[test]
    fn test_default_allowed_test_names() {
        let allowed = default_allowed_test_names();
        assert!(allowed.contains(&"unique".to_string()));
        assert!(allowed.contains(&"dbt_utils.unique_combination_of_columns".to_string()));
    }
}
