use dbtective::cli::table::{show_results, RuleResult};
use dbtective::core::checks::manifest::node_checks::apply_node_checks;
use dbtective::core::checks::manifest::source_checks::apply_manifest_object_checks;
use dbtective::core::config::severity::Severity;
use dbtective::core::config::Config;
use dbtective::core::manifest::Manifest;
use std::io::Write;
use tempfile::TempDir;

struct TestEnvironment {
    temp_dir: TempDir,
    manifest_path: std::path::PathBuf,
    config_path: std::path::PathBuf,
}

impl TestEnvironment {
    fn new(manifest_json: &str, config_yaml: &str) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        // Write manifest.json
        let manifest_path = temp_path.join("manifest.json");
        let mut manifest_file =
            std::fs::File::create(&manifest_path).expect("Failed to create manifest file");
        manifest_file
            .write_all(manifest_json.as_bytes())
            .expect("Failed to write manifest");

        // Write config.yml
        let config_path = temp_path.join("config.yml");
        let mut config_file =
            std::fs::File::create(&config_path).expect("Failed to create config file");
        config_file
            .write_all(config_yaml.as_bytes())
            .expect("Failed to write config");

        Self {
            temp_dir,
            manifest_path,
            config_path,
        }
    }

    fn run_checks(&self, verbose: bool) -> Vec<(RuleResult, Severity)> {
        let manifest = Manifest::from_file(&self.manifest_path).expect("Failed to load manifest");
        let config = Config::from_file(&self.config_path).expect("Failed to load config");

        let mut findings =
            apply_node_checks(&manifest, &config, verbose).expect("Failed to apply node checks");
        findings.extend(
            apply_manifest_object_checks(&manifest, &config, verbose)
                .expect("Failed to apply source checks"),
        );

        // Convert from Vec<(RuleResult, &Severity)> to Vec<(RuleResult, Severity)>
        findings
            .into_iter()
            .map(|(result, severity)| (result, severity.clone()))
            .collect()
    }

    fn run_and_show_results(&self, verbose: bool) -> i32 {
        let manifest = Manifest::from_file(&self.manifest_path).expect("Failed to load manifest");
        let config = Config::from_file(&self.config_path).expect("Failed to load config");

        let mut findings =
            apply_node_checks(&manifest, &config, verbose).expect("Failed to apply node checks");
        findings.extend(
            apply_manifest_object_checks(&manifest, &config, verbose)
                .expect("Failed to apply source checks"),
        );

        show_results(
            &findings,
            verbose,
            self.temp_dir.path().to_str().unwrap(),
            None,
        )
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_model_missing_description_fails() {
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
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
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
      "relation_name": "analytics.public.orders",
      "raw_code": "select * from raw_orders",
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
  - name: "models_must_have_description"
    type: "has_description"
    severity: "error"
    description: "All models must have a description."
    applies_to:
      - "models"
  - name: "naming_convention"
    type: "name_convention"
    severity: "warning"
    description: "All nodes must follow the naming convention."
    pattern: "pascal_case"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_checks(false);

    // Error 1: orders model missing description (fail)
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "models_must_have_description");
    assert!(findings[0].0.message.contains("orders"));
    assert!(findings[0].0.message.contains("missing a description"));

    // Error 2: orders model name not in PascalCase (warn)
    assert_eq!(findings[1].0.severity, "WARN");
    assert_eq!(findings[1].0.object_type, "Model");
    assert_eq!(findings[1].0.rule_name, "name_convention");
    assert!(findings[1].0.message.contains("orders"));
    assert!(findings[1].0.message.contains("PascalCase"));

    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 1);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_all_checks_pass() {
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
    let findings = env.run_checks(false);

    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );

    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 0);
}

#[test]
fn test_invalid_regex_pattern_fails() {
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
      "config": {
        "enabled": true,
        "materialized": "view",
        "tags": []
      },
      "tags": [],
      "description": "Order fact table",
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
      "raw_code": "select * from raw_orders",
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
  - name: "naming_convention"
    type: "name_convention"
    severity: "error"
    description: "All nodes must follow the naming convention."
    pattern: "(*invalid_regex"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);

    // The check should fail with an error about invalid regex
    let manifest = Manifest::from_file(&env.manifest_path).expect("Failed to load manifest");
    let config = Config::from_file(&env.config_path).expect("Failed to load config");

    let result = apply_node_checks(&manifest, &config, false);
    assert!(
        result.is_err(),
        "Expected error for invalid regex pattern, but got success"
    );

    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("Invalid regex") || error_message.contains("regex"),
        "Error message should mention regex issue, got: {error_message}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_has_tags() {
    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {}
  },
  "nodes": {
    "model.test.model_with_tags": {
      "database": "db",
      "schema": "public",
      "name": "model_with_tags",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model.sql",
      "original_file_path": "models/model.sql",
      "unique_id": "model.test.model_with_tags",
      "fqn": ["test", "model_with_tags"],
      "alias": "model_with_tags",
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": ["tag1", "tag2"],
      "description": "Has tags",
      "columns": {},
      "meta": {}
    },
    "model.test.model_no_tags": {
      "database": "db",
      "schema": "public",
      "package_name": "test_project",
      "name": "model_no_tags",
      "resource_type": "model",
      "path": "model2.sql",
      "original_file_path": "models/model2.sql",
      "unique_id": "model.test.model_no_tags",
      "fqn": ["test", "model_no_tags"],
      "alias": "model_no_tags",
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": ["tag3"],
      "description": "No tags",
      "columns": {},
      "meta": {}
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

    let tag_available_config = r#"
manifest_tests:
  - name: "models_need_tag1"
    type: has_tags
    severity: "error"
    required_tags: ["tag1"]
    applies_to:
      - "models"
"#;

    // Failure: 2nd model is missing tag1
    let env = TestEnvironment::new(manifest, tag_available_config);
    let findings = env.run_checks(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "has_tags");
    assert!(findings[0].0.message.contains("model_no_tags"));

    // Success: both models have at least one of the required tags
    let tag_any_config = r#"
manifest_tests:
  - name: "models_need_tag1_or_tag3"
    type: has_tags
    severity: "error"
    required_tags: ["tag1", "tag3"]
    criteria: "any"
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(manifest, tag_any_config);
    let findings = env.run_checks(false);
    assert_eq!(findings.len(), 0);

    // OneOf: Failure, model one has both tags, model two has only one of the required tags
    // 2 fails because neither has "only one" of the required tags
    let tag_one_of_config = r#"
manifest_tests:
  - name: "models_need_tag1_and_tag2"
    type: has_tags
    severity: "warning"
    required_tags: ["tag1", "tag2"]
    criteria: "one_of"
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(manifest, tag_one_of_config);
    let findings = env.run_checks(false);
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "has_tags");
    assert_eq!(findings[1].0.severity, "WARN");
    assert_eq!(findings[1].0.object_type, "Model");
    assert_eq!(findings[1].0.rule_name, "has_tags");
    // Check both model names appear (order not guaranteed)
    let has_model_with_tags = findings
        .iter()
        .any(|f| f.0.message.contains("model_with_tags"));
    let has_model_no_tags = findings
        .iter()
        .any(|f| f.0.message.contains("model_no_tags"));
    assert!(has_model_with_tags);
    assert!(has_model_no_tags);
}
