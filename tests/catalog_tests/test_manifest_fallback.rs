use crate::common::TestEnvironment;

/// Shared manifest fixture with a model that has columns with mixed descriptions.
///
/// These are the things that happen in this manifest:
/// - id: has description
/// - name: has description
/// - email: EMPTY description (should trigger `columns_have_description`)
/// - `FirstName`: violates `snake_case` (should trigger `columns_name_convention`)
/// - `cust_id`: invalid name for canonical `customer_id` (should trigger `columns_canonical_name`)
const MANIFEST_WITH_ISSUES: &str = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics",
      "schema": "public",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "customers.sql",
      "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Customer table",
      "columns": {
        "id": {"name": "id", "description": "Customer ID", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "", "tags": []},
        "FirstName": {"name": "FirstName", "description": "First name", "tags": []},
        "cust_id": {"name": "cust_id", "description": "Customer identifier", "tags": []}
      },
      "meta": {},
      "group": null,
      "docs": {"show": true},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {},
      "created_at": 1704067200.0,
      "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from source_customers",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
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

/// Manifest with a model where all columns have descriptions and follow `snake_case`.
const MANIFEST_ALL_PASS: &str = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics",
      "schema": "public",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "customers.sql",
      "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Customer table",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Customer ID", "tags": []},
        "full_name": {"name": "full_name", "description": "Customer name", "tags": []},
        "email_address": {"name": "email_address", "description": "Email", "tags": []}
      },
      "meta": {},
      "group": null,
      "docs": {"show": true},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {},
      "created_at": 1704067200.0,
      "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from source_customers",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
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

/// Manifest with sources that have column issues.
const MANIFEST_WITH_SOURCES: &str = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }
  },
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.users": {
      "database": "raw",
      "schema": "raw_data",
      "name": "users",
      "source_name": "raw_data",
      "source_description": "Raw data",
      "loader": "",
      "identifier": "users",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [],
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw_data.users",
      "fqn": ["test_project", "raw_data", "users"],
      "config": {"enabled": true},
      "description": "Raw user data",
      "columns": {
        "user_id": {"name": "user_id", "description": "User ID", "tags": []},
        "username": {"name": "username", "description": "", "tags": []},
        "CreatedAt": {"name": "CreatedAt", "description": "Creation timestamp", "tags": []}
      }
    }
  },
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

/// Manifest with both models and seeds for `applies_to` filtering tests.
const MANIFEST_WITH_MODEL_AND_SEED: &str = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }
  },
  "nodes": {
    "model.test_project.orders": {
      "database": "analytics",
      "schema": "public",
      "name": "orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "orders.sql",
      "original_file_path": "models/orders.sql",
      "unique_id": "model.test_project.orders",
      "fqn": ["test_project", "orders"],
      "alias": "orders",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Orders table",
      "columns": {
        "order_id": {"name": "order_id", "description": "", "tags": []}
      },
      "meta": {},
      "group": null,
      "docs": {"show": true},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {},
      "created_at": 1704067200.0,
      "config_call_dict": {},
      "relation_name": "analytics.public.orders",
      "raw_code": "select * from source_orders",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    },
    "seed.test_project.raw_data": {
      "database": "analytics",
      "schema": "public",
      "name": "raw_data",
      "resource_type": "seed",
      "package_name": "test_project",
      "path": "raw_data.csv",
      "original_file_path": "seeds/raw_data.csv",
      "unique_id": "seed.test_project.raw_data",
      "fqn": ["test_project", "raw_data"],
      "alias": "raw_data",
      "checksum": {"name": "sha256", "checksum": "def456"},
      "config": {"enabled": true},
      "tags": [],
      "description": "Raw seed data",
      "columns": {
        "value": {"name": "value", "description": "", "tags": []}
      },
      "meta": {},
      "group": null,
      "docs": {"show": true},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {},
      "created_at": 1704067200.0,
      "config_call_dict": {},
      "relation_name": "analytics.public.raw_data",
      "raw_code": "",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
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

/// Email column is missing a description
#[test]
fn test_fallback_columns_have_description_finds_missing() {
    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0].0.message.contains("email"));
}

#[test]
fn test_fallback_columns_have_description_passes() {
    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_ALL_PASS, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 0, "Expected no findings, got: {findings:?}");
}

/// `FirstName` is not `snake_case` in the manifest
#[test]
fn test_fallback_columns_name_convention_finds_violations() {
    let config = r#"
catalog_tests:
  - name: "columns_snake_case"
    type: "columns_name_convention"
    pattern: "snake_case"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.severity, "FAIL");
    assert!(findings[0].0.message.contains("FirstName"));
    assert!(findings[0].0.message.contains("snake_case"));
}

#[test]
fn test_fallback_columns_name_convention_passes() {
    let config = r#"
catalog_tests:
  - name: "columns_snake_case"
    type: "columns_name_convention"
    pattern: "snake_case"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_ALL_PASS, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 0, "Expected no findings, got: {findings:?}");
}

/// `cust_id` does not follow canonical of `customer_id`
#[test]
fn test_fallback_columns_canonical_name_finds_violations() {
    let config = r#"
catalog_tests:
  - name: "canonical_customer_id"
    type: "columns_canonical_name"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.severity, "FAIL");
    assert!(findings[0].0.message.contains("cust_id"));
    assert!(findings[0].0.message.contains("customer_id"));
}

#[test]
fn test_fallback_columns_canonical_name_passes() {
    let config = r#"
catalog_tests:
  - name: "canonical_customer_id"
    type: "columns_canonical_name"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_ALL_PASS, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 0, "Expected no findings, got: {findings:?}");
}

/// `columns_all_documented` needs a database connection
/// It compares whats in the DB with what is documented. It should never run.
#[test]
fn test_fallback_columns_all_documented_is_skipped() {
    let config = r#"
catalog_tests:
  - name: "columns_all_documented"
    type: "columns_all_documented"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings for skipped rule, got: {findings:?}"
    );
}

#[test]
fn test_fallback_mixed_rules_only_eligible_run() {
    let config = r#"
catalog_tests:
  - name: "columns_all_documented"
    type: "columns_all_documented"
    severity: "error"
    applies_to:
      - "models"
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
  - name: "columns_snake_case"
    type: "columns_name_convention"
    pattern: "snake_case"
    severity: "warning"
    applies_to:
      - "models"
  - name: "canonical_customer_id"
    type: "columns_canonical_name"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    // Should have findings from 3 eligible rules, NOT from columns_all_documented
    assert_eq!(findings.len(), 3, "Expected 3 findings, got: {findings:?}");

    // Verify no columns_all_documented findings
    for (result, _) in &findings {
        assert!(
            !result.rule_name.contains("columns_all_documented"),
            "columns_all_documented should be skipped in fallback mode"
        );
    }
}

#[test]
fn test_fallback_applies_to_filtering() {
    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    // Both model and seed have empty descriptions, but only models should be checked
    let env = TestEnvironment::new(MANIFEST_WITH_MODEL_AND_SEED, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("orders"));
}

#[test]
fn test_fallback_no_catalog_tests_configured() {
    let config = r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "warning"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings when no catalog_tests configured, got: {findings:?}"
    );
}

#[test]
fn test_fallback_source_columns_have_description() {
    let config = r#"
catalog_tests:
  - name: "source_columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_SOURCES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("users"));
    assert!(findings[0].0.message.contains("username"));
}

#[test]
fn test_fallback_source_columns_name_convention() {
    let config = r#"
catalog_tests:
  - name: "source_columns_snake_case"
    type: "columns_name_convention"
    pattern: "snake_case"
    severity: "warning"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_SOURCES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1, "Expected 1 finding, got: {findings:?}");
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("CreatedAt"));
}

#[test]
fn test_fallback_rule_name_matches_configured_name() {
    let config = r#"
catalog_tests:
  - name: "my_custom_rule_name"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(MANIFEST_WITH_ISSUES, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.rule_name, "my_custom_rule_name");
}
