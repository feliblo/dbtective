use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::RequiredTest, manifest_rule::ManifestRule},
        manifest::Manifest,
    },
};

use super::has_unique_test::TestAble;

/// Check that a testable object has at least one test for each required test entry.
/// Returns one `RuleResult` per missing required test.
pub fn has_required_tests<T: TestAble>(
    testable: &T,
    rule: &ManifestRule,
    manifest: &Manifest,
    required_tests: &[RequiredTest],
) -> Vec<RuleResult> {
    let tests = testable.get_tests(manifest);

    required_tests
        .iter()
        .filter(|req| {
            let allowed = req.allowed_names();
            !tests.iter().any(|test| {
                test.get_metadata_name()
                    .is_some_and(|name| allowed.iter().any(|a| *a == name))
            })
        })
        .map(|req| {
            RuleResult::new(
                &rule.severity,
                testable.get_object_type(),
                rule.get_name(),
                format!(
                    "{} is missing a required test: {}",
                    testable.get_object_string(),
                    req.display_name(),
                ),
                testable.get_problematic_path(false).map(str::to_owned),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::{
            check_config_options::RequiredTest, manifest_rule::ManifestSpecificRuleConfig,
            severity::Severity,
        },
        manifest::{Manifest, Node},
        rules::common_traits::Identifiable,
    };
    use dbt_artifact_parser::manifest::nodes::node::DependsOn;
    use dbt_artifact_parser::manifest::nodes::{Test, TestMetadata};

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
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
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

    fn make_rule(required_tests: Vec<RequiredTest>) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasRequiredTests { required_tests },
            Severity::Error,
        )
    }

    fn make_testable(unique_id: &str) -> MockTestable {
        MockTestable {
            unique_id: unique_id.to_string(),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        }
    }

    // ---- Tests ----

    #[test]
    fn test_all_required_tests_present() {
        let manifest = create_test_manifest(vec![
            create_mock_test("unique", "model.my_model"),
            create_mock_test("not_null", "model.my_model"),
        ]);
        let required = vec![
            RequiredTest::Simple("unique".to_string()),
            RequiredTest::Simple("not_null".to_string()),
        ];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert!(results.is_empty());
    }

    #[test]
    fn test_missing_one_required_test() {
        let manifest = create_test_manifest(vec![create_mock_test("unique", "model.my_model")]);
        let required = vec![
            RequiredTest::Simple("unique".to_string()),
            RequiredTest::Simple("not_null".to_string()),
        ];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("not_null"));
    }

    #[test]
    fn test_missing_all_required_tests() {
        let manifest = create_test_manifest(vec![]);
        let required = vec![
            RequiredTest::Simple("unique".to_string()),
            RequiredTest::Simple("not_null".to_string()),
        ];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_with_alternatives_satisfied() {
        let manifest = create_test_manifest(vec![create_mock_test(
            "dbt_utils.unique_combination_of_columns",
            "model.my_model",
        )]);
        let required = vec![RequiredTest::WithAlternatives {
            name: "uniqueness".to_string(),
            allowed_names: vec![
                "unique".to_string(),
                "dbt_utils.unique_combination_of_columns".to_string(),
            ],
        }];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert!(results.is_empty());
    }

    #[test]
    fn test_with_alternatives_not_satisfied() {
        let manifest =
            create_test_manifest(vec![create_mock_test("accepted_values", "model.my_model")]);
        let required = vec![RequiredTest::WithAlternatives {
            name: "uniqueness".to_string(),
            allowed_names: vec![
                "unique".to_string(),
                "dbt_utils.unique_combination_of_columns".to_string(),
            ],
        }];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("uniqueness"));
    }

    #[test]
    fn test_source_required_tests() {
        let manifest = create_test_manifest(vec![create_mock_source_test(
            "not_null",
            "source.my_project.raw.my_table",
        )]);
        let required = vec![
            RequiredTest::Simple("not_null".to_string()),
            RequiredTest::Simple("unique".to_string()),
        ];
        let rule = make_rule(required.clone());
        let testable = MockTestable {
            unique_id: "source.my_project.raw.my_table".to_string(),
            object_type: "source".to_string(),
            object_string: "my_table".to_string(),
            relative_path: Some("models/sources.yml".to_string()),
        };

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("unique"));
    }

    #[test]
    fn test_empty_required_tests_always_passes() {
        let manifest = create_test_manifest(vec![]);
        let required: Vec<RequiredTest> = vec![];
        let rule = make_rule(required.clone());
        let testable = make_testable("model.my_model");

        let results = has_required_tests(&testable, &rule, &manifest, &required);
        assert!(results.is_empty());
    }
}
