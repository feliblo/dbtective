mod common;

use common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_description_and_naming_convention_pass() {
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
    let findings = env.run_manifest_rules(false);

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
    assert_eq!(findings[1].0.rule_name, "naming_convention");
    assert!(findings[1].0.message.contains("orders"));
    assert!(findings[1].0.message.contains("PascalCase"));

    let exit_code = env.run_and_show_results(false);
    assert_eq!(exit_code, 1);
}

// ========================================================================
// Auto-parse Integration Tests
// ========================================================================

#[test]
fn test_run_with_auto_parse_and_configured_command() {
    use dbtective::cli::commands::{OutputFormat, RunOptions};
    use dbtective::core::artifacts::ArtifactFormat;
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create a manifest in the target/ subfolder
    let target_dir = temp_path.join("target");
    std::fs::create_dir(&target_dir).unwrap();

    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {},
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

    let manifest_path = target_dir.join("manifest.json");
    let mut mf = std::fs::File::create(&manifest_path).unwrap();
    mf.write_all(manifest.as_bytes()).unwrap();

    // Config with auto_parse_command (echo is a no-op that succeeds)
    let config = r#"
config:
  auto_parse_command: "echo auto-parse-executed"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to:
      - "models"
"#;
    let config_path = temp_path.join("dbtective.yml");
    let mut cf = std::fs::File::create(&config_path).unwrap();
    cf.write_all(config.as_bytes()).unwrap();

    let options = RunOptions {
        entry_point: temp_path.to_string_lossy().to_string(),
        config_file: None,
        manifest_file: "target/manifest.json".to_string(),
        catalog_file: "target/catalog.json".to_string(),
        index_dir: "target/index".to_string(),
        artifact_format: ArtifactFormat::Auto,
        only_manifest: true,
        disable_hyperlinks: true,
        hide_warnings: false,
        hide_catalog_tip: true,
        output_format: OutputFormat::Table,
        output_file: None,
        auto_parse: true,
    };

    // Should succeed (exit 0) since there are no models to fail on
    let exit_code = dbtective::core::run::run(&options, false);
    assert_eq!(exit_code, 0);
}

#[test]
fn test_run_without_auto_parse_does_not_run_command() {
    use dbtective::cli::commands::{OutputFormat, RunOptions};
    use dbtective::core::artifacts::ArtifactFormat;
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let target_dir = temp_path.join("target");
    std::fs::create_dir(&target_dir).unwrap();

    let manifest = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {},
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

    let manifest_path = target_dir.join("manifest.json");
    let mut mf = std::fs::File::create(&manifest_path).unwrap();
    mf.write_all(manifest.as_bytes()).unwrap();

    // Config with auto_parse_command pointing to a command that would FAIL
    // But since auto_parse is false, it should never be executed
    let config = r#"
config:
  auto_parse_command: "false"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to:
      - "models"
"#;
    let config_path = temp_path.join("dbtective.yml");
    let mut cf = std::fs::File::create(&config_path).unwrap();
    cf.write_all(config.as_bytes()).unwrap();

    let options = RunOptions {
        entry_point: temp_path.to_string_lossy().to_string(),
        config_file: None,
        manifest_file: "target/manifest.json".to_string(),
        catalog_file: "target/catalog.json".to_string(),
        index_dir: "target/index".to_string(),
        artifact_format: ArtifactFormat::Auto,
        only_manifest: true,
        disable_hyperlinks: true,
        hide_warnings: false,
        hide_catalog_tip: true,
        output_format: OutputFormat::Table,
        output_file: None,
        auto_parse: false,
    };

    // Should succeed because auto_parse is false, so the failing command is never run
    let exit_code = dbtective::core::run::run(&options, false);
    assert_eq!(exit_code, 0);
}
