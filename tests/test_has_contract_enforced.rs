mod common;

use common::TestEnvironment;

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
    let findings = env.run_checks(false);

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
