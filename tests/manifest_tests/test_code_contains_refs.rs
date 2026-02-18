use crate::common::TestEnvironment;

#[test]
fn test_code_contains_refs_with_ref_passes() {
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
    "model.test.good_model": {
      "database": "db",
      "schema": "public",
      "name": "good_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "good_model.sql",
      "original_file_path": "models/good_model.sql",
      "unique_id": "model.test.good_model",
      "fqn": ["test", "good_model"],
      "alias": "good_model",
      "depends_on": { "nodes": ["model.test.stg_users"] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with ref",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id, name FROM {{ ref('stg_users') }}"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_contains_refs_with_source_passes() {
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
    "model.test.source_model": {
      "database": "db",
      "schema": "public",
      "name": "source_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "source_model.sql",
      "original_file_path": "models/source_model.sql",
      "unique_id": "model.test.source_model",
      "fqn": ["test", "source_model"],
      "alias": "source_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with source",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT * FROM {{ source('raw', 'users') }}"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_contains_refs_hardcoded_sql_fails() {
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
    "model.test.hardcoded_model": {
      "database": "db",
      "schema": "public",
      "name": "hardcoded_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "hardcoded_model.sql",
      "original_file_path": "models/hardcoded_model.sql",
      "unique_id": "model.test.hardcoded_model",
      "fqn": ["test", "hardcoded_model"],
      "alias": "hardcoded_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with hardcoded SQL",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id, name FROM raw_schema.users WHERE active = true"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "code_must_use_refs");
    assert!(findings[0].0.message.contains("hardcoded_model"));
    assert!(findings[0]
        .0
        .message
        .contains("does not contain any ref() or source()"));
}

#[test]
fn test_code_contains_refs_commented_ref_fails() {
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
    "model.test.commented_model": {
      "database": "db",
      "schema": "public",
      "name": "commented_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "commented_model.sql",
      "original_file_path": "models/commented_model.sql",
      "unique_id": "model.test.commented_model",
      "fqn": ["test", "commented_model"],
      "alias": "commented_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with commented-out ref",
      "columns": {},
      "meta": {},
      "raw_code": "-- SELECT * FROM {{ ref('stg_users') }}\nSELECT id FROM raw_schema.users"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should fail because the ref is commented out
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert!(findings[0].0.message.contains("commented_model"));
}

#[test]
fn test_code_contains_refs_multiline_comment_fails() {
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
    "model.test.block_commented_model": {
      "database": "db",
      "schema": "public",
      "name": "block_commented_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "block_commented_model.sql",
      "original_file_path": "models/block_commented_model.sql",
      "unique_id": "model.test.block_commented_model",
      "fqn": ["test", "block_commented_model"],
      "alias": "block_commented_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with block-commented ref",
      "columns": {},
      "meta": {},
      "raw_code": "/* {{ ref('old_model') }} */\nSELECT id FROM raw_schema.users"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("block_commented_model"));
}

#[test]
fn test_code_contains_refs_mixed_models() {
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
    "model.test.good_model": {
      "database": "db",
      "schema": "public",
      "name": "good_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "good_model.sql",
      "original_file_path": "models/good_model.sql",
      "unique_id": "model.test.good_model",
      "fqn": ["test", "good_model"],
      "alias": "good_model",
      "depends_on": { "nodes": ["model.test.stg_users"] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Good model with ref",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT * FROM {{ ref('stg_users') }}"
    },
    "model.test.bad_model": {
      "database": "db",
      "schema": "public",
      "name": "bad_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "bad_model.sql",
      "original_file_path": "models/bad_model.sql",
      "unique_id": "model.test.bad_model",
      "fqn": ["test", "bad_model"],
      "alias": "bad_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Bad model without ref",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM raw_schema.users"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Only bad_model should fail
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("bad_model"));
}

#[test]
fn test_code_contains_refs_case_insensitive() {
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
    "model.test.uppercase_ref_model": {
      "database": "db",
      "schema": "public",
      "name": "uppercase_ref_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "uppercase_ref_model.sql",
      "original_file_path": "models/uppercase_ref_model.sql",
      "unique_id": "model.test.uppercase_ref_model",
      "fqn": ["test", "uppercase_ref_model"],
      "alias": "uppercase_ref_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with uppercase REF",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT * FROM {{ REF('stg_users') }}"
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

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should pass — case-insensitive detection
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_contains_refs_macros() {
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
  "nodes": {},
  "sources": {},
  "macros": {
    "macro.test.good_macro": {
      "name": "good_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/good_macro.sql",
      "original_file_path": "macros/good_macro.sql",
      "unique_id": "macro.test.good_macro",
      "macro_sql": "{% macro good_macro() %}\n  SELECT * FROM {{ ref('users') }}\n{% endmacro %}",
      "depends_on": { "macros": [] },
      "description": "Good macro with ref",
      "meta": {},
      "docs": { "show": true },
      "patch_path": null,
      "arguments": []
    },
    "macro.test.bad_macro": {
      "name": "bad_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/bad_macro.sql",
      "original_file_path": "macros/bad_macro.sql",
      "unique_id": "macro.test.bad_macro",
      "macro_sql": "{% macro bad_macro() %}\n  SELECT * FROM raw_schema.users\n{% endmacro %}",
      "depends_on": { "macros": [] },
      "description": "Bad macro without ref",
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
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "code_must_use_refs"
    type: code_contains_refs
    severity: "warning"
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Only bad_macro should fail
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Macro");
    assert!(findings[0].0.message.contains("bad_macro"));
}
