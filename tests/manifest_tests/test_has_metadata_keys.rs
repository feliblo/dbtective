use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_metadata_keys() {
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
    "model.test_project.doesntcontain": {
      "database": "analytics",
      "schema": "public",
      "name": "doesntcontain",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "doesntcontain.sql",
      "original_file_path": "models/doesntcontain.sql",
      "unique_id": "model.test_project.doesntcontain",
      "fqn": ["test_project", "doesntcontain"],
      "alias": "doesntcontain",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "table",
        "tags": ["finance"]
      },
      "tags": ["finance"],
      "description": "doesntcontain dimension table with all customer information",
      "columns": {
        "doesntcontain": {
          "name": "doesntcontain",
          "description": "Primary key for doesntcontain",
          "meta": {},
          "data_type": "integer",
          "constraints": [],
          "tags": []
        }
      },
      "meta": {
        "key1": "value1",
        "key2": "value2"
      },
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
      "raw_code": "select * from raw_customers",
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
  "sources": {
    "source.test_project.raw_data.raw_customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "raw_customers",
      "source_name": "raw_data",
      "source_description": "Raw data sources",
      "loader": "",
      "identifier": "raw_customers",
      "resource_type": "source",
      "package_name": "test_project",
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw_data.raw_customers",
      "fqn": ["test_project", "raw_data", "raw_customers"],
      "source_meta": {},
      "tags": [],
      "config": {"enabled": true},
      "patch_path": null,
      "unrendered_config": {},
      "relation_name": "raw.raw_data.raw_customers",
      "created_at": 1704067200.0,
      "description": "Raw customer data from CRM system",
      "columns": {},
      "meta": {
        "key1": "value1"
      },
      "source_description": "Raw data sources",
      "freshness": {
        "warn_after": {"count": null, "period": null},
        "error_after": {"count": null, "period": null},
        "filter": null
      },
      "quoting": {
        "database": null,
        "schema": null,
        "identifier": null,
        "column": null
      },
      "loaded_at_field": null,
      "external": null
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

    let config = r#"
manifest_tests:
  - name: "models_have_metadata_keys"
    type: "has_metadata_keys"
    severity: "error"
    description: "All models must have required metadata keys."
    required_keys: ["key1", "key2"]
    applies_to:
      - "models"

  - name: "sources_have_metadata_keys"
    type: "has_metadata_keys"
    severity: "warning"
    description: "All sources should have required metadata keys."
    required_keys: ["key1", "key2"]
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Expected one finding for source (misses.1 key), but got: {findings:?}"
    );

    assert_eq!(findings[0].0.rule_name, "sources_have_metadata_keys");
    assert_eq!(
        findings[0].0.message,
        "raw_customers is missing required metadata keys: key2."
    );
    // shouldnt contain any reference to the model which does contain all keys
    assert!(
        !findings[0].0.message.contains("doesntcontain"),
        "Message should not contain 'doesntcontain'"
    );
    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 0);
}
