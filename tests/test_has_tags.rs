mod common;

use common::TestEnvironment;
#[test]
#[allow(clippy::too_many_lines)]
fn test_has_tags() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {}
  },
  "nodes": {
    "model.test.model_with_tags": {
      "database": "db",
      "schema": "public",
      "name": "model_with_tags",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model.sql",
      "original_file_path": "models/model.sql",
      "unique_id": "model.test.model_with_tags",
      "fqn": ["test", "model_with_tags"],
      "alias": "model_with_tags",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": ["tag1", "tag2"],
      "description": "Has tags",
      "columns": {},
      "meta": {}
    },
    "model.test.model_no_tags": {
      "database": "db",
      "schema": "public",
      "package_name": "test_project",
      "name": "model_no_tags",
      "resource_type": "model",
      "path": "model2.sql",
      "original_file_path": "models/model2.sql",
      "unique_id": "model.test.model_no_tags",
      "fqn": ["test", "model_no_tags"],
      "alias": "model_no_tags",
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": ["tag3"],
      "depends_on": {
        "nodes": []
      },
      "description": "No tags",
      "columns": {},
      "meta": {}
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {},
  "child_map": {},
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let tag_available_config = r#"
manifest_tests:
  - name: "models_need_tag1"
    type: has_tags
    severity: "error"
    required_tags: ["tag1"]
    applies_to:
      - "models"
"#;

    // Failure: 2nd model is missing tag1
    let env = TestEnvironment::new(manifest, tag_available_config);
    let findings = env.run_checks(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "models_need_tag1");
    assert!(findings[0].0.message.contains("model_no_tags"));

    // Success: both models have at least one of the required tags
    let tag_any_config = r#"
manifest_tests:
  - name: "models_need_tag1_or_tag3"
    type: has_tags
    severity: "error"
    required_tags: ["tag1", "tag3"]
    criteria: "any"
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(manifest, tag_any_config);
    let findings = env.run_checks(false);
    assert_eq!(findings.len(), 0);

    // OneOf: Failure, model one has both tags, model two has only one of the required tags
    // 2 fails because neither has "only one" of the required tags
    let tag_one_of_config = r#"
manifest_tests:
  - name: "models_need_tag1_and_tag2"
    type: has_tags
    severity: "warning"
    required_tags: ["tag1", "tag2"]
    criteria: "one_of"
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(manifest, tag_one_of_config);
    let findings = env.run_checks(false);
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "models_need_tag1_and_tag2");
    assert_eq!(findings[1].0.severity, "WARN");
    assert_eq!(findings[1].0.object_type, "Model");
    assert_eq!(findings[1].0.rule_name, "models_need_tag1_and_tag2");
    // Check both model names appear (order not guaranteed)
    let has_model_with_tags = findings
        .iter()
        .any(|f| f.0.message.contains("model_with_tags"));
    let has_model_no_tags = findings
        .iter()
        .any(|f| f.0.message.contains("model_no_tags"));
    assert!(has_model_with_tags);
    assert!(has_model_no_tags);
}
