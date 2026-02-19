use crate::{
    cli::table::RuleResult,
    core::{
        config::manifest_rule::ManifestRule, manifest::Manifest, rules::common_traits::Identifiable,
    },
};
use dbt_artifact_parser::manifest::nodes::Test;

pub trait TestAble: Identifiable {
    fn get_tests<'a>(&'a self, manifest: &'a Manifest) -> Vec<&'a Test> {
        let unique_id = self.get_unique_id();
        manifest.get_tests_by_parent(unique_id)
    }
    fn get_unique_id(&self) -> &String;
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
                "{} should have atleast 1 unique test",
                testable.get_object_string(),
            ),
            testable.get_problematic_path(false).map(str::to_owned),
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
        manifest::{Manifest, Node},
    };
    use dbt_artifact_parser::manifest::nodes::node::DependsOn;
    use dbt_artifact_parser::manifest::nodes::TestMetadata;

    struct MockTestable {
        unique_id: String,
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
    }
    impl Identifiable for MockTestable {
        fn get_object_type(&self) -> &str {
            &self.object_type
        }

        fn get_object_string(&self) -> &str {
            &self.object_string
        }

        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }
    impl TestAble for MockTestable {
        fn get_unique_id(&self) -> &String {
            &self.unique_id
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

    fn create_mock_source_test(name: &str, depends_on_parent: &str) -> Test {
        let mut test = Test::default();
        test.base.name = name.to_string();
        test.test_metadata = Some(TestMetadata {
            name: name.to_string(),
            kwargs: None,
            namespace: None,
        });
        test.attached_node = None;
        test.base.depends_on = DependsOn {
            nodes: Some(vec![depends_on_parent.to_string()]),
            macros: None,
        };
        test.base.unique_id = format!("test.source_{name}");
        test
    }

    #[test]
    fn test_has_unique_test_found() {
        let manifest = create_test_manifest(vec![create_mock_test("unique", "model.my_model")]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
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
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
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
        assert_eq!(rule_result.rule_name, "has_unique_test");
        assert_eq!(
            rule_result.message,
            "my_model should have atleast 1 unique test"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/my_model.sql".to_string())
        );
    }

    #[test]
    fn test_has_unique_test_with_not_allowed_name() {
        let manifest = create_test_manifest(vec![create_mock_test("not_unique", "model.my_model")]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
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
            "my_model should have atleast 1 unique test"
        );
    }

    #[test]
    fn test_custom_allowed_test_names() {
        let manifest = create_test_manifest(vec![create_mock_test(
            "custom_unique_test",
            "model.my_model",
        )]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: vec!["custom_unique_test".to_string()],
            },
            Severity::Error,
        );
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

    #[test]
    fn test_has_unique_test_found_via_depends_on_for_source() {
        let manifest = create_test_manifest(vec![create_mock_source_test(
            "unique",
            "source.my_project.raw.my_table",
        )]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "source.my_project.raw.my_table".to_string(),
            object_type: "source".to_string(),
            object_string: "my_table".to_string(),
            relative_path: Some("models/sources.yml".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_none(), "source with unique test should pass");
    }

    #[test]
    fn test_has_unique_test_not_found_for_source() {
        let manifest = create_test_manifest(vec![]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "source.my_project.raw.my_table".to_string(),
            object_type: "source".to_string(),
            object_string: "my_table".to_string(),
            relative_path: Some("models/sources.yml".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(result.is_some(), "source without unique test should fail");
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "source");
        assert_eq!(rule_result.rule_name, "has_unique_test");
    }

    #[test]
    fn test_source_test_with_wrong_name_not_counted() {
        let manifest = create_test_manifest(vec![create_mock_source_test(
            "not_null",
            "source.my_project.raw.my_table",
        )]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "source.my_project.raw.my_table".to_string(),
            object_type: "source".to_string(),
            object_string: "my_table".to_string(),
            relative_path: Some("models/sources.yml".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(
            result.is_some(),
            "source with non-unique test should still fail"
        );
    }

    #[test]
    fn test_source_test_not_matched_to_wrong_parent() {
        // A source test for table_a should not match when checking table_b
        let manifest = create_test_manifest(vec![create_mock_source_test(
            "unique",
            "source.my_project.raw.table_a",
        )]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: default_allowed_test_names(),
            },
            Severity::Error,
        );
        let allowed = vec!["unique".to_string()];
        let testable = MockTestable {
            unique_id: "source.my_project.raw.table_b".to_string(),
            object_type: "source".to_string(),
            object_string: "table_b".to_string(),
            relative_path: Some("models/sources.yml".to_string()),
        };
        let result = has_unique_test(&testable, &rule, &manifest, &allowed);
        assert!(
            result.is_some(),
            "source test for different parent should not match"
        );
    }
}
