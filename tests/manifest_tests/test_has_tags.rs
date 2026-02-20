use crate::common::TestEnvironment;

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
    let findings = env.run_manifest_rules(false);

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
    let findings = env.run_manifest_rules(false);
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
    let findings = env.run_manifest_rules(false);
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

/// Ensures that `has_tags` rules do not produce findings for macros, semantic models,
/// or unit tests — even when those objects are present in the manifest alongside
/// models that do get checked. These object types implement `Tagable` (returning
/// `None` to comply with `includes_excludes`) but the rule application code correctly skips the `has_tags` rule for them.
#[test]
#[allow(clippy::too_many_lines)]
fn test_has_tags_does_not_flag_macros_semantic_models_unit_tests() {
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
    "model.test.tagged_model": {
      "database": "db",
      "schema": "public",
      "name": "tagged_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model.sql",
      "original_file_path": "models/model.sql",
      "unique_id": "model.test.tagged_model",
      "fqn": ["test", "tagged_model"],
      "alias": "tagged_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": ["important"],
      "description": "A model with the required tag",
      "columns": {},
      "meta": {}
    }
  },
  "sources": {},
  "macros": {
    "macro.test.my_macro": {
      "name": "my_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/my_macro.sql",
      "original_file_path": "macros/my_macro.sql",
      "unique_id": "macro.test.my_macro",
      "macro_sql": "{% macro my_macro() %}\n  SELECT 1\n{% endmacro %}",
      "depends_on": { "macros": [] },
      "description": "A macro without tags",
      "meta": {},
      "docs": { "show": true },
      "patch_path": null,
      "arguments": []
    }
  },
  "exposures": {},
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {},
  "child_map": {},
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {
    "semantic_model.test.my_semantic_model": {
      "name": "my_semantic_model",
      "package_name": "test_project",
      "original_file_path": "models/semantic/my_semantic_model.yml",
      "patch_path": null,
      "description": "A semantic model without tags",
      "metadata": null,
      "depends_on": { "nodes": [] }
    }
  },
  "unit_tests": {
    "unit_test.test.my_unit_test": {
      "name": "my_unit_test",
      "model": "tagged_model",
      "package_name": "test_project",
      "original_file_path": "models/unit_tests/my_unit_test.yml",
      "patch_path": null,
      "description": "A unit test without tags"
    }
  }
}"#;

    // has_tags rule targeting models — the model has the tag so no findings.
    // Crucially, the macro, semantic model, and unit test must NOT produce
    // findings even though they have no tags and the rule runs across all objects.
    let config = r#"
manifest_tests:
  - name: "need_important_tag"
    type: has_tags
    severity: "error"
    required_tags: ["important"]
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Only the model should be checked (and it has the tag), macros/semantic_models/unit_tests should be skipped"
    );

    // Now use a has_description rule that DOES apply to macros, semantic_models,
    // and unit_tests to prove they are present and reachable in the manifest.
    let description_config = r#"
manifest_tests:
  - name: "need_description"
    type: has_description
    severity: "warning"
    applies_to:
      - "macros"
      - "semantic_models"
      - "unit_tests"
"#;
    let env = TestEnvironment::new(manifest, description_config);
    let findings = env.run_manifest_rules(false);
    // All three have descriptions, so no findings — but this proves the objects are present
    assert_eq!(
        findings.len(),
        0,
        "All objects have descriptions, confirming they are present in the manifest"
    );

    // Remove the description from the unit test to confirm it IS reachable
    let manifest_no_desc = manifest.replace(
        r#""description": "A unit test without tags""#,
        r#""description": """#,
    );
    let env = TestEnvironment::new(&manifest_no_desc, description_config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Unit test with empty description should be flagged by has_description, proving it is reachable"
    );
    assert_eq!(findings[0].0.object_type, "UnitTest");
}
