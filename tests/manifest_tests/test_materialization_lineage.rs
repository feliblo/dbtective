use crate::common::TestEnvironment;

const MANIFEST_TEMPLATE: &str = r#"{
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
      "database": "db",
      "schema": "public",
      "name": "stg_orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "stg_orders.sql",
      "original_file_path": "models/staging/stg_orders.sql",
      "unique_id": "model.test_project.stg_orders",
      "fqn": ["test_project", "staging", "stg_orders"],
      "alias": "stg_orders",
      "checksum": {"name": "sha256", "checksum": "aaa"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["source.test_project.raw.orders"], "macros": []}
    },
    "model.test_project.int_orders": {
      "database": "db",
      "schema": "public",
      "name": "int_orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "int_orders.sql",
      "original_file_path": "models/intermediate/int_orders.sql",
      "unique_id": "model.test_project.int_orders",
      "fqn": ["test_project", "intermediate", "int_orders"],
      "alias": "int_orders",
      "checksum": {"name": "sha256", "checksum": "bbb"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.stg_orders"], "macros": []}
    },
    "model.test_project.int_order_items": {
      "database": "db",
      "schema": "public",
      "name": "int_order_items",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "int_order_items.sql",
      "original_file_path": "models/intermediate/int_order_items.sql",
      "unique_id": "model.test_project.int_order_items",
      "fqn": ["test_project", "intermediate", "int_order_items"],
      "alias": "int_order_items",
      "checksum": {"name": "sha256", "checksum": "ccc"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "ephemeral"},
      "depends_on": {"nodes": ["model.test_project.int_orders"], "macros": []}
    },
    "model.test_project.int_order_agg": {
      "database": "db",
      "schema": "public",
      "name": "int_order_agg",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "int_order_agg.sql",
      "original_file_path": "models/intermediate/int_order_agg.sql",
      "unique_id": "model.test_project.int_order_agg",
      "fqn": ["test_project", "intermediate", "int_order_agg"],
      "alias": "int_order_agg",
      "checksum": {"name": "sha256", "checksum": "ddd"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.int_order_items"], "macros": []}
    },
    "model.test_project.fct_orders": {
      "database": "db",
      "schema": "public",
      "name": "fct_orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "fct_orders.sql",
      "original_file_path": "models/marts/fct_orders.sql",
      "unique_id": "model.test_project.fct_orders",
      "fqn": ["test_project", "marts", "fct_orders"],
      "alias": "fct_orders",
      "checksum": {"name": "sha256", "checksum": "eee"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": ["model.test_project.int_order_agg"], "macros": []}
    },
    "model.test_project.dim_customers": {
      "database": "db",
      "schema": "public",
      "name": "dim_customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "dim_customers.sql",
      "original_file_path": "models/marts/dim_customers.sql",
      "unique_id": "model.test_project.dim_customers",
      "fqn": ["test_project", "marts", "dim_customers"],
      "alias": "dim_customers",
      "checksum": {"name": "sha256", "checksum": "fff"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "table"},
      "depends_on": {"nodes": ["source.test_project.raw.customers"], "macros": []}
    }
  },
  "sources": {
    "source.test_project.raw.orders": {
      "database": "db",
      "schema": "raw",
      "name": "orders",
      "resource_type": "source",
      "package_name": "test_project",
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw.orders",
      "fqn": ["test_project", "raw", "orders"],
      "source_name": "raw",
      "source_description": "",
      "loader": "",
      "identifier": "orders"
    },
    "source.test_project.raw.customers": {
      "database": "db",
      "schema": "raw",
      "name": "customers",
      "resource_type": "source",
      "package_name": "test_project",
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw.customers",
      "fqn": ["test_project", "raw", "customers"],
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
    "model.test_project.stg_orders": ["source.test_project.raw.orders"],
    "model.test_project.int_orders": ["model.test_project.stg_orders"],
    "model.test_project.int_order_items": ["model.test_project.int_orders"],
    "model.test_project.int_order_agg": ["model.test_project.int_order_items"],
    "model.test_project.fct_orders": ["model.test_project.int_order_agg"],
    "model.test_project.dim_customers": ["source.test_project.raw.customers"]
  },
  "child_map": {},
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

#[test]
fn test_max_materialization_lineage_chain_exceeds_limit() {
    // Chain: stg_orders(view) -> int_orders(view) -> int_order_items(ephemeral)
    //        -> int_order_agg(view) -> fct_orders(view)
    // fct_orders has chain length 5, max is 4 -> should fail
    let config = r#"
manifest_tests:
  - name: "no_long_view_chains"
    type: "max_materialization_lineage"
    max: 4
    severity: "warning"
"#;

    let env = TestEnvironment::new(MANIFEST_TEMPLATE, config);
    let findings = env.run_manifest_rules(false);

    // fct_orders (chain=5) should fail. int_order_agg (chain=4) is at the limit but not over.
    let lineage_findings: Vec<_> = findings
        .iter()
        .filter(|(r, _)| r.rule_name == "no_long_view_chains")
        .collect();

    assert_eq!(lineage_findings.len(), 1, "Expected exactly 1 violation");
    assert!(lineage_findings[0].0.message.contains("fct_orders"));
    assert!(lineage_findings[0].0.message.contains('5'));
    assert!(lineage_findings[0].0.message.contains("max: 4"));
}

#[test]
fn test_max_materialization_lineage_within_limit() {
    // With max=5, fct_orders (chain=5) is at the limit, not over -> should pass
    let config = r#"
manifest_tests:
  - name: "no_long_view_chains"
    type: "max_materialization_lineage"
    max: 5
    severity: "warning"
"#;

    let env = TestEnvironment::new(MANIFEST_TEMPLATE, config);
    let findings = env.run_manifest_rules(false);

    assert!(
        !findings
            .iter()
            .any(|(r, _)| r.rule_name == "no_long_view_chains"),
        "Expected no violations with max=5"
    );
}

#[test]
fn test_max_materialization_lineage_table_model_not_checked() {
    // dim_customers is a table -> should never trigger a violation
    let config = r#"
manifest_tests:
  - name: "no_long_view_chains"
    type: "max_materialization_lineage"
    max: 1
    severity: "error"
"#;

    let env = TestEnvironment::new(MANIFEST_TEMPLATE, config);
    let findings = env.run_manifest_rules(false);

    assert!(
        !findings
            .iter()
            .any(|(r, _)| r.message.contains("dim_customers")),
        "Table models should not be checked"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_exposure_parents_materialized_violation() {
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
    "model.test_project.fct_revenue": {
      "database": "db",
      "schema": "public",
      "name": "fct_revenue",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "fct_revenue.sql",
      "original_file_path": "models/marts/fct_revenue.sql",
      "unique_id": "model.test_project.fct_revenue",
      "fqn": ["test_project", "marts", "fct_revenue"],
      "alias": "fct_revenue",
      "checksum": {"name": "sha256", "checksum": "aaa"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "table"},
      "depends_on": {"nodes": [], "macros": []}
    },
    "model.test_project.dim_dates": {
      "database": "db",
      "schema": "public",
      "name": "dim_dates",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "dim_dates.sql",
      "original_file_path": "models/marts/dim_dates.sql",
      "unique_id": "model.test_project.dim_dates",
      "fqn": ["test_project", "marts", "dim_dates"],
      "alias": "dim_dates",
      "checksum": {"name": "sha256", "checksum": "bbb"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "view"},
      "depends_on": {"nodes": [], "macros": []}
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {
    "exposure.test_project.weekly_dashboard": {
      "name": "weekly_dashboard",
      "package_name": "test_project",
      "original_file_path": "models/exposures.yml",
      "depends_on": {
        "nodes": ["model.test_project.fct_revenue", "model.test_project.dim_dates"],
        "macros": []
      }
    }
  },
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
  - name: "exposure_parents_must_be_materialized"
    type: "exposure_parents_materialized"
    severity: "error"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    let exposure_findings: Vec<_> = findings
        .iter()
        .filter(|(r, _)| r.rule_name == "exposure_parents_must_be_materialized")
        .collect();

    // dim_dates is a view -> should fail. fct_revenue is a table -> should pass.
    assert_eq!(
        exposure_findings.len(),
        1,
        "Expected 1 violation for view parent"
    );
    assert!(exposure_findings[0].0.message.contains("dim_dates"));
    assert!(exposure_findings[0].0.message.contains("view"));
    assert_eq!(exposure_findings[0].0.severity, "FAIL");
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_exposure_parents_materialized_all_pass() {
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
    "model.test_project.fct_revenue": {
      "database": "db",
      "schema": "public",
      "name": "fct_revenue",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "fct_revenue.sql",
      "original_file_path": "models/marts/fct_revenue.sql",
      "unique_id": "model.test_project.fct_revenue",
      "fqn": ["test_project", "marts", "fct_revenue"],
      "alias": "fct_revenue",
      "checksum": {"name": "sha256", "checksum": "aaa"},
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "config": {"materialized": "incremental"},
      "depends_on": {"nodes": [], "macros": []}
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {
    "exposure.test_project.dashboard": {
      "name": "dashboard",
      "package_name": "test_project",
      "original_file_path": "models/exposures.yml",
      "depends_on": {
        "nodes": ["model.test_project.fct_revenue"],
        "macros": []
      }
    }
  },
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
  - name: "exposure_parents_must_be_materialized"
    type: "exposure_parents_materialized"
    severity: "error"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert!(
        !findings
            .iter()
            .any(|(r, _)| r.rule_name == "exposure_parents_must_be_materialized"),
        "Incremental parent should pass"
    );
}
