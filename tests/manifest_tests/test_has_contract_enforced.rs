use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_contract_enforced() {
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
        "model.test_project.contract_enforced": {
            "database": "analytics",
            "schema": "public",
            "name": "contract_enforced",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "customers.sql",
            "original_file_path": "models/customers.sql",
            "unique_id": "model.test_project.contract_enforced",
            "fqn": [
                "test_project",
                "customers"
            ],
            "alias": "customers",
            "checksum": {
                "name": "sha256",
                "checksum": "abc123"
            },
            "config": {
                "materialized": "table",
                "tags": [
                    "finance"
                ],
                "contract": {
                    "enforced": true,
                    "alias_types": true
                }
            },
            "tags": [
                "finance"
            ],
            "depends_on": {
                "macros": [],
                "nodes": []
            },
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
            }
        },
        "model.test_project.contract_not_enforced": {
            "database": "analytics",
            "schema": "public",
            "name": "contract_not_enforced",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "customers.sql",
            "original_file_path": "models/customers.sql",
            "unique_id": "model.test_project.contract_not_enforced",
            "fqn": [
                "test_project",
                "customers"
            ],
            "alias": "customers",
            "checksum": {
                "name": "sha256",
                "checksum": "abc123"
            },
            "config": {
                "enabled": true,
                "materialized": "table",
                "tags": [
                    "finance"
                ],
                "contract": {
                    "enforced": false,
                    "alias_types": true
                }
            },
            "depends_on": {
                "macros": [],
                "nodes": []
            },
            "tags": [
                "finance"
            ],
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
            }
        },
        "model.test_project.model_no_config": {
            "database": "analytics",
            "schema": "public",
            "name": "model_no_config",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "customers.sql",
            "original_file_path": "models/customers.sql",
            "unique_id": "model.test_project.model_no_config",
            "fqn": [
                "test_project",
                "customers"
            ],
            "alias": "customers",
            "checksum": {
                "name": "sha256",
                "checksum": "abc123"
            },
            "tags": [
                "finance"
            ],
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
            "docs": {
                "show": true
            },
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
            "depends_on": {
                "macros": [],
                "nodes": []
            },
            "compiled_code": null,
            "extra_ctes_injected": false,
            "extra_ctes": [],
            "contract": {
                "enforced": false,
                "checksum": null
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

    let config = r#"
manifest_tests:
  - name: "models_have_contract_enforced"
    type: has_contract_enforced
    severity: "error"
    description: "All models must have a contract enforced."
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 2);
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("model_no_config") }));

    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("contract_not_enforced") }));

    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 1);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_contract_enforced_with_access_level() {
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
        "model.test_project.public_with_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "public_with_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "public_with_contract.sql",
            "original_file_path": "models/public_with_contract.sql",
            "unique_id": "model.test_project.public_with_contract",
            "fqn": ["test_project", "public_with_contract"],
            "alias": "public_with_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": true, "alias_types": true }
            },
            "access": "public",
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Public model with contract"
        },
        "model.test_project.public_without_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "public_without_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "public_without_contract.sql",
            "original_file_path": "models/public_without_contract.sql",
            "unique_id": "model.test_project.public_without_contract",
            "fqn": ["test_project", "public_without_contract"],
            "alias": "public_without_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "access": "public",
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Public model without contract"
        },
        "model.test_project.protected_without_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "protected_without_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "protected_without_contract.sql",
            "original_file_path": "models/protected_without_contract.sql",
            "unique_id": "model.test_project.protected_without_contract",
            "fqn": ["test_project", "protected_without_contract"],
            "alias": "protected_without_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "access": "protected",
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Protected model without contract"
        },
        "model.test_project.no_access_without_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "no_access_without_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "no_access_without_contract.sql",
            "original_file_path": "models/no_access_without_contract.sql",
            "unique_id": "model.test_project.no_access_without_contract",
            "fqn": ["test_project", "no_access_without_contract"],
            "alias": "no_access_without_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Model without access set and without contract"
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

    // Only public models should be checked
    let config = r#"
manifest_tests:
  - name: "public_models_have_contract_enforced"
    type: has_contract_enforced
    access_level: "public"
    severity: "error"
    description: "Public models must have a contract enforced."
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // Only public_without_contract should fail; protected and no-access models are skipped
    assert_eq!(findings.len(), 1);
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("public_without_contract") }));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_contract_enforced_with_common_includes() {
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
        "model.test_project.marts_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "marts_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "marts/marts_no_contract.sql",
            "original_file_path": "models/marts/marts_no_contract.sql",
            "unique_id": "model.test_project.marts_no_contract",
            "fqn": ["test_project", "marts", "marts_no_contract"],
            "alias": "marts_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Marts model without contract"
        },
        "model.test_project.intermediate_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "intermediate_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "intermediate/intermediate_no_contract.sql",
            "original_file_path": "models/intermediate/intermediate_no_contract.sql",
            "unique_id": "model.test_project.intermediate_no_contract",
            "fqn": ["test_project", "intermediate", "intermediate_no_contract"],
            "alias": "intermediate_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Intermediate model without contract"
        },
        "model.test_project.staging_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "staging_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "staging/staging_no_contract.sql",
            "original_file_path": "models/staging/staging_no_contract.sql",
            "unique_id": "model.test_project.staging_no_contract",
            "fqn": ["test_project", "staging", "staging_no_contract"],
            "alias": "staging_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Staging model without contract"
        },
        "model.test_project.marts_with_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "marts_with_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "marts/marts_with_contract.sql",
            "original_file_path": "models/marts/marts_with_contract.sql",
            "unique_id": "model.test_project.marts_with_contract",
            "fqn": ["test_project", "marts", "marts_with_contract"],
            "alias": "marts_with_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": true, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Marts model with contract"
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

    // Common data model includes: only marts and intermediate
    let config = r#"
manifest_tests:
  - name: "has_contract_enforced"
    type: has_contract_enforced
    includes: ["models/marts", "models/intermediate"]
    severity: "error"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // marts_no_contract and intermediate_no_contract should fail
    // staging_no_contract should be skipped (not in includes)
    // marts_with_contract should pass
    assert_eq!(findings.len(), 2);
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("marts_no_contract") }));
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("intermediate_no_contract") }));
    assert!(!findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("staging_no_contract") }));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_contract_enforced_with_medallion_includes() {
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
        "model.test_project.silver_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "silver_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "silver/silver_no_contract.sql",
            "original_file_path": "models/silver/silver_no_contract.sql",
            "unique_id": "model.test_project.silver_no_contract",
            "fqn": ["test_project", "silver", "silver_no_contract"],
            "alias": "silver_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Silver model without contract"
        },
        "model.test_project.gold_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "gold_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "gold/gold_no_contract.sql",
            "original_file_path": "models/gold/gold_no_contract.sql",
            "unique_id": "model.test_project.gold_no_contract",
            "fqn": ["test_project", "gold", "gold_no_contract"],
            "alias": "gold_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Gold model without contract"
        },
        "model.test_project.bronze_no_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "bronze_no_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "bronze/bronze_no_contract.sql",
            "original_file_path": "models/bronze/bronze_no_contract.sql",
            "unique_id": "model.test_project.bronze_no_contract",
            "fqn": ["test_project", "bronze", "bronze_no_contract"],
            "alias": "bronze_no_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": false, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Bronze model without contract"
        },
        "model.test_project.gold_with_contract": {
            "database": "analytics",
            "schema": "public",
            "name": "gold_with_contract",
            "resource_type": "model",
            "package_name": "test_project",
            "path": "gold/gold_with_contract.sql",
            "original_file_path": "models/gold/gold_with_contract.sql",
            "unique_id": "model.test_project.gold_with_contract",
            "fqn": ["test_project", "gold", "gold_with_contract"],
            "alias": "gold_with_contract",
            "checksum": { "name": "sha256", "checksum": "abc123" },
            "config": {
                "materialized": "table",
                "contract": { "enforced": true, "alias_types": true }
            },
            "tags": [],
            "depends_on": { "macros": [], "nodes": [] },
            "description": "Gold model with contract"
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

    // Medallion data model includes: only silver and gold
    let config = r#"
manifest_tests:
  - name: "has_contract_enforced"
    type: has_contract_enforced
    includes: ["models/silver", "models/gold"]
    severity: "error"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_maniest_rules(false);

    // silver_no_contract and gold_no_contract should fail
    // bronze_no_contract should be skipped (not in includes)
    // gold_with_contract should pass
    assert_eq!(findings.len(), 2);
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("silver_no_contract") }));
    assert!(findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("gold_no_contract") }));
    assert!(!findings
        .iter()
        .any(|(finding, _)| { finding.message.contains("bronze_no_contract") }));
}
