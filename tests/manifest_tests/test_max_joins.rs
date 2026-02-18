use crate::common::TestEnvironment;

#[test]
fn test_max_joins_within_limit_passes() {
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
    "model.test.simple_model": {
      "database": "db",
      "schema": "public",
      "name": "simple_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "simple_model.sql",
      "original_file_path": "models/simple_model.sql",
      "unique_id": "model.test.simple_model",
      "fqn": ["test", "simple_model"],
      "alias": "simple_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Simple model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id, b.name FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id"
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
  - name: "max_joins_check"
    type: max_joins
    severity: "error"
    max_joins: 2
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_max_joins_exceeds_limit_fails() {
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
    "model.test.complex_model": {
      "database": "db",
      "schema": "public",
      "name": "complex_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "complex_model.sql",
      "original_file_path": "models/complex_model.sql",
      "unique_id": "model.test.complex_model",
      "fqn": ["test", "complex_model"],
      "alias": "complex_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Complex model with many joins",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id JOIN {{ ref('products') }} c ON b.product_id = c.id JOIN {{ ref('categories') }} d ON c.cat_id = d.id"
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
  - name: "max_joins_check"
    type: max_joins
    severity: "error"
    max_joins: 2
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "max_joins_check");
    assert!(findings[0].0.message.contains("complex_model"));
    assert!(findings[0].0.message.contains("3 JOIN(s)"));
    assert!(findings[0].0.message.contains("maximum allowed of 2"));
}

#[test]
fn test_max_joins_commented_joins_not_counted() {
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
      "description": "Model with commented out joins",
      "columns": {},
      "meta": {},
      "raw_code": "-- JOIN old_table ON ...\n/* LEFT JOIN another_table ON ... */\nSELECT a.id FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id"
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
  - name: "max_joins_check"
    type: max_joins
    severity: "error"
    max_joins: 1
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // Only 1 real JOIN (the commented ones should not count)
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_max_joins_case_insensitive() {
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
    "model.test.mixed_case_model": {
      "database": "db",
      "schema": "public",
      "name": "mixed_case_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "mixed_case_model.sql",
      "original_file_path": "models/mixed_case_model.sql",
      "unique_id": "model.test.mixed_case_model",
      "fqn": ["test", "mixed_case_model"],
      "alias": "mixed_case_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with mixed case joins",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM {{ ref('users') }} a join {{ ref('orders') }} b ON a.id = b.user_id LEFT JOIN {{ ref('products') }} c ON b.product_id = c.id"
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
  - name: "max_joins_check"
    type: max_joins
    severity: "warning"
    max_joins: 1
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert!(findings[0].0.message.contains("2 JOIN(s)"));
}

#[test]
fn test_max_joins_mixed_models() {
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
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Good model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id"
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
      "description": "Bad model with too many joins",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id JOIN products c ON b.product_id = c.id JOIN categories d ON c.cat_id = d.id JOIN suppliers e ON d.sup_id = e.id"
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
  - name: "max_joins_check"
    type: max_joins
    severity: "error"
    max_joins: 2
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // Only bad_model should fail (4 joins > 2 limit)
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("bad_model"));
}

#[test]
fn test_max_joins_default_threshold() {
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
    "model.test.ok_model": {
      "database": "db",
      "schema": "public",
      "name": "ok_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "ok_model.sql",
      "original_file_path": "models/ok_model.sql",
      "unique_id": "model.test.ok_model",
      "fqn": ["test", "ok_model"],
      "alias": "ok_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with 6 joins (should fail with default of 5)",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM t1 a JOIN t2 b ON a.id=b.id JOIN t3 c ON b.id=c.id JOIN t4 d ON c.id=d.id JOIN t5 e ON d.id=e.id JOIN t6 f ON e.id=f.id JOIN t7 g ON f.id=g.id"
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

    // No max_joins specified — should use default of 5
    let config = r#"
manifest_tests:
  - name: "max_joins_default"
    type: max_joins
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // 6 joins > default 5, should fail
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("ok_model"));
}

#[test]
fn test_max_joins_no_code_skips() {
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
    "model.test.no_code_model": {
      "database": "db",
      "schema": "public",
      "name": "no_code_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "no_code_model.sql",
      "original_file_path": "models/no_code_model.sql",
      "unique_id": "model.test.no_code_model",
      "fqn": ["test", "no_code_model"],
      "alias": "no_code_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model without code",
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

    let config = r#"
manifest_tests:
  - name: "max_joins_check"
    type: max_joins
    severity: "error"
    max_joins: 1
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // No code = skip gracefully
    assert_eq!(findings.len(), 0);
}
