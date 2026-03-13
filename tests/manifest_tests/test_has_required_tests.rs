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

fn model_node(unique_id: &str, name: &str) -> String {
    format!(
        r#""{unique_id}": {{
      "database": "analytics",
      "schema": "staging",
      "name": "{name}",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/{name}.sql",
      "original_file_path": "models/staging/{name}.sql",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "staging", "{name}"],
      "alias": "{name}",
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "config": {{"enabled": true, "materialized": "view", "tags": []}},
      "tags": [],
      "description": "Test model",
      "columns": {{"id": {{"name": "id", "description": "PK", "tags": []}}}},
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
      "relation_name": "analytics.staging.{name}",
      "raw_code": "select * from raw.{name}",
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

fn test_node(
    test_unique_id: &str,
    test_name: &str,
    attached_node: &str,
    namespace: Option<&str>,
) -> String {
    let namespace_field = namespace
        .map(|ns| format!(r#", "namespace": "{ns}""#))
        .unwrap_or_default();
    format!(
        r#""{test_unique_id}": {{
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "{test_name}",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "{test_name}.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "{test_unique_id}",
      "fqn": ["test_project", "staging", "{test_name}"],
      "alias": "{test_name}",
      "checksum": {{"name": "sha256", "checksum": "def456"}},
      "config": {{"enabled": true}},
      "tags": [],
      "description": "",
      "columns": {{}},
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
      "relation_name": null,
      "raw_code": "",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": ["{attached_node}"]}},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {{"enforced": false, "checksum": null}},
      "test_metadata": {{
        "name": "{test_name}"{namespace_field}
      }},
      "attached_node": "{attached_node}"
    }}"#
    )
}

fn source_node(unique_id: &str, name: &str, source_name: &str) -> String {
    format!(
        r#""{unique_id}": {{
      "database": "raw",
      "schema": "{source_name}",
      "name": "{name}",
      "source_name": "{source_name}",
      "source_description": "Raw data",
      "loader": "",
      "identifier": "{name}",
      "resource_type": "source",
      "package_name": "test_project",
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "{source_name}", "{name}"],
      "source_meta": {{}},
      "tags": [],
      "config": {{"enabled": true}},
      "patch_path": null,
      "unrendered_config": {{}},
      "relation_name": "raw.{source_name}.{name}",
      "created_at": 1704067200.0,
      "description": "Test source",
      "columns": {{}},
      "meta": {{}},
      "freshness": {{
        "warn_after": {{"count": null, "period": null}},
        "error_after": {{"count": null, "period": null}},
        "filter": null
      }},
      "quoting": {{
        "database": null,
        "schema": null,
        "identifier": null,
        "column": null
      }},
      "loaded_at_field": null,
      "external": null
    }}"#
    )
}

fn source_test_node(test_unique_id: &str, test_name: &str, depends_on_source: &str) -> String {
    format!(
        r#""{test_unique_id}": {{
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "{test_name}",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "{test_name}.sql",
      "original_file_path": "models/sources.yml",
      "unique_id": "{test_unique_id}",
      "fqn": ["test_project", "{test_name}"],
      "alias": "{test_name}",
      "checksum": {{"name": "sha256", "checksum": "def456"}},
      "config": {{"enabled": true}},
      "tags": [],
      "description": "",
      "columns": {{}},
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
      "relation_name": null,
      "raw_code": "",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": ["{depends_on_source}"]}},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {{"enforced": false, "checksum": null}},
      "test_metadata": {{
        "name": "{test_name}"
      }}
    }}"#
    )
}

fn build_manifest(nodes: &[String]) -> String {
    build_manifest_with_sources(nodes, &[])
}

fn build_manifest_with_sources(nodes: &[String], sources: &[String]) -> String {
    format!(
        r#"{{
{MANIFEST_METADATA},
  "nodes": {{
    {}
  }},
  "sources": {{
    {}
  }},
  "macros": {{}},
  "exposures": {{}},
  "metrics": {{}},
  "groups": {{}},
  "selectors": {{}},
  "disabled": {{}},
  "parent_map": {{}},
  "child_map": {{}},
  "group_map": {{}},
  "saved_queries": {{}},
  "semantic_models": {{}},
  "unit_tests": {{}}
}}"#,
        nodes.join(",\n    "),
        sources.join(",\n    ")
    )
}

// ---------------------------------------------------------------------------
// Test: model missing all required tests → two violations
// ---------------------------------------------------------------------------
#[test]
fn test_model_missing_all_required_tests() {
    let manifest = build_manifest(&[model_node("model.test_project.customers", "customers")]);

    let config = r#"
manifest_tests:
  - name: "primary_key_integrity"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
      - "unique"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        2,
        "Expected 2 violations (not_null + unique)"
    );
    let messages: Vec<&str> = findings.iter().map(|f| f.0.message.as_str()).collect();
    assert!(messages.iter().any(|m| m.contains("not_null")));
    assert!(messages.iter().any(|m| m.contains("unique")));
}

// ---------------------------------------------------------------------------
// Test: model has all required tests → no violations
// ---------------------------------------------------------------------------
#[test]
fn test_model_with_all_required_tests_passes() {
    let manifest = build_manifest(&[
        model_node("model.test_project.customers", "customers"),
        test_node(
            "test.test_project.unique_customers_id",
            "unique",
            "model.test_project.customers",
            None,
        ),
        test_node(
            "test.test_project.not_null_customers_id",
            "not_null",
            "model.test_project.customers",
            None,
        ),
    ]);

    let config = r#"
manifest_tests:
  - name: "primary_key_integrity"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
      - "unique"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0, "All required tests present, should pass");
}

// ---------------------------------------------------------------------------
// Test: model missing one of two required tests → one violation
// ---------------------------------------------------------------------------
#[test]
fn test_model_missing_one_required_test() {
    let manifest = build_manifest(&[
        model_node("model.test_project.customers", "customers"),
        test_node(
            "test.test_project.unique_customers_id",
            "unique",
            "model.test_project.customers",
            None,
        ),
    ]);

    let config = r#"
manifest_tests:
  - name: "primary_key_integrity"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
      - "unique"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1, "Only not_null is missing");
    assert!(findings[0].0.message.contains("not_null"));
    assert!(findings[0].0.message.contains("customers"));
}

// ---------------------------------------------------------------------------
// Test: with_alternatives — dbt_utils variant satisfies uniqueness requirement
// ---------------------------------------------------------------------------
#[test]
fn test_with_alternatives_dbt_utils_satisfies_uniqueness() {
    let manifest = build_manifest(&[
        model_node("model.test_project.order_items", "order_items"),
        test_node(
            "test.test_project.unique_combo_order_items",
            "unique_combination_of_columns",
            "model.test_project.order_items",
            Some("dbt_utils"),
        ),
        test_node(
            "test.test_project.not_null_order_items_id",
            "not_null",
            "model.test_project.order_items",
            None,
        ),
    ]);

    let config = r#"
manifest_tests:
  - name: "primary_key_integrity"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
      - name: "uniqueness"
        allowed_names:
          - "unique"
          - "dbt_utils.unique_combination_of_columns"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "dbt_utils variant should satisfy uniqueness"
    );
}

// ---------------------------------------------------------------------------
// Test: with_alternatives — none of the alternatives present → violation uses display name
// ---------------------------------------------------------------------------
#[test]
fn test_with_alternatives_missing_uses_display_name() {
    let manifest = build_manifest(&[model_node("model.test_project.customers", "customers")]);

    let config = r#"
manifest_tests:
  - name: "require_uniqueness"
    type: "has_required_tests"
    severity: "warning"
    applies_to: ["models"]
    required_tests:
      - name: "uniqueness"
        allowed_names:
          - "unique"
          - "dbt_utils.unique_combination_of_columns"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert!(
        findings[0].0.message.contains("uniqueness"),
        "Violation message should use the display name 'uniqueness', got: {}",
        findings[0].0.message
    );
}

// ---------------------------------------------------------------------------
// Test: empty required_tests list → always passes
// ---------------------------------------------------------------------------
#[test]
fn test_empty_required_tests_passes() {
    let manifest = build_manifest(&[model_node("model.test_project.customers", "customers")]);

    let config = r#"
manifest_tests:
  - name: "no_requirements"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests: []
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0);
}

// ---------------------------------------------------------------------------
// Test: multiple models, some passing some failing
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_models_mixed_results() {
    let manifest = build_manifest(&[
        model_node("model.test_project.customers", "customers"),
        model_node("model.test_project.orders", "orders"),
        // Only customers has not_null
        test_node(
            "test.test_project.not_null_customers_id",
            "not_null",
            "model.test_project.customers",
            None,
        ),
    ]);

    let config = r#"
manifest_tests:
  - name: "require_not_null"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1, "Only orders should fail");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("not_null"));
}

// ---------------------------------------------------------------------------
// Test: severity is correctly propagated
// ---------------------------------------------------------------------------
#[test]
fn test_severity_is_propagated() {
    let manifest = build_manifest(&[model_node("model.test_project.customers", "customers")]);

    let config = r#"
manifest_tests:
  - name: "soft_requirement"
    type: "has_required_tests"
    severity: "warning"
    applies_to: ["models"]
    required_tests:
      - "not_null"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
}

// ---------------------------------------------------------------------------
// Test: test attached to wrong model doesn't count
// ---------------------------------------------------------------------------
#[test]
fn test_attached_to_wrong_model_does_not_count() {
    let manifest = build_manifest(&[
        model_node("model.test_project.customers", "customers"),
        model_node("model.test_project.orders", "orders"),
        // not_null is on orders, not customers
        test_node(
            "test.test_project.not_null_orders_id",
            "not_null",
            "model.test_project.orders",
            None,
        ),
    ]);

    let config = r#"
manifest_tests:
  - name: "require_not_null"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    // customers is missing not_null, orders has it
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("customers"));
}

// ---------------------------------------------------------------------------
// Test: source missing required test → violation
// ---------------------------------------------------------------------------
#[test]
fn test_source_missing_required_test() {
    let manifest = build_manifest_with_sources(
        &[],
        &[source_node(
            "source.test_project.raw_data.raw_customers",
            "raw_customers",
            "raw_data",
        )],
    );

    let config = r#"
manifest_tests:
  - name: "sources_need_not_null"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["sources"]
    required_tests:
      - "not_null"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("raw_customers"));
    assert!(findings[0].0.message.contains("not_null"));
}

// ---------------------------------------------------------------------------
// Test: source with required test present → passes
// ---------------------------------------------------------------------------
#[test]
fn test_source_with_required_test_passes() {
    let manifest = build_manifest_with_sources(
        &[source_test_node(
            "test.test_project.not_null_raw_customers",
            "not_null",
            "source.test_project.raw_data.raw_customers",
        )],
        &[source_node(
            "source.test_project.raw_data.raw_customers",
            "raw_customers",
            "raw_data",
        )],
    );

    let config = r#"
manifest_tests:
  - name: "sources_need_not_null"
    type: "has_required_tests"
    severity: "error"
    applies_to: ["sources"]
    required_tests:
      - "not_null"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Source has the required test, should pass"
    );
}
