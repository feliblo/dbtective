use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_refs() {
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
    "model.test.model_with_refs": {
      "database": "db",
      "schema": "public",
      "name": "model_with_refs",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model_with_refs.sql",
      "original_file_path": "models/model_with_refs.sql",
      "unique_id": "model.test.model_with_refs",
      "fqn": ["test", "model_with_refs"],
      "alias": "model_with_refs",
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with upstream references",
      "columns": {},
      "meta": {},
      "depends_on": {
        "nodes": ["model.test.base_model", "source.test.raw.customers"]
      }
    },
    "model.test.model_no_refs": {
      "database": "db",
      "schema": "public",
      "name": "model_no_refs",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model_no_refs.sql",
      "original_file_path": "models/model_no_refs.sql",
      "unique_id": "model.test.model_no_refs",
      "fqn": ["test", "model_no_refs"],
      "alias": "model_no_refs",
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Model without upstream references",
      "columns": {},
      "meta": {},
      "depends_on": {
        "nodes": []
      }
    },
    "model.test.base_model": {
      "database": "db",
      "schema": "public",
      "name": "base_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "base_model.sql",
      "original_file_path": "models/base_model.sql",
      "unique_id": "model.test.base_model",
      "fqn": ["test", "base_model"],
      "alias": "base_model",
      "checksum": {"name": "sha256", "checksum": "ghi"},
      "tags": [],
      "description": "Base model",
      "columns": {},
      "meta": {},
      "depends_on": {
        "nodes": ["source.test.raw.customers"]
      }
    }
  },
  "sources": {
    "source.test.raw.customers": {
      "database": "db",
      "schema": "raw",
      "name": "customers",
      "resource_type": "source",
      "package_name": "test_project",
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test.raw.customers",
      "fqn": ["test", "raw", "customers"],
      "source_name": "raw",
      "source_description": "",
      "loader": "",
      "identifier": "customers"
    }
  },
  "macros": {},
  "exposures": {},
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {
    "model.test.model_with_refs": ["model.test.base_model", "source.test.raw.customers"],
    "model.test.base_model": ["source.test.raw.customers"],
    "model.test.model_no_refs": []
  },
  "child_map": {
    "model.test.base_model": ["model.test.model_with_refs"],
    "source.test.raw.customers": ["model.test.model_with_refs", "model.test.base_model"],
    "model.test.model_no_refs": []
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    // Test case 1: Failure - model without refs should fail
    let has_refs_config = r#"
    manifest_tests:
      - name: "models_must_have_refs"
        type: has_refs
        severity: "error"
        applies_to:
          - "models"
    "#;

    let env = TestEnvironment::new(manifest, has_refs_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "models_must_have_refs");
    assert!(findings[0]
        .0
        .message
        .contains("does not have any upstream references"));
    assert!(findings[0].0.message.contains("model_no_refs"));

    // Test case 2: Success - all models have refs when we exclude the problematic one
    let has_refs_with_exclude_config = r#"
    manifest_tests:
      - name: "models_must_have_refs_exclude"
        type: has_refs
        severity: "error"
        applies_to:
          - "models"
        excludes:
          - "models/model_no_refs.sql"
    "#;

    let env = TestEnvironment::new(manifest, has_refs_with_exclude_config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(findings.len(), 0);

    // Test case 3: Warning severity
    let has_refs_warning_config = r#"
manifest_tests:
  - name: "models_should_have_refs"
    type: has_refs
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, has_refs_warning_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "models_should_have_refs");
}
