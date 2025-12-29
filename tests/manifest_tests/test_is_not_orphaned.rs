use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_source_with_no_children() {
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
    "source.test_project.raw_data.customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "customers",
      "source_name": "raw_data",
      "source_description": "Raw data",
      "loader": "",
      "identifier": "customers",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [],
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw_data.customers",
      "fqn": ["test_project", "raw_data", "customers"],
      "config": {"enabled": true},
      "description": "Raw customer data",
      "columns": {
        "id": {"name": "id", "description": "Customer ID", "tags": []}
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
  "child_map": {
    "source.test_project.raw_data.customers": []
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "sources_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Sources should not be orphaned."
    severity: "error"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0]
        .0
        .message
        .contains("is orphaned (not referenced by any other object)"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_source_with_model_child_passes() {
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
    "model.test_project.stg_customers": {
      "database": "analytics",
      "schema": "staging",
      "name": "stg_customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/stg_customers.sql",
      "original_file_path": "models/staging/stg_customers.sql",
      "unique_id": "model.test_project.stg_customers",
      "fqn": ["test_project", "staging", "stg_customers"],
      "alias": "stg_customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Staging customers model",
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
      "relation_name": "analytics.staging.stg_customers",
      "raw_code": "select * from {{ source('raw_data', 'customers') }}",
      "language": "sql",
      "refs": [],
      "sources": [["raw_data", "customers"]],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["source.test_project.raw_data.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    }
  },
  "sources": {
    "source.test_project.raw_data.customers": {
      "database": "raw",
      "schema": "raw_data",
      "name": "customers",
      "source_name": "raw_data",
      "source_description": "Raw data",
      "loader": "",
      "identifier": "customers",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [],
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw_data.customers",
      "fqn": ["test_project", "raw_data", "customers"],
      "config": {"enabled": true},
      "description": "Raw customer data",
      "columns": {
        "id": {"name": "id", "description": "Customer ID", "tags": []}
      }
    }
  },
  "macros": {},
  "exposures": {},
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {
    "model.test_project.stg_customers": ["source.test_project.raw_data.customers"]
  },
  "child_map": {
    "source.test_project.raw_data.customers": ["model.test_project.stg_customers"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "sources_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Sources should not be orphaned."
    severity: "error"
    allowed_references:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should pass: source is referenced by a model
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_model_with_no_children() {
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
    "model.test_project.unused_model": {
      "database": "analytics",
      "schema": "staging",
      "name": "unused_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/unused_model.sql",
      "original_file_path": "models/staging/unused_model.sql",
      "unique_id": "model.test_project.unused_model",
      "fqn": ["test_project", "staging", "unused_model"],
      "alias": "unused_model",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "This model is not used anywhere",
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
      "relation_name": "analytics.staging.unused_model",
      "raw_code": "select 1 as id",
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
  "child_map": {
    "model.test_project.unused_model": []
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Models should not be orphaned."
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("unused_model"));
    assert!(findings[0]
        .0
        .message
        .contains("is orphaned (not referenced by any other object)"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_model_referenced_by_exposure_passes() {
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
    "model.test_project.customer_metrics": {
      "database": "analytics",
      "schema": "marts",
      "name": "customer_metrics",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "marts/customer_metrics.sql",
      "original_file_path": "models/marts/customer_metrics.sql",
      "unique_id": "model.test_project.customer_metrics",
      "fqn": ["test_project", "marts", "customer_metrics"],
      "alias": "customer_metrics",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "table",
        "tags": []
      },
      "tags": [],
      "description": "Customer metrics for dashboard",
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
      "relation_name": "analytics.marts.customer_metrics",
      "raw_code": "select * from customers",
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
  "exposures": {
    "exposure.test_project.customer_dashboard": {
      "name": "customer_dashboard",
      "type": "dashboard",
      "owner": {"name": "Analytics Team", "email": "analytics@example.com"},
      "resource_type": "exposure",
      "package_name": "test_project",
      "path": "exposures.yml",
      "original_file_path": "models/exposures.yml",
      "unique_id": "exposure.test_project.customer_dashboard",
      "fqn": ["test_project", "customer_dashboard"],
      "description": "Customer dashboard in Tableau",
      "label": null,
      "maturity": null,
      "meta": {},
      "tags": [],
      "config": {"enabled": true},
      "unrendered_config": {},
      "url": null,
      "depends_on": {"macros": [], "nodes": ["model.test_project.customer_metrics"]},
      "refs": [["customer_metrics"]],
      "sources": [],
      "metrics": [],
      "created_at": 1704067200.0
    }
  },
  "metrics": {},
  "groups": {},
  "selectors": {},
  "disabled": {},
  "parent_map": {
    "exposure.test_project.customer_dashboard": ["model.test_project.customer_metrics"]
  },
  "child_map": {
    "model.test_project.customer_metrics": ["exposure.test_project.customer_dashboard"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Models should not be orphaned."
    severity: "error"
    applies_to:
      - "models"
    allowed_references:
      - "exposures"
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should pass: model is referenced by an exposure
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_only_referenced_by_non_allowed_objects() {
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
    "model.test_project.test_helper": {
      "database": "analytics",
      "schema": "staging",
      "name": "test_helper",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/test_helper.sql",
      "original_file_path": "models/staging/test_helper.sql",
      "unique_id": "model.test_project.test_helper",
      "fqn": ["test_project", "staging", "test_helper"],
      "alias": "test_helper",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Helper model only used in tests",
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
      "relation_name": "analytics.staging.test_helper",
      "raw_code": "select 1 as id",
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
    "test.test_project.some_test": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "some_test",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "some_test.sql",
      "original_file_path": "tests/some_test.sql",
      "unique_id": "test.test_project.some_test",
      "fqn": ["test_project", "some_test"],
      "alias": "some_test",
      "checksum": {"name": "sha256", "checksum": "def456"},
      "config": {"enabled": true},
      "tags": [],
      "description": "",
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
      "relation_name": null,
      "raw_code": "select * from {{ ref('test_helper') }}",
      "language": "sql",
      "refs": [["test_helper"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.test_helper"]},
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
  "parent_map": {
    "test.test_project.some_test": ["model.test_project.test_helper"]
  },
  "child_map": {
    "model.test_project.test_helper": ["test.test_project.some_test"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Models should not be orphaned."
    severity: "error"
    applies_to:
      - "models"
    allowed_references:
      - "models"
      - "exposures"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should fail: model is only referenced by a test, which is not in allowed_references
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("test_helper"));
    assert!(findings[0]
        .0
        .message
        .contains("is orphaned (only referenced by non-allowed objects)"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_is_not_orphaned_mixed_scenario() {
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
    "model.test_project.used_model": {
      "database": "analytics",
      "schema": "staging",
      "name": "used_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/used_model.sql",
      "original_file_path": "models/staging/used_model.sql",
      "unique_id": "model.test_project.used_model",
      "fqn": ["test_project", "staging", "used_model"],
      "alias": "used_model",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "This model is used",
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
      "relation_name": "analytics.staging.used_model",
      "raw_code": "select 1 as id",
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
    "model.test_project.orphaned_model": {
      "database": "analytics",
      "schema": "staging",
      "name": "orphaned_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/orphaned_model.sql",
      "original_file_path": "models/staging/orphaned_model.sql",
      "unique_id": "model.test_project.orphaned_model",
      "fqn": ["test_project", "staging", "orphaned_model"],
      "alias": "orphaned_model",
      "checksum": {"name": "sha256", "checksum": "def456"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "This model is orphaned",
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
      "relation_name": "analytics.staging.orphaned_model",
      "raw_code": "select 2 as id",
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
    "model.test_project.downstream_model": {
      "database": "analytics",
      "schema": "marts",
      "name": "downstream_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "marts/downstream_model.sql",
      "original_file_path": "models/marts/downstream_model.sql",
      "unique_id": "model.test_project.downstream_model",
      "fqn": ["test_project", "marts", "downstream_model"],
      "alias": "downstream_model",
      "checksum": {"name": "sha256", "checksum": "ghi789"},
      "config": {
        "enabled": true,
        "materialized": "table",
        "tags": []
      },
      "tags": [],
      "description": "Downstream model",
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
      "relation_name": "analytics.marts.downstream_model",
      "raw_code": "select * from {{ ref('used_model') }}",
      "language": "sql",
      "refs": [["used_model"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.used_model"]},
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
  "parent_map": {
    "model.test_project.downstream_model": ["model.test_project.used_model"]
  },
  "child_map": {
    "model.test_project.used_model": ["model.test_project.downstream_model"],
    "model.test_project.orphaned_model": [],
    "model.test_project.downstream_model": []
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_not_be_orphaned"
    type: "is_not_orphaned"
    description: "Models should not be orphaned."
    severity: "error"
    applies_to:
      - "models"
    allowed_references:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Should find 2 orphaned models: orphaned_model (no children) and downstream_model (no children)
    assert_eq!(findings.len(), 2);

    let orphaned_names: Vec<&str> = findings
        .iter()
        .filter_map(|f| {
            if f.0.message.contains("orphaned_model") {
                Some("orphaned_model")
            } else if f.0.message.contains("downstream_model") {
                Some("downstream_model")
            } else {
                None
            }
        })
        .collect();

    assert_eq!(orphaned_names.len(), 2);
    assert!(orphaned_names.contains(&"orphaned_model"));
    assert!(orphaned_names.contains(&"downstream_model"));
}
