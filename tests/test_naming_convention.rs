mod common;

use common::TestEnvironment;
use dbtective::core::{
    checks::manifest::node_checks::apply_node_checks, config::Config, manifest::Manifest,
};

#[test]
fn test_invalid_regex_pattern_fails() {
    let manifest = r#"{
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
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Order fact table",
      "columns": {},
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
      "raw_code": "select * from raw_orders",
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

    let config = r#"
manifest_tests:
  - name: "naming_convention"
    type: "name_convention"
    severity: "error"
    description: "All nodes must follow the naming convention."
    pattern: "(*invalid_regex"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);

    // The check should fail with an error about invalid regex
    let manifest = Manifest::from_file(&env.manifest_path).expect("Failed to load manifest");
    let config = Config::from_file(&env.config_path).expect("Failed to load config");

    let result = apply_node_checks(&manifest, &config, false);
    assert!(
        result.is_err(),
        "Expected error for invalid regex pattern, but got success"
    );

    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("Invalid regex") || error_message.contains("regex"),
        "Error message should mention regex issue, got: {error_message}"
    );
}
