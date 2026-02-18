use crate::common::TestEnvironment;

#[test]
fn test_source_with_freshness_passes() {
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
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.raw_customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "raw_customers",
      "source_name": "raw_data",
      "source_description": "Raw data sources",
      "loader": "fivetran",
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
      "description": "Raw customer data",
      "columns": {},
      "meta": {},
      "freshness": {
        "warn_after": {"count": 24, "period": "hour"},
        "error_after": {"count": 48, "period": "hour"},
        "filter": null
      },
      "quoting": {
        "database": null,
        "schema": null,
        "identifier": null,
        "column": null
      },
      "loaded_at_field": "updated_at",
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
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_source_without_freshness_fails() {
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
  "nodes": {},
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
      "description": "Raw customer data",
      "columns": {},
      "meta": {},
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
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("missing freshness"));
}

#[test]
fn test_source_without_freshness_key_fails() {
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
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.raw_customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "raw_customers",
      "source_name": "raw_data",
      "source_description": "Raw data sources",
      "loader": "fivetran",
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
      "description": "Raw customer data",
      "columns": {},
      "meta": {},
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
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("missing freshness"));
}

#[test]
fn test_source_without_freshness_subkeys_fails() {
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
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.raw_customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "raw_customers",
      "source_name": "raw_data",
      "source_description": "Raw data sources",
      "loader": "fivetran",
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
      "description": "Raw customer data",
      "columns": {},
      "meta": {},
      "freshness": {},
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
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    severity: "warning"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("missing freshness"));
}

#[test]
fn test_source_with_only_warn_after_passes() {
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
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.raw_customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "raw_customers",
      "source_name": "raw_data",
      "source_description": "Raw data sources",
      "loader": "fivetran",
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
      "description": "Raw customer data",
      "columns": {},
      "meta": {},
      "freshness": {
        "warn_after": {"count": 12, "period": "hour"},
        "error_after": {"count": null, "period": null},
        "filter": null
      },
      "quoting": {
        "database": null,
        "schema": null,
        "identifier": null,
        "column": null
      },
      "loaded_at_field": "updated_at",
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
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    severity: "warning"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}
