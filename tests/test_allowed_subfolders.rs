mod common;

use common::TestEnvironment;

#[test]
fn test_allowed_subfolder_pass() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.stg_orders": {
      "name": "stg_orders",
      "resource_type": "model",
      "path": "models/bronze/source1/stg_orders.sql",
      "original_file_path": "models/bronze/source1/stg_orders.sql",
      "unique_id": "model.test.stg_orders",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
      - "source2"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Should pass - model is in allowed subfolder source1"
    );
}

#[test]
fn test_allowed_subfolder_fail() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.stg_orders": {
      "name": "stg_orders",
      "resource_type": "model",
      "path": "models/bronze/source3/stg_orders.sql",
      "original_file_path": "models/bronze/source3/stg_orders.sql",
      "unique_id": "model.test.stg_orders",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
      - "source2"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1, "Should fail - source3 is not allowed");
    let finding = &findings[0].0;
    assert!(
        finding.message.contains("source3"),
        "Error message should mention source3"
    );
}

#[test]
fn test_with_path_prefix() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.stg_orders": {
      "name": "stg_orders",
      "resource_type": "model",
      "path": "dbt/models/bronze/source1/stg_orders.sql",
      "original_file_path": "dbt/models/bronze/source1/stg_orders.sql",
      "unique_id": "model.test.stg_orders",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_prefix: "dbt"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0, "Should pass with path_prefix");
}

#[test]
fn test_no_prefix_or_postfix() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.dim_customers": {
      "name": "dim_customers",
      "resource_type": "model",
      "path": "models/bronze/dim_customers.sql",
      "original_file_path": "models/bronze/dim_customers.sql",
      "unique_id": "model.test.dim_customers",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    allowed_subfolders:
      - "bronze"
      - "silver"
      - "gold"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 0, "Should pass - bronze is allowed");
}

#[test]
fn test_directly_in_base_path_fails() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models/bronze/model.sql",
      "original_file_path": "models/bronze/model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
      - "source2"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Should fail - file is directly in bronze without subfolder"
    );
    let finding = &findings[0].0;
    assert!(
        finding.message.contains("directly in"),
        "Error message should mention 'directly in'"
    );
}

#[test]
fn test_deep_nested_path() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models/bronze/source1/subdir/deeper/model.sql",
      "original_file_path": "models/bronze/source1/subdir/deeper/model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Should pass - deep nesting under allowed subfolder is ok"
    );
}

#[test]
fn test_outside_base_path_ignored() {
    // Files outside the base path should be ignored
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models/silver/source1/model.sql",
      "original_file_path": "models/silver/source1/model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Should pass - models/silver/ is not in base path (models/bronze/)"
    );
}

#[test]
fn test_intermediate_and_marts_ignored_when_rule_is_for_staging() {
    // Test the medallion architecture scenario: rule applies to staging/, not intermediate/ or marts/
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.stg_customers": {
      "name": "stg_customers",
      "resource_type": "model",
      "path": "models/staging/bronze/stg_customers.sql",
      "original_file_path": "models/staging/bronze/stg_customers.sql",
      "unique_id": "model.test.stg_customers",
      "description": "Staging model - should pass",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    },
    "model.test.int_model": {
      "name": "int_model",
      "resource_type": "model",
      "path": "models/intermediate/finance/int_model.sql",
      "original_file_path": "models/intermediate/finance/int_model.sql",
      "unique_id": "model.test.int_model",
      "description": "Intermediate model - should be ignored",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    },
    "model.test.customers": {
      "name": "customers",
      "resource_type": "model",
      "path": "models/marts/finance/customers.sql",
      "original_file_path": "models/marts/finance/customers.sql",
      "unique_id": "model.test.customers",
      "description": "Marts model - should be ignored",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "medallion_structure"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "staging"
    allowed_subfolders:
      - "bronze"
      - "silver"
      - "gold"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Should pass - only staging/bronze/ model should be checked, intermediate/ and marts/ should be ignored"
    );
}

#[test]
fn test_staging_with_wrong_subfolder_fails() {
    // Test that models in staging/ but with wrong subfolders fail
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.stg_payments": {
      "name": "stg_payments",
      "resource_type": "model",
      "path": "models/staging/crm/stg_payments.sql",
      "original_file_path": "models/staging/crm/stg_payments.sql",
      "unique_id": "model.test.stg_payments",
      "description": "Staging model in wrong subfolder",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    },
    "model.test.int_model": {
      "name": "int_model",
      "resource_type": "model",
      "path": "models/intermediate/finance/int_model.sql",
      "original_file_path": "models/intermediate/finance/int_model.sql",
      "unique_id": "model.test.int_model",
      "description": "Intermediate model - should be ignored",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

    let config = r#"
manifest_tests:
  - name: "medallion_structure"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "staging"
    allowed_subfolders:
      - "bronze"
      - "silver"
      - "gold"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Should fail - stg_payments is in staging/crm/ which is not allowed"
    );
    let finding = &findings[0].0;
    assert!(
        finding.message.contains("crm"),
        "Error message should mention the wrong subfolder 'crm'"
    );
    assert!(
        finding.message.contains("bronze, silver, gold"),
        "Error message should mention allowed subfolders"
    );
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use super::*;

    #[test]
    fn test_windows_backslash_paths() {
        let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models\\bronze\\source1\\model.sql",
      "original_file_path": "models\\bronze\\source1\\model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

        let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
"#;

        let env = TestEnvironment::new(manifest, config);
        let findings = env.run_manifest_rules(false);

        assert_eq!(
            findings.len(),
            0,
            "Windows backslash paths should be normalized and pass"
        );
    }

    #[test]
    fn test_windows_double_slashes() {
        let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models\\\\bronze\\\\source1\\\\model.sql",
      "original_file_path": "models\\\\bronze\\\\source1\\\\model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

        let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
"#;

        let env = TestEnvironment::new(manifest, config);
        let findings = env.run_manifest_rules(false);

        assert_eq!(
            findings.len(),
            0,
            "Double slashes should be collapsed and normalized"
        );
    }

    #[test]
    fn test_windows_wrong_subfolder_fails() {
        let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  },
  "nodes": {
    "model.test.model": {
      "name": "model",
      "resource_type": "model",
      "path": "models\\bronze\\source3\\model.sql",
      "original_file_path": "models\\bronze\\source3\\model.sql",
      "unique_id": "model.test.model",
      "description": "Test model",
      "config": {"materialized": "view"},
      "raw_code": "SELECT 1"
    }
  },
  "sources": {},
  "macros": {},
  "exposures": {},
  "semantic_models": {},
  "unit_tests": {},
  "parent_map": {},
  "child_map": {}
}"#;

        let config = r#"
manifest_tests:
  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    severity: "error"
    applies_to:
      - "models"
    path_postfix: "bronze"
    allowed_subfolders:
      - "source1"
      - "source2"
"#;

        let env = TestEnvironment::new(manifest, config);
        let findings = env.run_manifest_rules(false);

        assert_eq!(
            findings.len(),
            1,
            "Windows paths: wrong subfolder should fail"
        );
        let finding = &findings[0].0;
        assert!(
            finding.message.contains("source3"),
            "Error message should mention source3"
        );
    }
}
