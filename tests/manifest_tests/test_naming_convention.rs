use crate::common::TestEnvironment;
use dbtective::core::config::Config;

/// Test that invalid regex patterns are caught during config parsing.
/// This validates that regex validation happens early (at config load time)
/// rather than at rule execution time, providing faster feedback to users.
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

    // The config loading should fail with an error about invalid regex
    // (regex patterns are now validated at config parse time)
    let result = Config::from_file(&env.config_path);
    assert!(
        result.is_err(),
        "Expected error for invalid regex pattern during config loading, but got success"
    );

    let error_message = result.unwrap_err().to_string();
    assert!(
        error_message.contains("Invalid regex") || error_message.contains("regex"),
        "Error message should mention regex issue, got: {error_message}"
    );
}
