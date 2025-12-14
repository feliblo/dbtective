mod common;

use common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_catalog_columns_documented_fails() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Orders table",
      "columns": {
        "id": {
          "name": "id",
          "description": "Order ID",
          "index": 1,
          "meta": {},
          "data_type": null,
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "orders",
        "database": "analytics"
      },
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "customer_id": {"type": "INTEGER", "name": "customer_id", "index": 2},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should fail: catalog has 3 columns but manifest only documents 1
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "columns_documented");
    assert!(findings[0].0.message.contains("orders"));
    assert!(
        findings[0].0.message.contains("customer_id") || findings[0].0.message.contains("amount")
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_catalog_columns_documented_pass() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "depends_on": {
        "macros": []
      },
      "description": "Customer table",
      "columns": {
        "id": {"name": "id", "description": "Customer ID", "tags": []},
        "name": {"name": "name", "description": "Customer name", "tags": []},
        "email": {"name": "email", "description": "Customer email", "tags": []}
      }
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.customers": {
      "unique_id": "model.test_project.customers",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "customers",
        "database": "analytics"
      },
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should pass: all catalog columns are documented in manifest
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_catalog_applies_to_filter() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "depends_on": {
        "nodes": [],
        "macros": []
      },
      "description": "Orders table",
      "columns": {"id": {"name": "id", "description": "Order ID", "tags": []}}
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
      "depends_on": {
        "macros": []
      },
      "description": "Raw seed data",
      "columns": {"value": {"name": "value", "description": "Data value", "tags": []}}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {"type": "BASE TABLE", "schema": "public", "name": "orders", "database": "analytics"},
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "extra_col": {"type": "VARCHAR", "name": "extra_col", "index": 2}
      },
      "stats": {}
    },
    "seed.test_project.raw_data": {
      "unique_id": "seed.test_project.raw_data",
      "metadata": {"type": "BASE TABLE", "schema": "public", "name": "raw_data", "database": "analytics"},
      "columns": {
        "value": {"type": "INTEGER", "name": "value", "index": 1},
        "undocumented": {"type": "VARCHAR", "name": "undocumented", "index": 2}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should only fail for models, not seeds (due to applies_to filter)
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("extra_col"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_catalog_different_severities() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "depends_on": {
        "nodes": [],
        "macros": []
      },
      "description": "Orders table",
      "columns": {}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {"type": "BASE TABLE", "schema": "public", "name": "orders", "database": "analytics"},
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 2}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config_warning = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "warning"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_warning);
    let findings = env.run_catalog_checks(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");

    let config_error = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "error"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_error);
    let findings = env.run_catalog_checks(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_catalog_source_columns_documented() {
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
      },
      "depends_on": {
        "nodes": [],
        "macros": []
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.customers": {
      "unique_id": "source.test_project.raw_data.customers",
      "metadata": {"type": "BASE TABLE", "schema": "raw_data", "name": "customers", "database": "raw"},
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3}
      },
      "stats": {}
    }
  }
}"#;

    let config = r#"
catalog_tests:
  - name: "source_columns_documented"
    type: "columns_all_documented"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should fail: source has 3 columns but only 1 is documented
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0].0.message.contains("name") || findings[0].0.message.contains("email"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_all_documented() {
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
        "email": {"name": "email", "description": "Customer email", "tags": []}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.customers": {
      "unique_id": "model.test_project.customers",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "customers",
        "database": "analytics"
      },
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "name": {"type": "VARCHAR", "name": "name", "index": 2},
        "email": {"type": "VARCHAR", "name": "email", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    description: "All columns must have a description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should pass: all columns have descriptions
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_some_missing() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Orders table",
      "columns": {
        "order_id": {"name": "order_id", "description": "Order ID", "tags": []},
        "customer_id": {"name": "customer_id", "description": "", "tags": []},
        "amount": {"name": "amount", "description": "Order amount", "tags": []}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "orders",
        "database": "analytics"
      },
      "columns": {
        "order_id": {"type": "INTEGER", "name": "order_id", "index": 1},
        "customer_id": {"type": "INTEGER", "name": "customer_id", "index": 2},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    description: "All columns must have a description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should fail: customer_id has empty description
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "columns_have_description");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("customer_id"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_none_documented() {
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
    "model.test_project.products": {
      "database": "analytics",
      "schema": "public",
      "name": "products",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "products.sql",
      "original_file_path": "models/products.sql",
      "unique_id": "model.test_project.products",
      "fqn": ["test_project", "products"],
      "alias": "products",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Products table",
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
      "relation_name": "analytics.public.products",
      "raw_code": "select * from source_products",
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.products": {
      "unique_id": "model.test_project.products",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "products",
        "database": "analytics"
      },
      "columns": {
        "product_id": {"type": "INTEGER", "name": "product_id", "index": 1},
        "product_name": {"type": "VARCHAR", "name": "product_name", "index": 2},
        "price": {"type": "DECIMAL", "name": "price", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    description: "All columns must have a description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(true);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("products"));
    assert!(findings[0].0.message.contains("No columns"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_with_sources() {
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
        "created_at": {"name": "created_at", "description": "Creation timestamp", "tags": []}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {},
  "sources": {
    "source.test_project.raw_data.users": {
      "unique_id": "source.test_project.raw_data.users",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "raw_data",
        "name": "users",
        "database": "raw"
      },
      "columns": {
        "user_id": {"type": "INTEGER", "name": "user_id", "index": 1},
        "username": {"type": "VARCHAR", "name": "username", "index": 2},
        "created_at": {"type": "TIMESTAMP", "name": "created_at", "index": 3}
      },
      "stats": {}
    }
  }
}"#;

    let config = r#"
catalog_tests:
  - name: "source_columns_have_description"
    type: "columns_have_description"
    description: "All source columns must have a description"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should fail: username has empty description
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("users"));
    assert!(findings[0].0.message.contains("username"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_severity_warning() {
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
    "model.test_project.transactions": {
      "database": "analytics",
      "schema": "public",
      "name": "transactions",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "transactions.sql",
      "original_file_path": "models/transactions.sql",
      "unique_id": "model.test_project.transactions",
      "fqn": ["test_project", "transactions"],
      "alias": "transactions",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Transactions table",
      "columns": {
        "id": {"name": "id", "description": "Transaction ID", "tags": []},
        "amount": {"name": "amount", "description": "", "tags": []}
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
      "relation_name": "analytics.public.transactions",
      "raw_code": "select * from source_transactions",
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.transactions": {
      "unique_id": "model.test_project.transactions",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "transactions",
        "database": "analytics"
      },
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 2}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    description: "All columns must have a description"
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should produce warning, not error
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("transactions"));
    assert!(findings[0].0.message.contains("amount"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_have_description_applies_to_filter() {
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
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Orders table",
      "columns": {
        "id": {"name": "id", "description": "", "tags": []}
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

    let catalog = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  },
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "orders",
        "database": "analytics"
      },
      "columns": {
        "id": {"type": "INTEGER", "name": "id", "index": 1}
      },
      "stats": {}
    },
    "seed.test_project.raw_data": {
      "unique_id": "seed.test_project.raw_data",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "raw_data",
        "database": "analytics"
      },
      "columns": {
        "value": {"type": "INTEGER", "name": "value", "index": 1}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    description: "All columns must have a description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_checks(false);

    // Should only fail for models (not seeds) due to applies_to filter
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("id"));
}
