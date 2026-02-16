use crate::common::TestEnvironment;

const MANIFEST_METADATA: &str = r#"
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
  }"#;

const MANIFEST_EMPTY_SECTIONS: &str = r#"
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
  "unit_tests": {}"#;

const CATALOG_METADATA: &str = r#"
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  }"#;

fn node_json(columns_json: &str) -> String {
    format!(
        r#"{{
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
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "tags": [],
      "config": {{
        "enabled": true,
        "materialized": "view",
        "tags": []
      }},
      "description": "Customer table",
      "columns": {{{columns_json}}},
      "meta": {{}},
      "group": null,
      "docs": {{"show": true}},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {{}},
      "created_at": 1704067200.0,
      "config_call_dict": {{}},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from source_customers",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": []}},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {{"enforced": false, "checksum": null}}
    }}"#
    )
}

fn manifest_json(columns_json: &str) -> String {
    format!(
        r#"{{
{MANIFEST_METADATA},
  "nodes": {{
    "model.test_project.customers": {node}
  }},
{MANIFEST_EMPTY_SECTIONS}
}}"#,
        node = node_json(columns_json)
    )
}

fn catalog_json(columns_json: &str) -> String {
    format!(
        r#"{{
{CATALOG_METADATA},
  "nodes": {{
    "model.test_project.customers": {{
      "unique_id": "model.test_project.customers",
      "metadata": {{
        "type": "BASE TABLE",
        "schema": "public",
        "name": "customers",
        "database": "analytics"
      }},
      "columns": {{{columns_json}}},
      "stats": {{}}
    }}
  }},
  "sources": {{}}
}}"#
    )
}

#[test]
fn test_all_columns_have_data_types_pass() {
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "data_type": "varchar", "tags": []},
        "email": {"name": "email", "description": "Customer email", "data_type": "varchar", "tags": []}
        "#,
    );
    let catalog = catalog_json(
        r#"
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_some_columns_missing_data_types_fail() {
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []}
        "#,
    );
    let catalog = catalog_json(
        r#"
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0]
        .0
        .message
        .contains("do not have data types defined"));
}

#[test]
fn test_percentage_threshold_pass() {
    // 2 out of 4 columns have types = 50%, threshold is 50%
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "data_type": "varchar", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []},
        "phone": {"name": "phone", "description": "Phone number", "tags": []}
        "#,
    );
    let catalog = catalog_json(
        r#"
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3},
        "phone": {"type": "VARCHAR", "name": "phone", "index": 4}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    min_coverage: 50
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings (50% meets 50% threshold), but got: {findings:?}"
    );
}

#[test]
fn test_percentage_threshold_fail() {
    // 1 out of 4 columns has types = 25%, threshold is 90%
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []},
        "phone": {"name": "phone", "description": "Phone number", "tags": []}
        "#,
    );
    let catalog = catalog_json(
        r#"
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3},
        "phone": {"type": "VARCHAR", "name": "phone", "index": 4}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    min_coverage: 90
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("25%"));
    assert!(findings[0].0.message.contains("90%"));
}

#[test]
fn test_default_min_coverage_is_100() {
    // Config without min_coverage should default to 100 (all)
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []}
        "#,
    );
    let catalog = catalog_json(
        r#"
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // name is missing data_type, so with default 100% it should fail
    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding (default 100% coverage), but got: {findings:?}"
    );
}

#[test]
fn test_manifest_fallback_columns_have_data_type_pass() {
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "data_type": "varchar", "tags": []}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings in fallback mode, but got: {findings:?}"
    );
}

#[test]
fn test_manifest_fallback_columns_have_data_type_fail() {
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding in fallback mode, but got: {findings:?}"
    );
    assert!(findings[0]
        .0
        .message
        .contains("do not have data types defined"));
}

#[test]
fn test_manifest_fallback_percentage_threshold() {
    // 1 out of 3 columns has types = 33%, threshold is 30%
    let manifest = manifest_json(
        r#"
        "id": {"name": "id", "description": "Customer ID", "data_type": "integer", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []}
        "#,
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    min_coverage: 30
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings (33% meets 30% threshold), but got: {findings:?}"
    );
}
