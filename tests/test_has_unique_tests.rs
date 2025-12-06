mod common;

use common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_without_unique_test_fails() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data without unique test",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0].0.message.contains("should have a unique test"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_with_unique_test_passes() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data with unique test",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "test.test_project.unique_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_customers_customer_id",
      "fqn": ["test_project", "staging", "unique_customers_customer_id"],
      "alias": "unique_customers_customer_id",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {
          "column_name": "customer_id",
          "model": "{{ get_where_subquery(ref('customers')) }}"
        }
      },
      "attached_node": "model.test_project.customers"
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
    "test.test_project.unique_customers_customer_id": ["model.test_project.customers"]
  },
  "child_map": {
    "model.test_project.customers": ["test.test_project.unique_customers_customer_id"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: model has a unique test
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_with_dbt_utils_unique_combination_passes() {
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
    "model.test_project.order_items": {
      "database": "analytics",
      "schema": "staging",
      "name": "order_items",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/order_items.sql",
      "original_file_path": "models/staging/order_items.sql",
      "unique_id": "model.test_project.order_items",
      "fqn": ["test_project", "staging", "order_items"],
      "alias": "order_items",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Order items with composite key",
      "columns": {
        "order_id": {"name": "order_id", "description": "Order ID", "tags": []},
        "item_id": {"name": "item_id", "description": "Item ID", "tags": []}
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
      "relation_name": "analytics.staging.order_items",
      "raw_code": "select * from raw.order_items",
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
    "test.test_project.unique_combination_order_items": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "dbt_utils.unique_combination_of_columns",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_combination_order_items.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_combination_order_items",
      "fqn": ["test_project", "staging", "unique_combination_order_items"],
      "alias": "unique_combination_order_items",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["order_items"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.order_items"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "dbt_utils.unique_combination_of_columns",
        "kwargs": {
          "combination_of_columns": ["order_id", "item_id"],
          "model": "{{ get_where_subquery(ref('order_items')) }}"
        }
      },
      "attached_node": "model.test_project.order_items"
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
    "test.test_project.unique_combination_order_items": ["model.test_project.order_items"]
  },
  "child_map": {
    "model.test_project.order_items": ["test.test_project.unique_combination_order_items"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: model has dbt_utils.unique_combination_of_columns test
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_with_only_non_unique_test_fails() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data with only not_null test",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "test.test_project.not_null_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "not_null_customers_customer_id",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "not_null_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.not_null_customers_customer_id",
      "fqn": ["test_project", "staging", "not_null_customers_customer_id"],
      "alias": "not_null_customers_customer_id",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "not_null",
        "kwargs": {
          "column_name": "customer_id",
          "model": "{{ get_where_subquery(ref('customers')) }}"
        }
      },
      "attached_node": "model.test_project.customers"
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
    "test.test_project.not_null_customers_customer_id": ["model.test_project.customers"]
  },
  "child_map": {
    "model.test_project.customers": ["test.test_project.not_null_customers_customer_id"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0].0.message.contains("should have a unique test"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_with_multiple_tests_including_unique_passes() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data with multiple tests",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "test.test_project.not_null_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "not_null_customers_customer_id",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "not_null_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.not_null_customers_customer_id",
      "fqn": ["test_project", "staging", "not_null_customers_customer_id"],
      "alias": "not_null_customers_customer_id",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "not_null",
        "kwargs": {
          "column_name": "customer_id",
          "model": "{{ get_where_subquery(ref('customers')) }}"
        }
      },
      "attached_node": "model.test_project.customers"
    },
    "test.test_project.unique_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_customers_customer_id",
      "fqn": ["test_project", "staging", "unique_customers_customer_id"],
      "alias": "unique_customers_customer_id",
      "checksum": {"name": "sha256", "checksum": "ghi789"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {
          "column_name": "customer_id",
          "model": "{{ get_where_subquery(ref('customers')) }}"
        }
      },
      "attached_node": "model.test_project.customers"
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
    "test.test_project.not_null_customers_customer_id": ["model.test_project.customers"],
    "test.test_project.unique_customers_customer_id": ["model.test_project.customers"]
  },
  "child_map": {
    "model.test_project.customers": [
      "test.test_project.not_null_customers_customer_id",
      "test.test_project.unique_customers_customer_id"
    ]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: model has both not_null and unique tests
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_custom_allowed_test_names() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data with custom unique test",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "test.test_project.custom_uniqueness_test": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "my_custom_unique_test",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "custom_uniqueness_test.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.custom_uniqueness_test",
      "fqn": ["test_project", "staging", "custom_uniqueness_test"],
      "alias": "custom_uniqueness_test",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "my_custom_unique_test",
        "kwargs": {
          "column_name": "customer_id",
          "model": "{{ get_where_subquery(ref('customers')) }}"
        }
      },
      "attached_node": "model.test_project.customers"
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
    "test.test_project.custom_uniqueness_test": ["model.test_project.customers"]
  },
  "child_map": {
    "model.test_project.customers": ["test.test_project.custom_uniqueness_test"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
    allowed_test_names:
      - "my_custom_unique_test"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: model has custom unique test that's in allowed_test_names
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_mixed_scenario_multiple_models() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Has unique test",
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "model.test_project.orders": {
      "database": "analytics",
      "schema": "staging",
      "name": "orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/orders.sql",
      "original_file_path": "models/staging/orders.sql",
      "unique_id": "model.test_project.orders",
      "fqn": ["test_project", "staging", "orders"],
      "alias": "orders",
      "checksum": {"name": "sha256", "checksum": "def456"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Missing unique test",
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
      "relation_name": "analytics.staging.orders",
      "raw_code": "select * from raw.orders",
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
    "model.test_project.products": {
      "database": "analytics",
      "schema": "staging",
      "name": "products",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/products.sql",
      "original_file_path": "models/staging/products.sql",
      "unique_id": "model.test_project.products",
      "fqn": ["test_project", "staging", "products"],
      "alias": "products",
      "checksum": {"name": "sha256", "checksum": "ghi789"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Has dbt_utils unique test",
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
      "relation_name": "analytics.staging.products",
      "raw_code": "select * from raw.products",
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
    "test.test_project.unique_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_customers_customer_id",
      "fqn": ["test_project", "staging", "unique_customers_customer_id"],
      "alias": "unique_customers_customer_id",
      "checksum": {"name": "sha256", "checksum": "test1"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {}
      },
      "attached_node": "model.test_project.customers"
    },
    "test.test_project.not_null_orders_order_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "not_null_orders_order_id",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "not_null_orders_order_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.not_null_orders_order_id",
      "fqn": ["test_project", "staging", "not_null_orders_order_id"],
      "alias": "not_null_orders_order_id",
      "checksum": {"name": "sha256", "checksum": "test2"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [["orders"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.orders"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "not_null",
        "kwargs": {}
      },
      "attached_node": "model.test_project.orders"
    },
    "test.test_project.unique_products": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "dbt_utils.unique_combination_of_columns",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_products.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_products",
      "fqn": ["test_project", "staging", "unique_products"],
      "alias": "unique_products",
      "checksum": {"name": "sha256", "checksum": "test3"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [["products"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.products"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "dbt_utils.unique_combination_of_columns",
        "kwargs": {}
      },
      "attached_node": "model.test_project.products"
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
    "test.test_project.unique_customers_customer_id": ["model.test_project.customers"],
    "test.test_project.not_null_orders_order_id": ["model.test_project.orders"],
    "test.test_project.unique_products": ["model.test_project.products"]
  },
  "child_map": {
    "model.test_project.customers": ["test.test_project.unique_customers_customer_id"],
    "model.test_project.orders": ["test.test_project.not_null_orders_order_id"],
    "model.test_project.products": ["test.test_project.unique_products"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should find 1 failure: orders model (missing unique test)
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("should have a unique test"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_source_with_unique_test_passes() {
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
    "test.test_project.unique_raw_customers_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_raw_customers_id.sql",
      "original_file_path": "models/sources.yml",
      "unique_id": "test.test_project.unique_raw_customers_id",
      "fqn": ["test_project", "unique_raw_customers_id"],
      "alias": "unique_raw_customers_id",
      "checksum": {"name": "sha256", "checksum": "test1"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [],
      "sources": [["raw_data", "customers"]],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["source.test_project.raw_data.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {}
      },
      "attached_node": "source.test_project.raw_data.customers"
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
    "test.test_project.unique_raw_customers_id": ["source.test_project.raw_data.customers"]
  },
  "child_map": {
    "source.test_project.raw_data.customers": ["test.test_project.unique_raw_customers_id"]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "sources_should_have_unique_test"
    type: "has_unique_test"
    description: "All sources should have a unique test"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: source has a unique test
    assert_eq!(findings.len(), 0);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_source_without_unique_test_fails() {
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
  "child_map": {},
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "sources_should_have_unique_test"
    type: "has_unique_test"
    description: "All sources should have a unique test"
    severity: "error"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("customers"));
    assert!(findings[0].0.message.contains("should have a unique test"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_with_two_unique_tests_passes() {
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
      "schema": "staging",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "staging/customers.sql",
      "original_file_path": "models/staging/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "staging", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Customer data with two unique tests",
      "columns": {
        "customer_id": {"name": "customer_id", "description": "Primary key", "tags": []},
        "email": {"name": "email", "description": "Email", "tags": []}
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
      "relation_name": "analytics.staging.customers",
      "raw_code": "select * from raw.customers",
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
    "test.test_project.unique_customers_customer_id": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_customers_customer_id.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_customers_customer_id",
      "fqn": ["test_project", "staging", "unique_customers_customer_id"],
      "alias": "unique_customers_customer_id",
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {}
      },
      "attached_node": "model.test_project.customers"
    },
    "test.test_project.unique_customers_email": {
      "database": "analytics",
      "schema": "dbt_test__audit",
      "name": "unique",
      "resource_type": "test",
      "package_name": "test_project",
      "path": "unique_customers_email.sql",
      "original_file_path": "models/staging/schema.yml",
      "unique_id": "test.test_project.unique_customers_email",
      "fqn": ["test_project", "staging", "unique_customers_email"],
      "alias": "unique_customers_email",
      "checksum": {"name": "sha256", "checksum": "ghi789"},
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
      "raw_code": "",
      "language": "sql",
      "refs": [["customers"]],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": ["model.test_project.customers"]},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null},
      "test_metadata": {
        "name": "unique",
        "kwargs": {}
      },
      "attached_node": "model.test_project.customers"
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
    "test.test_project.unique_customers_customer_id": ["model.test_project.customers"],
    "test.test_project.unique_customers_email": ["model.test_project.customers"]
  },
  "child_map": {
    "model.test_project.customers": [
      "test.test_project.unique_customers_customer_id",
      "test.test_project.unique_customers_email"
    ]
  },
  "group_map": {},
  "saved_queries": {},
  "semantic_models": {},
  "unit_tests": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Should pass: model has two unique tests (one for customer_id and one for email)
    assert_eq!(findings.len(), 0);
}
