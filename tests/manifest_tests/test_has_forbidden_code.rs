use crate::common::TestEnvironment;

#[test]
fn test_has_forbidden_code_models() {
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
    "model.test.clean_model": {
      "database": "db",
      "schema": "public",
      "name": "clean_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "clean_model.sql",
      "original_file_path": "models/clean_model.sql",
      "unique_id": "model.test.clean_model",
      "fqn": ["test", "clean_model"],
      "alias": "clean_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Clean model without jinja",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id, name\nFROM users\nWHERE active = true"
    },
    "model.test.jinja_model": {
      "database": "db",
      "schema": "public",
      "name": "jinja_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "jinja_model.sql",
      "original_file_path": "models/jinja_model.sql",
      "unique_id": "model.test.jinja_model",
      "fqn": ["test", "jinja_model"],
      "alias": "jinja_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Model with jinja patterns",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT {{ ref('other_model') }}\nFROM {% if target.name == 'prod' %}prod_table{% endif %}"
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

    // Config: forbid {{ and {% in models
    let config = r#"
manifest_tests:
  - name: "no_jinja_in_models"
    type: has_forbidden_code
    severity: "error"
    forbidden_patterns: ["{{", "{%"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // Only jinja_model should have violations
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "no_jinja_in_models");
    assert!(findings[0].0.message.contains("jinja_model"));
    assert!(findings[0].0.message.contains("forbidden code pattern"));
}

#[test]
fn test_has_forbidden_code_no_violations() {
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
    "model.test.pure_sql_model": {
      "database": "db",
      "schema": "public",
      "name": "pure_sql_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "pure_sql_model.sql",
      "original_file_path": "models/pure_sql_model.sql",
      "unique_id": "model.test.pure_sql_model",
      "fqn": ["test", "pure_sql_model"],
      "alias": "pure_sql_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Pure SQL model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id, name FROM users"
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
  - name: "no_jinja"
    type: has_forbidden_code
    severity: "error"
    forbidden_patterns: ["{{", "{%"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0);
}

#[test]
fn test_has_forbidden_code_macros() {
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
    "macro.test.bad_macro": {
      "name": "bad_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/bad_macro.sql",
      "original_file_path": "macros/bad_macro.sql",
      "unique_id": "macro.test.bad_macro",
      "macro_sql": "{% macro bad_macro() %}\n  SELECT * FROM {{ source('raw', 'users') }}\n{% endmacro %}",
      "depends_on": { "macros": [] },
      "description": "Bad macro",
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
  - name: "no_select_star_in_macros"
    type: has_forbidden_code
    severity: "warning"
    forbidden_patterns: ["SELECT *"]
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Macro");
    assert_eq!(findings[0].0.rule_name, "no_select_star_in_macros");
    assert!(findings[0].0.message.contains("bad_macro"));
    assert!(findings[0].0.message.contains("'SELECT *'"));
}

#[test]
fn test_has_forbidden_code_case_insensitive_default() {
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
    "model.test.lower_model": {
      "database": "db",
      "schema": "public",
      "name": "lower_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "lower_model.sql",
      "original_file_path": "models/lower_model.sql",
      "unique_id": "model.test.lower_model",
      "fqn": ["test", "lower_model"],
      "alias": "lower_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with lowercase select star",
      "columns": {},
      "meta": {},
      "raw_code": "select * from users"
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

    // Default (no case_sensitive) should be case-insensitive
    let config = r#"
manifest_tests:
  - name: "no_select_star"
    type: has_forbidden_code
    severity: "error"
    forbidden_patterns: ["SELECT *"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // "select *" should match "SELECT *" case-insensitively
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.rule_name, "no_select_star");
    assert!(findings[0].0.message.contains("lower_model"));
}

#[test]
fn test_has_forbidden_code_case_sensitive_no_match() {
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
    "model.test.lower_model": {
      "database": "db",
      "schema": "public",
      "name": "lower_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "lower_model.sql",
      "original_file_path": "models/lower_model.sql",
      "unique_id": "model.test.lower_model",
      "fqn": ["test", "lower_model"],
      "alias": "lower_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with lowercase select star",
      "columns": {},
      "meta": {},
      "raw_code": "select * from users"
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

    // case_sensitive: true means "SELECT *" should NOT match "select *"
    let config = r#"
manifest_tests:
  - name: "no_select_star_exact"
    type: has_forbidden_code
    severity: "error"
    forbidden_patterns: ["SELECT *"]
    case_sensitive: true
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0);
}

#[test]
fn test_has_forbidden_code_case_sensitive_exact_match() {
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
    "model.test.upper_model": {
      "database": "db",
      "schema": "public",
      "name": "upper_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "upper_model.sql",
      "original_file_path": "models/upper_model.sql",
      "unique_id": "model.test.upper_model",
      "fqn": ["test", "upper_model"],
      "alias": "upper_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with uppercase SELECT star",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT * FROM users"
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

    // case_sensitive: true, exact case matches
    let config = r#"
manifest_tests:
  - name: "no_select_star_exact"
    type: has_forbidden_code
    severity: "error"
    forbidden_patterns: ["SELECT *"]
    case_sensitive: true
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.rule_name, "no_select_star_exact");
    assert!(findings[0].0.message.contains("upper_model"));
}
