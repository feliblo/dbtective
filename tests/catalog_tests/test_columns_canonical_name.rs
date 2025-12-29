use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_with_literal_match() {
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
        "customer_id": {"type": "INTEGER", "name": "customer_id", "index": 1},
        "cust_id": {"type": "INTEGER", "name": "cust_id", "index": 2},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "customer_id_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
      - "custid"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: cust_id should be customer_id
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "customer_id_canonical");
    assert!(findings[0].0.message.contains("customer_id"));
    assert!(findings[0].0.message.contains("cust_id"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_with_regex_match() {
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
    "model.test_project.addresses": {
      "database": "analytics",
      "schema": "public",
      "name": "addresses",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "addresses.sql",
      "original_file_path": "models/addresses.sql",
      "unique_id": "model.test_project.addresses",
      "fqn": ["test_project", "addresses"],
      "alias": "addresses",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "tags": [],
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "description": "Addresses table",
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
      "relation_name": "analytics.public.addresses",
      "raw_code": "select * from source_addresses",
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
    "model.test_project.addresses": {
      "unique_id": "model.test_project.addresses",
      "metadata": {
        "type": "BASE TABLE",
        "schema": "public",
        "name": "addresses",
        "database": "analytics"
      },
      "columns": {
        "zip_code": {"type": "VARCHAR", "name": "zip_code", "index": 1},
        "zipcode": {"type": "VARCHAR", "name": "zipcode", "index": 2},
        "postal_code": {"type": "VARCHAR", "name": "postal_code", "index": 3},
        "city": {"type": "VARCHAR", "name": "city", "index": 4}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "zip_code_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "zip_code"
    invalid_names:
      - "^zip"
      - "postal_code"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: zipcode and postal_code should be zip_code
    assert_eq!(findings.len(), 1, "findings: {:?}", findings);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert!(
        findings[0].0.message.contains("zip_code"),
        "message: {}",
        findings[0].0.message
    );
    assert!(
        findings[0].0.message.contains("zipcode"),
        "message: {}",
        findings[0].0.message
    );
    assert!(
        findings[0].0.message.contains("postal_code"),
        "message: {}",
        findings[0].0.message
    );
    // city should not be flagged
    assert!(!findings[0].0.message.contains("city"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_no_violations() {
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
        "customer_id": {"type": "INTEGER", "name": "customer_id", "index": 1},
        "order_id": {"type": "INTEGER", "name": "order_id", "index": 2},
        "amount": {"type": "DECIMAL", "name": "amount", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "customer_id_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
      - "custid"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should pass: no invalid column names present
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_case_insensitive_literal() {
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
        "USERID": {"type": "INTEGER", "name": "USERID", "index": 2},
        "UserId": {"type": "INTEGER", "name": "UserId", "index": 3}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "user_id_canonical"
    type: "columns_canonical_name"
    severity: "warning"
    canonical: "user_id"
    invalid_names:
      - "userid"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: USERID and UserId match "userid" case-insensitively
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert!(findings[0].0.message.contains("USERID") || findings[0].0.message.contains("UserId"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_with_sources() {
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
      "columns": {},
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
        "email_address": {"type": "VARCHAR", "name": "email_address", "index": 1},
        "emailaddress": {"type": "VARCHAR", "name": "emailaddress", "index": 2},
        "e_mail": {"type": "VARCHAR", "name": "e_mail", "index": 3}
      },
      "stats": {}
    }
  }
}"#;

    let config = r#"
catalog_tests:
  - name: "email_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "email_address"
    invalid_names:
      - "emailaddress"
      - "e_mail"
      - "regex:^email$"
    applies_to:
      - "sources"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should fail: emailaddress and e_mail should be email_address
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Source");
    assert!(findings[0].0.message.contains("emailaddress"));
    assert!(findings[0].0.message.contains("e_mail"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_multiple_rules() {
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
        "customer_id": {"type": "INTEGER", "name": "customer_id", "index": 1},
        "cust_id": {"type": "INTEGER", "name": "cust_id", "index": 2},
        "created_at": {"type": "TIMESTAMP", "name": "created_at", "index": 3},
        "createdat": {"type": "TIMESTAMP", "name": "createdat", "index": 4}
      },
      "stats": {}
    }
  },
  "sources": {}
}"#;

    let config = r#"
catalog_tests:
  - name: "customer_id_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
    applies_to:
      - "models"

  - name: "created_at_canonical"
    type: "columns_canonical_name"
    severity: "warning"
    canonical: "created_at"
    invalid_names:
      - "createdat"
      - "regex:^create_date"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should have 2 findings: one for each rule
    assert_eq!(findings.len(), 2);

    let customer_finding = findings
        .iter()
        .find(|f| f.0.rule_name == "customer_id_canonical");
    let created_finding = findings
        .iter()
        .find(|f| f.0.rule_name == "created_at_canonical");

    assert!(customer_finding.is_some());
    assert!(created_finding.is_some());

    assert_eq!(customer_finding.unwrap().0.severity, "FAIL");
    assert_eq!(created_finding.unwrap().0.severity, "WARN");
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_columns_canonical_name_applies_to_filter() {
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
      "depends_on": {"macros": [], "nodes": []}
    }
  },
  "sources": {
    "source.test_project.raw_data.orders": {
      "database": "raw",
      "schema": "raw_data",
      "name": "orders",
      "source_name": "raw_data",
      "source_description": "Raw data",
      "loader": "",
      "identifier": "orders",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [],
      "path": "models/sources.yml",
      "original_file_path": "models/sources.yml",
      "unique_id": "source.test_project.raw_data.orders",
      "fqn": ["test_project", "raw_data", "orders"],
      "config": {"enabled": true},
      "description": "Raw order data",
      "columns": {},
      "depends_on": {"nodes": [], "macros": []}
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
  "nodes": {
    "model.test_project.orders": {
      "unique_id": "model.test_project.orders",
      "metadata": {"type": "BASE TABLE", "schema": "public", "name": "orders", "database": "analytics"},
      "columns": {
        "cust_id": {"type": "INTEGER", "name": "cust_id", "index": 1}
      },
      "stats": {}
    }
  },
  "sources": {
    "source.test_project.raw_data.orders": {
      "unique_id": "source.test_project.raw_data.orders",
      "metadata": {"type": "BASE TABLE", "schema": "raw_data", "name": "orders", "database": "raw"},
      "columns": {
        "cust_id": {"type": "INTEGER", "name": "cust_id", "index": 1}
      },
      "stats": {}
    }
  }
}"#;

    let config = r#"
catalog_tests:
  - name: "customer_id_canonical"
    type: "columns_canonical_name"
    severity: "error"
    canonical: "customer_id"
    invalid_names:
      - "cust_id"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new_with_catalog(manifest, catalog, config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    // Should only fail for models, not sources (due to applies_to filter)
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Model");
}
