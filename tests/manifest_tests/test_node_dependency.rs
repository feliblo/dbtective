use crate::common::TestEnvironment;

// Shared manifest: a small layered DAG with source → stg → int → fct
// Also includes a few "bad" models that violate common DAG patterns.
//
//  source.raw.orders
//    └─► stg_orders        (good: stg from source)
//          └─► int_orders  (good: int from stg)
//                └─► fct_orders (good: fct from int)
//
//  stg_bad_dep      depends on stg_orders  (bad: stg → stg)
//  int_direct_src   depends on source.raw.orders  (bad: int → source)
//  stg_inverted     depends on int_orders   (bad: stg → downstream)

const MANIFEST: &str = r#"{
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
    "model.test_project.stg_orders": {
      "database": "db", "schema": "public", "name": "stg_orders",
      "resource_type": "model", "package_name": "test_project",
      "path": "stg_orders.sql",
      "original_file_path": "models/staging/stg_orders.sql",
      "unique_id": "model.test_project.stg_orders",
      "fqn": ["test_project", "staging", "stg_orders"],
      "alias": "stg_orders", "checksum": {"name": "sha256", "checksum": "aaa"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["source.test_project.raw.orders"], "macros": []}
    },
    "model.test_project.int_orders": {
      "database": "db", "schema": "public", "name": "int_orders",
      "resource_type": "model", "package_name": "test_project",
      "path": "int_orders.sql",
      "original_file_path": "models/intermediate/int_orders.sql",
      "unique_id": "model.test_project.int_orders",
      "fqn": ["test_project", "intermediate", "int_orders"],
      "alias": "int_orders", "checksum": {"name": "sha256", "checksum": "bbb"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.stg_orders"], "macros": []}
    },
    "model.test_project.fct_orders": {
      "database": "db", "schema": "public", "name": "fct_orders",
      "resource_type": "model", "package_name": "test_project",
      "path": "fct_orders.sql",
      "original_file_path": "models/marts/fct_orders.sql",
      "unique_id": "model.test_project.fct_orders",
      "fqn": ["test_project", "marts", "fct_orders"],
      "alias": "fct_orders", "checksum": {"name": "sha256", "checksum": "ccc"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "table"},
      "depends_on": {"nodes": ["model.test_project.int_orders"], "macros": []}
    },
    "model.test_project.stg_bad_dep": {
      "database": "db", "schema": "public", "name": "stg_bad_dep",
      "resource_type": "model", "package_name": "test_project",
      "path": "stg_bad_dep.sql",
      "original_file_path": "models/staging/stg_bad_dep.sql",
      "unique_id": "model.test_project.stg_bad_dep",
      "fqn": ["test_project", "staging", "stg_bad_dep"],
      "alias": "stg_bad_dep", "checksum": {"name": "sha256", "checksum": "ddd"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.stg_orders"], "macros": []}
    },
    "model.test_project.int_direct_src": {
      "database": "db", "schema": "public", "name": "int_direct_src",
      "resource_type": "model", "package_name": "test_project",
      "path": "int_direct_src.sql",
      "original_file_path": "models/intermediate/int_direct_src.sql",
      "unique_id": "model.test_project.int_direct_src",
      "fqn": ["test_project", "intermediate", "int_direct_src"],
      "alias": "int_direct_src", "checksum": {"name": "sha256", "checksum": "eee"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["source.test_project.raw.orders"], "macros": []}
    },
    "model.test_project.stg_inverted": {
      "database": "db", "schema": "public", "name": "stg_inverted",
      "resource_type": "model", "package_name": "test_project",
      "path": "stg_inverted.sql",
      "original_file_path": "models/staging/stg_inverted.sql",
      "unique_id": "model.test_project.stg_inverted",
      "fqn": ["test_project", "staging", "stg_inverted"],
      "alias": "stg_inverted", "checksum": {"name": "sha256", "checksum": "fff"},
      "tags": [], "description": "", "columns": {}, "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.int_orders"], "macros": []}
    }
  },
  "sources": {
    "source.test_project.raw.orders": {
      "database": "db", "schema": "raw", "name": "orders",
      "source_name": "raw",
      "resource_type": "source", "package_name": "test_project",
      "path": "models/staging/_sources.yml",
      "original_file_path": "models/staging/_sources.yml",
      "unique_id": "source.test_project.raw.orders",
      "fqn": ["test_project", "raw", "orders"],
      "description": "", "columns": {}, "meta": {}, "tags": [],
      "source_description": "", "loader": "",
      "identifier": "orders"
    }
  },
  "macros": {},
  "exposures": {},
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {
    "model.test_project.stg_orders": ["source.test_project.raw.orders"],
    "model.test_project.int_orders": ["model.test_project.stg_orders"],
    "model.test_project.fct_orders": ["model.test_project.int_orders"],
    "model.test_project.stg_bad_dep": ["model.test_project.stg_orders"],
    "model.test_project.int_direct_src": ["source.test_project.raw.orders"],
    "model.test_project.stg_inverted": ["model.test_project.int_orders"]
  },
  "child_map": {},
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

// ============================================================================
// Use case 1: stg_ must not depend on stg_
// ============================================================================

#[test]
fn test_stg_to_stg_catches_violation() {
    let config = r#"
manifest_tests:
  - name: "no_stg_to_stg"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^stg_"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // Only stg_bad_dep → stg_orders should fire
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("stg_bad_dep"));
    assert!(findings[0].0.message.contains("stg_orders"));
}

#[test]
fn test_stg_to_stg_no_violation_for_compliant_models() {
    let config = r#"
manifest_tests:
  - name: "no_stg_to_stg"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^stg_"
    severity: "error"
    includes: ["models/staging/stg_orders.sql"]
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // stg_orders itself has no stg_ parent, so no violations
    assert!(findings.is_empty());
}

// ============================================================================
// Use case 2: int_/fct_/dim_ must not bypass staging (no direct source deps)
// ============================================================================

#[test]
fn test_bypass_staging_catches_violation() {
    let config = r#"
manifest_tests:
  - name: "no_bypass_staging"
    type: "node_dependency"
    from_name_pattern: "^(int_|fct_|dim_)"
    parent_type: "source"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // Only int_direct_src → source.raw.orders should fire
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("int_direct_src"));
}

#[test]
fn test_bypass_staging_clean_models_pass() {
    let config = r#"
manifest_tests:
  - name: "no_bypass_staging"
    type: "node_dependency"
    from_name_pattern: "^(int_|fct_|dim_)"
    parent_type: "source"
    severity: "error"
    includes: ["models/marts"]
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // fct_orders (the only mart here) has int_orders as parent, not a source
    assert!(findings.is_empty());
}

// ============================================================================
// Use case 3: stg_ must not depend on downstream layers (inverted DAG)
// ============================================================================

#[test]
fn test_inverted_dag_catches_violation() {
    let config = r#"
manifest_tests:
  - name: "no_inverted_dag"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^int_"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // Only stg_inverted → int_orders should fire
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("stg_inverted"));
    assert!(findings[0].0.message.contains("int_orders"));
}

// ============================================================================
// parent_type alone (no name pattern)
// ============================================================================

#[test]
fn test_parent_type_source_no_name_pattern() {
    let config = r#"
manifest_tests:
  - name: "no_source_deps_anywhere"
    type: "node_dependency"
    parent_type: "source"
    severity: "warning"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // stg_orders and int_direct_src both depend on source directly
    assert_eq!(findings.len(), 2);
    let names: Vec<_> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("stg_orders")));
    assert!(names.iter().any(|m| m.contains("int_direct_src")));
}

// ============================================================================
// No filters at all — all dependencies flagged
// ============================================================================

#[test]
fn test_empty_filters_flags_all_deps() {
    let config = r#"
manifest_tests:
  - name: "no_deps_at_all"
    type: "node_dependency"
    severity: "warning"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // Every model has exactly one parent in our DAG = 6 models × 1 parent each = 6 violations
    assert_eq!(findings.len(), 6);
}

// ============================================================================
// Severity is passed through correctly
// ============================================================================

#[test]
fn test_severity_warning() {
    let config = r#"
manifest_tests:
  - name: "no_stg_to_stg"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^stg_"
    severity: "warning"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].1,
        dbtective::core::config::severity::Severity::Warning
    );
}

// ============================================================================
// Rule name propagates to results
// ============================================================================

#[test]
fn test_rule_name_propagates() {
    let config = r#"
manifest_tests:
  - name: "my_custom_rule_name"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^stg_"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.rule_name, "my_custom_rule_name");
}

// ============================================================================
// parent_path_pattern filtering
// ============================================================================

#[test]
fn test_parent_path_pattern_matches_staging_path() {
    // stg_bad_dep depends on stg_orders (path: models/staging/stg_orders.sql)
    // Using parent_path_pattern "staging" should flag that dependency
    let config = r#"
manifest_tests:
  - name: "no_dep_on_staging_path"
    type: "node_dependency"
    parent_path_pattern: "staging"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // stg_bad_dep → stg_orders (staging), int_orders → stg_orders (staging),
    // fct_orders → int_orders (intermediate — no match), stg_orders → source (no match)
    // stg_inverted → int_orders (intermediate — no match)
    // int_direct_src → source (no match)
    // Expected: stg_bad_dep and int_orders both depend on stg_orders (staging path)
    let names: Vec<_> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(
        names.iter().any(|m| m.contains("stg_bad_dep")),
        "expected stg_bad_dep violation, got: {names:?}"
    );
    assert!(
        names.iter().any(|m| m.contains("int_orders")),
        "expected int_orders violation, got: {names:?}"
    );
}

#[test]
fn test_parent_path_pattern_no_match_produces_no_violations() {
    let config = r#"
manifest_tests:
  - name: "no_dep_on_nonexistent_path"
    type: "node_dependency"
    parent_path_pattern: "nonexistent_layer"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert!(findings.is_empty());
}

#[test]
fn test_parent_path_pattern_combined_with_from_name_pattern() {
    // Flag stg_ models that depend on a parent whose path contains "staging"
    let config = r#"
manifest_tests:
  - name: "stg_to_stg_path"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_path_pattern: "staging"
    severity: "error"
"#;
    let env = TestEnvironment::new(MANIFEST, config);
    let findings = env.run_manifest_rules(false);

    // stg_bad_dep → stg_orders (models/staging/stg_orders.sql) — matches
    // stg_orders → source.raw.orders (models/staging/_sources.yml) — source path also contains "staging", matches
    // stg_inverted → int_orders (models/intermediate/int_orders.sql) — no match
    assert_eq!(findings.len(), 2);
    let names: Vec<_> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("stg_bad_dep")));
    assert!(names.iter().any(|m| m.contains("stg_orders")));
}
