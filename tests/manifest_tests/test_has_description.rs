use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_passes() {
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
      "config": {
        "enabled": true,
        "materialized": "table",
        "tags": ["finance"]
      },
      "tags": ["finance"],
      "description": "Customer dimension table with all customer information",
      "columns": {
        "customer_id": {
          "name": "customer_id",
          "description": "Primary key for customers",
          "meta": {},
          "data_type": "integer",
          "constraints": [],
          "tags": []
        }
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
      "meta": {},
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
  - name: "models_have_description"
    type: "has_description"
    severity: "error"
    description: "All models must have a description."
    applies_to:
      - "models"

  - name: "sources_have_description"
    type: "has_description"
    severity: "warning"
    description: "All sources should have a description."
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );

    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_min_length_passes() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics", "schema": "public", "name": "customers",
      "resource_type": "model", "package_name": "test_project",
      "path": "customers.sql", "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"], "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "Customer dimension table with all customer information",
      "columns": {}, "meta": {}, "group": null,
      "docs": {"show": true}, "patch_path": null, "compiled_path": null,
      "build_path": null, "deferred": false, "unrendered_config": {},
      "created_at": 1704067200.0, "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers", "language": "sql",
      "refs": [], "sources": [], "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null, "extra_ctes_injected": false, "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {}, "macros": {}, "exposures": {}, "metrics": {},
  "groups": {}, "selectors": {}, "disabled": {},
  "parent_map": {}, "child_map": {}, "group_map": {},
  "saved_queries": {}, "semantic_models": {}, "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "description_min_length"
    type: "has_description"
    min_length: 10
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_min_length_fails() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics", "schema": "public", "name": "customers",
      "resource_type": "model", "package_name": "test_project",
      "path": "customers.sql", "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"], "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "Short",
      "columns": {}, "meta": {}, "group": null,
      "docs": {"show": true}, "patch_path": null, "compiled_path": null,
      "build_path": null, "deferred": false, "unrendered_config": {},
      "created_at": 1704067200.0, "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers", "language": "sql",
      "refs": [], "sources": [], "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null, "extra_ctes_injected": false, "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {}, "macros": {}, "exposures": {}, "metrics": {},
  "groups": {}, "selectors": {}, "disabled": {},
  "parent_map": {}, "child_map": {}, "group_map": {},
  "saved_queries": {}, "semantic_models": {}, "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "description_min_length"
    type: "has_description"
    min_length: 10
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("too short"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_forbidden_substrings_passes() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics", "schema": "public", "name": "customers",
      "resource_type": "model", "package_name": "test_project",
      "path": "customers.sql", "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"], "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "Customer dimension table with all customer information",
      "columns": {}, "meta": {}, "group": null,
      "docs": {"show": true}, "patch_path": null, "compiled_path": null,
      "build_path": null, "deferred": false, "unrendered_config": {},
      "created_at": 1704067200.0, "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers", "language": "sql",
      "refs": [], "sources": [], "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null, "extra_ctes_injected": false, "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {}, "macros": {}, "exposures": {}, "metrics": {},
  "groups": {}, "selectors": {}, "disabled": {},
  "parent_map": {}, "child_map": {}, "group_map": {},
  "saved_queries": {}, "semantic_models": {}, "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "no_placeholder_descriptions"
    type: "has_description"
    forbidden_substrings: ["TODO", "test_description", "placeholder"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_forbidden_substrings_fails() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics", "schema": "public", "name": "customers",
      "resource_type": "model", "package_name": "test_project",
      "path": "customers.sql", "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"], "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "TODO: add a proper description later",
      "columns": {}, "meta": {}, "group": null,
      "docs": {"show": true}, "patch_path": null, "compiled_path": null,
      "build_path": null, "deferred": false, "unrendered_config": {},
      "created_at": 1704067200.0, "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers", "language": "sql",
      "refs": [], "sources": [], "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null, "extra_ctes_injected": false, "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {}, "macros": {}, "exposures": {}, "metrics": {},
  "groups": {}, "selectors": {}, "disabled": {},
  "parent_map": {}, "child_map": {}, "group_map": {},
  "saved_queries": {}, "semantic_models": {}, "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "no_placeholder_descriptions"
    type: "has_description"
    forbidden_substrings: ["TODO", "test_description", "placeholder"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("forbidden substring"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_combined_min_length_and_forbidden() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.customers": {
      "database": "analytics", "schema": "public", "name": "customers",
      "resource_type": "model", "package_name": "test_project",
      "path": "customers.sql", "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"], "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "A proper and meaningful description of the customers model",
      "columns": {}, "meta": {}, "group": null,
      "docs": {"show": true}, "patch_path": null, "compiled_path": null,
      "build_path": null, "deferred": false, "unrendered_config": {},
      "created_at": 1704067200.0, "config_call_dict": {},
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers", "language": "sql",
      "refs": [], "sources": [], "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null, "extra_ctes_injected": false, "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {}, "macros": {}, "exposures": {}, "metrics": {},
  "groups": {}, "selectors": {}, "disabled": {},
  "parent_map": {}, "child_map": {}, "group_map": {},
  "saved_queries": {}, "semantic_models": {}, "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "quality_descriptions"
    type: "has_description"
    min_length: 10
    forbidden_substrings: ["TODO", "placeholder"]
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}
