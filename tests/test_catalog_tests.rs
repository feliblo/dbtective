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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");

    let config_error = r#"
catalog_tests:
  - name: "columns_documented"
    type: "columns_all_documented"
    severity: "error"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_error);
    let findings = env.run_catalog_rules(false).expect("should not error");
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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

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
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should only fail for models (not seeds) due to applies_to filter
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("id"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_all_preset_cases() {
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
        "id": {"name": "id_snake_case", "description": "", "tags": []}
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
        "value": {"name": "value_snake_case", "description": "", "tags": []}
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
        "id_snake_case": {"type": "INTEGER", "name": "id_snake_case", "index": 1}
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
        "value_snake_case": {"type": "INTEGER", "name": "value_snake_case", "index": 1}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config_snake = r#"
catalog_tests:
  - name: "columns_follow_snakecase"
    type: "columns_name_convention"
    description: "All columns must follow snake_case naming convention"
    pattern: snake_case
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_snake);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should pass: all column names are in snake_case
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );

    let config_camel = r#"
catalog_tests:
  - name: "columns_follow_camelcase"
    type: "columns_name_convention"
    description: "All columns must follow camelCase naming convention"
    pattern: camelCase
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_camel);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: column names are not in camelCase
    assert_eq!(findings.len(), 2);
    for finding in findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert!(finding.0.message.contains("do not follow the camelCase"));
    }

    let config_pascal = r#"
    catalog_tests:
      - name: "columns_follow_pascalcase"
        type: "columns_name_convention"
        description: "All columns must follow PascalCase naming convention"
        pattern: PascalCase
    "#;
    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_pascal);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: column names are not in PascalCase
    assert_eq!(findings.len(), 2);
    for finding in findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert!(finding.0.message.contains("do not follow the PascalCase"));
    }

    let config_kebab = r#"
    catalog_tests:
      - name: "columns_follow_kebabcase"
        type: "columns_name_convention"
        description: "All columns must follow kebab-case naming convention"
        pattern: kebab-case
    "#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_kebab);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: column names are not in kebab-case
    assert_eq!(findings.len(), 2);
    for finding in findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert!(finding.0.message.contains("do not follow the kebab-case"));
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_custom_regex() {
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
        "id": {"name": "id_snake_case", "description": "", "tags": []}
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
        "value": {"name": "value_snake_case", "description": "", "tags": []}
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
        "id_snake_case": {"type": "INTEGER", "name": "id_snake_case", "index": 1}
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
        "value_snake_case": {"type": "INTEGER", "name": "value_snake_case", "index": 1}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config_custom_regex = r#"
catalog_tests:
  - name: "columns_follow_snakecase"
    type: "columns_name_convention"
    description: "All columns must follow snake_case naming convention"
    pattern: "^[a-z]+(_[a-z]+)*$"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_custom_regex);
    let findings = env.run_catalog_rules(false).expect("should not error");
    // Should pass: all column names match custom regex for snake_case
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );

    let config_custom_regex_no_match = r#"
catalog_tests:
  - name: "columns_start_with_col"
    type: "columns_name_convention"
    description: "All columns must start with 'col_'"
    pattern: "^col_.*$"
"#;
    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_custom_regex_no_match);
    let findings = env.run_catalog_rules(false).expect("should not error");
    // Should fail: column names do not start with 'col_'
    assert_eq!(findings.len(), 2);
    for finding in findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert!(finding.0.message.contains("do not follow the ^col_.*$"));
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_invalid_regex() {
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
        "id": {"name": "id_snake_case", "description": "", "tags": []}
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
        "value": {"name": "value_snake_case", "description": "", "tags": []}
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
        "id_snake_case": {"type": "INTEGER", "name": "id_snake_case", "index": 1}
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
        "value_snake_case": {"type": "INTEGER", "name": "value_snake_case", "index": 1}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config_invalid_regex = r#"
catalog_tests:
  - name: "columns_follow_snakecase"
    type: "columns_name_convention"
    description: "All columns must follow snake_case naming convention"
    pattern: "*[a-z]+(_[a-z]+)*$"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config_invalid_regex);
    let findings = env.run_catalog_rules(false);
    // Should raise anyhow an error due to invalid regex
    assert!(findings.is_err());
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_filter_by_single_data_type() {
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
    "model.test_project.users": {
      "database": "analytics",
      "schema": "public",
      "name": "users",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "users.sql",
      "original_file_path": "models/users.sql",
      "unique_id": "model.test_project.users",
      "fqn": ["test_project", "users"],
      "alias": "users",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Users table",
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
      "relation_name": "analytics.public.users",
      "raw_code": "select * from source_users",
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
    "model.test_project.users": {
      "unique_id": "model.test_project.users",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "users",
        "database": "analytics"
      },
      "columns": {
        "user_id": {"type": "INTEGER", "name": "user_id", "index": 1},
        "UserName": {"type": "VARCHAR", "name": "UserName", "index": 2},
        "created_at": {"type": "TIMESTAMP", "name": "created_at", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "varchar_columns_snake_case"
    type: "columns_name_convention"
    description: "VARCHAR columns must follow snake_case"
    pattern: snake_case
    data_types:
      - varchar
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: UserName (VARCHAR) violates snake_case, but user_id (INTEGER) and created_at (TIMESTAMP) are ignored
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("users"));
    assert!(findings[0].0.message.contains("UserName"));
    assert!(!findings[0].0.message.contains("user_id"));
    assert!(!findings[0].0.message.contains("created_at"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_filter_by_multiple_data_types() {
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
        "ProductName": {"type": "VARCHAR", "name": "ProductName", "index": 2},
        "Description": {"type": "TEXT", "name": "Description", "index": 3},
        "Price": {"type": "DECIMAL", "name": "Price", "index": 4}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "string_columns_snake_case"
    type: "columns_name_convention"
    description: "VARCHAR and TEXT columns must follow snake_case"
    pattern: snake_case
    data_types:
      - varchar
      - text
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: ProductName (VARCHAR) and Description (TEXT) violate snake_case
    // Price (DECIMAL) and product_id (INTEGER) should be ignored
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert!(findings[0].0.message.contains("ProductName"));
    assert!(findings[0].0.message.contains("Description"));
    assert!(!findings[0].0.message.contains("product_id"));
    assert!(!findings[0].0.message.contains("Price"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_filter_passes_when_filtered_columns_valid() {
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
        "OrderID": {"type": "INTEGER", "name": "OrderID", "index": 1},
        "customer_name": {"type": "VARCHAR", "name": "customer_name", "index": 2},
        "order_status": {"type": "VARCHAR", "name": "order_status", "index": 3},
        "CreatedAt": {"type": "TIMESTAMP", "name": "CreatedAt", "index": 4}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "varchar_columns_snake_case"
    type: "columns_name_convention"
    description: "VARCHAR columns must follow snake_case"
    pattern: snake_case
    data_types:
      - varchar
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should pass: customer_name and order_status (VARCHAR) follow snake_case
    // OrderID (INTEGER) and CreatedAt (TIMESTAMP) are ignored even though they violate the pattern
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_array_type_detection() {
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
    "model.test_project.events": {
      "database": "analytics",
      "schema": "public",
      "name": "events",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "events.sql",
      "original_file_path": "models/events.sql",
      "unique_id": "model.test_project.events",
      "fqn": ["test_project", "events"],
      "alias": "events",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Events table",
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
      "relation_name": "analytics.public.events",
      "raw_code": "select * from source_events",
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
    "model.test_project.events": {
      "unique_id": "model.test_project.events",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "events",
        "database": "analytics"
      },
      "columns": {
        "event_id": {"type": "INTEGER", "name": "event_id", "index": 1},
        "TagList": {"type": "ARRAY<VARCHAR>", "name": "TagList", "index": 2},
        "MetricValues": {"type": "ARRAY<INT>", "name": "MetricValues", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "array_columns_snake_case"
    type: "columns_name_convention"
    description: "ARRAY columns must follow snake_case"
    pattern: snake_case
    data_types:
      - array
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: TagList and MetricValues (both ARRAY types) violate snake_case
    // event_id (INTEGER) should be ignored
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert!(findings[0].0.message.contains("TagList"));
    assert!(findings[0].0.message.contains("MetricValues"));
    assert!(!findings[0].0.message.contains("event_id"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_warehouse_specific_aliases() {
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
    "model.test_project.metrics": {
      "database": "analytics",
      "schema": "public",
      "name": "metrics",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "metrics.sql",
      "original_file_path": "models/metrics.sql",
      "unique_id": "model.test_project.metrics",
      "fqn": ["test_project", "metrics"],
      "alias": "metrics",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Metrics table",
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
      "relation_name": "analytics.public.metrics",
      "raw_code": "select * from source_metrics",
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
    "model.test_project.metrics": {
      "unique_id": "model.test_project.metrics",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "metrics",
        "database": "analytics"
      },
      "columns": {
        "SmallCount": {"type": "int2", "name": "SmallCount", "index": 1},
        "RegularCount": {"type": "int4", "name": "RegularCount", "index": 2},
        "BigCount": {"type": "int8", "name": "BigCount", "index": 3},
        "user_name": {"type": "VARCHAR", "name": "user_name", "index": 4}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "integer_columns_pascal_case"
    type: "columns_name_convention"
    description: "Integer columns must follow PascalCase"
    pattern: PascalCase
    data_types:
      - small_int
      - integer
      - big_int
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should pass: SmallCount (int2->SmallInt), RegularCount (int4->Integer), BigCount (int8->BigInt) all follow PascalCase
    // user_name (VARCHAR) should be ignored
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_no_columns_match_filter() {
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
    "model.test_project.stats": {
      "database": "analytics",
      "schema": "public",
      "name": "stats",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "stats.sql",
      "original_file_path": "models/stats.sql",
      "unique_id": "model.test_project.stats",
      "fqn": ["test_project", "stats"],
      "alias": "stats",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Stats table",
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
      "relation_name": "analytics.public.stats",
      "raw_code": "select * from source_stats",
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
    "model.test_project.stats": {
      "unique_id": "model.test_project.stats",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "stats",
        "database": "analytics"
      },
      "columns": {
        "TotalCount": {"type": "INTEGER", "name": "TotalCount", "index": 1},
        "AverageValue": {"type": "DECIMAL", "name": "AverageValue", "index": 2},
        "CreatedDate": {"type": "DATE", "name": "CreatedDate", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "varchar_columns_snake_case"
    type: "columns_name_convention"
    description: "VARCHAR columns must follow snake_case"
    pattern: snake_case
    data_types:
      - varchar
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should pass: no VARCHAR columns exist, so no columns to check
    // Even though all columns violate snake_case, they're not VARCHAR
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_column_names_date_time_types_filter() {
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
    "model.test_project.events": {
      "database": "analytics",
      "schema": "public",
      "name": "events",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "events.sql",
      "original_file_path": "models/events.sql",
      "unique_id": "model.test_project.events",
      "fqn": ["test_project", "events"],
      "alias": "events",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Events table",
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
      "relation_name": "analytics.public.events",
      "raw_code": "select * from source_events",
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
    "model.test_project.events": {
      "unique_id": "model.test_project.events",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "events",
        "database": "analytics"
      },
      "columns": {
        "EventID": {"type": "INTEGER", "name": "EventID", "index": 1},
        "EventName": {"type": "VARCHAR", "name": "EventName", "index": 2},
        "created_at": {"type": "TIMESTAMP", "name": "created_at", "index": 3},
        "event_date": {"type": "DATE", "name": "event_date", "index": 4},
        "UpdatedAt": {"type": "TIMESTAMPTZ", "name": "UpdatedAt", "index": 5}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "datetime_columns_snake_case"
    type: "columns_name_convention"
    description: "Date/time columns must follow snake_case"
    pattern: snake_case
    data_types:
      - date
      - timestamp
      - timestamptz
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: UpdatedAt (TIMESTAMPTZ) violates snake_case
    // created_at (TIMESTAMP) and event_date (DATE) follow snake_case
    // EventID (INTEGER) and EventName (VARCHAR) should be ignored
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert!(findings[0].0.message.contains("UpdatedAt"));
    assert!(!findings[0].0.message.contains("created_at"));
    assert!(!findings[0].0.message.contains("event_date"));
    assert!(!findings[0].0.message.contains("EventID"));
    assert!(!findings[0].0.message.contains("EventName"));
}
