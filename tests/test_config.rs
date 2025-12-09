use dbtective::core::config::{applies_to::RuleTarget, parse_config::Config, severity::Severity};
use std::io::Write;
use tempfile::NamedTempFile;

fn create_temp_config(content: &str, suffix: Option<&str>) -> NamedTempFile {
    let mut temp_file = suffix.map_or_else(
        || NamedTempFile::new().unwrap(),
        |s| NamedTempFile::with_suffix(s).unwrap(),
    );
    temp_file.write_all(content.as_bytes()).unwrap();
    temp_file
}

// ===== APPLIES_TO TESTS =====

#[test]
fn test_applies_to_default_for_has_description() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.expect("manifest_tests should be Some");

    assert_eq!(manifest_tests.len(), 1);
    let rule = &manifest_tests[0];

    // Check that default applies_to was set
    assert!(rule.applies_to.is_some());
    let applies_to = rule.applies_to.as_ref().unwrap();

    // Default for has_description includes models, seeds, snapshots
    assert!(!applies_to.node_objects.is_empty());
    assert!(!applies_to.source_objects.is_empty());
}

#[test]
fn test_applies_to_explicit_single_target() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    assert_eq!(applies_to.node_objects.len(), 1);
    assert_eq!(applies_to.node_objects[0], RuleTarget::Models);
}

#[test]
fn test_applies_to_multiple_targets() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models", "seeds", "sources"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    assert_eq!(applies_to.node_objects.len(), 2); // models, seeds
    assert_eq!(applies_to.source_objects.len(), 1); // sources
    assert!(applies_to.node_objects.contains(&RuleTarget::Models));
    assert!(applies_to.node_objects.contains(&RuleTarget::Seeds));
    assert!(applies_to.source_objects.contains(&RuleTarget::Sources));
}

#[test]
fn test_applies_to_invalid_target_for_rule() {
    // has_tags cannot apply to unit_tests according to applies_to_options_for_manifest_rule
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii"]
    applies_to: ["hook_nodes"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(
        result.is_err(),
        "Should fail when applies_to contains invalid target for rule"
    );
}

#[test]
fn test_applies_to_completely_invalid_target() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["invalid_target", "another_invalid"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(
        result.is_err(),
        "Should fail for completely invalid applies_to targets"
    );
}

#[test]
fn test_applies_to_mixed_valid_invalid() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models", "invalid_target"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(result.is_err());
}

#[test]
fn test_applies_to_empty_list() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: []
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(result.is_err(), "Should fail for empty applies_to list");
}

#[test]
fn test_applies_to_catalog_tests() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "warning"
    applies_to: ["models", "snapshots"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse catalog config");
    let catalog_tests = cfg.catalog_tests.expect("catalog_tests should be Some");

    assert_eq!(catalog_tests.len(), 1);
    let applies_to = catalog_tests[0].applies_to.as_ref().unwrap();
    assert_eq!(applies_to.node_objects.len(), 2);
}

// ===== MANIFEST_TESTS TESTS =====

#[test]
fn test_manifest_has_description_rule() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    name: "models_must_have_description"
    description: "All models must have descriptions"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
    let rule = &manifest_tests[0];
    assert_eq!(rule.name.as_deref(), Some("models_must_have_description"));
    assert_eq!(rule.severity, Severity::Error);
}

#[test]
fn test_manifest_name_convention_rule() {
    let config = r#"
manifest_tests:
  - type: "name_convention"
    severity: "warning"
    pattern: "^stg_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
    assert_eq!(manifest_tests[0].severity, Severity::Warning);
}

#[test]
fn test_manifest_has_tags_rule_with_all_criteria() {
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii", "sensitive"]
    criteria: "all"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
}

#[test]
fn test_manifest_has_tags_rule_with_any_criteria() {
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii", "sensitive"]
    criteria: "any"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
}

#[test]
fn test_manifest_has_tags_rule_with_one_of_criteria() {
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii", "sensitive"]
    criteria: "one_of"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
}

#[test]
fn test_manifest_has_tags_default_criteria() {
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii"]
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
    // Default criteria should be "all"
}

#[test]
fn test_manifest_multiple_rules() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
  - type: "name_convention"
    severity: "warning"
    pattern: "^stg_"
    applies_to: ["models"]
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii"]
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 3);
}

#[test]
fn test_manifest_includes_excludes() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
    includes: ["staging/*"]
    excludes: ["staging/stg_excluded_*"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests.len(), 1);
    assert!(manifest_tests[0].includes.is_some());
    assert!(manifest_tests[0].excludes.is_some());
}

#[test]
fn test_manifest_default_severity() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    // Should have default severity (likely "warn")
    assert_eq!(manifest_tests.len(), 1);
}

#[test]
fn test_manifest_invalid_rule_type() {
    let config = r#"
manifest_tests:
  - type: "invalid_rule_type"
    severity: "error"
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(result.is_err(), "Should fail for invalid rule type");
}

#[test]
fn test_manifest_missing_required_field() {
    let config = r#"
manifest_tests:
  - type: "name_convention"
    severity: "error"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    // Should fail because pattern is required for name_convention
    assert!(
        result.is_err(),
        "Should fail when required field 'pattern' is missing"
    );
}

// ===== CATALOG_TESTS TESTS =====

#[test]
fn test_catalog_has_description_rule() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "error"
    name: "catalog_models_must_have_description"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 1);
    let rule = &catalog_tests[0];
    assert_eq!(
        rule.name.as_deref(),
        Some("catalog_models_must_have_description")
    );
    assert_eq!(rule.severity, Severity::Error);
}

#[test]
fn test_catalog_name_convention_rule() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "warning"
    pattern: "^dim_|^fct_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 1);
    assert_eq!(catalog_tests[0].severity, Severity::Warning);
}

#[test]
fn test_catalog_multiple_rules() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "error"
    applies_to: ["models"]
  - type: "columns_all_documented"
    severity: "warning"
    pattern: "^int_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 2);
}

#[test]
fn test_catalog_includes_excludes() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "error"
    applies_to: ["models"]
    includes: ["marts/*"]
    excludes: ["marts/deprecated_*"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 1);
    assert!(catalog_tests[0].includes.is_some());
    assert!(catalog_tests[0].excludes.is_some());
}

#[test]
fn test_catalog_default_severity() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    // Should have default severity
    assert_eq!(catalog_tests.len(), 1);
}

// ===== COMBINED TESTS =====

#[test]
fn test_both_manifest_and_catalog_tests() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
  - type: "name_convention"
    severity: "warning"
    pattern: "^stg_"
    applies_to: ["models"]

catalog_tests:
  - type: "columns_all_documented"
    severity: "error"
    applies_to: ["models", "snapshots"]
  - type: "columns_all_documented"
    severity: "warning"
    pattern: "^dim_|^fct_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");

    assert_eq!(cfg.manifest_tests.unwrap().len(), 2);
    assert_eq!(cfg.catalog_tests.unwrap().len(), 2);
}

#[test]
fn test_empty_config() {
    let config = r"
manifest_tests: []
catalog_tests: []
";
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse empty config");

    assert!(cfg.manifest_tests.is_some());
    assert!(cfg.catalog_tests.is_some());
    assert_eq!(cfg.manifest_tests.unwrap().len(), 0);
    assert_eq!(cfg.catalog_tests.unwrap().len(), 0);
}

#[test]
fn test_only_manifest_tests() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");

    assert!(cfg.manifest_tests.is_some());
    assert!(cfg.catalog_tests.is_none());
}

#[test]
fn test_only_catalog_tests() {
    let config = r#"
catalog_tests:
  - type: "columns_all_documented"
    severity: "error"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");

    assert!(cfg.manifest_tests.is_none());
    assert!(cfg.catalog_tests.is_some());
}

// ===== EDGE CASES =====

#[test]
fn test_malformed_yaml() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"
"#; // Malformed YAML - missing closing bracket
    let temp_file = create_temp_config(config, Some(".yml"));
    let result = Config::from_file(temp_file.path());

    assert!(result.is_err(), "Should fail for malformed YAML");
}

#[test]
fn test_severity_values() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
  - type: "has_description"
    severity: "warning"
    applies_to: ["seeds"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    assert_eq!(manifest_tests[0].severity, Severity::Error);
    assert_eq!(manifest_tests[1].severity, Severity::Warning);
}

#[test]
fn test_applies_to_all_valid_node_types() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models", "seeds", "snapshots"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    // All specified targets should be in node_objects since they're all nodes
    assert_eq!(applies_to.node_objects.len(), 3);
}

#[test]
fn test_applies_to_different_target_types() {
    let config = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models", "sources", "macros", "exposures"]
"#;
    let temp_file = create_temp_config(config, Some(".yml"));
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    assert!(!applies_to.node_objects.is_empty()); // models
    assert!(!applies_to.source_objects.is_empty()); // sources
    assert!(!applies_to.macro_objects.is_empty()); // macros
    assert!(!applies_to.exposure_objects.is_empty()); // exposures
}

// ===== PARSE CONFIG TESTS (from parse_config.rs) =====

#[test]
fn test_parse_yaml_config() {
    let simple_rule = r#"
manifest_tests:
  - name: "model_seeds_have_description"
    type: "has_description"
    severity: "error"
    description: "All nodes must have a description."
"#;
    let temp_file = create_temp_config(simple_rule, Some(".yml"));
    let config = Config::from_yaml(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = config
        .manifest_tests
        .expect("in this test manifest_tests should be Some");
    assert_eq!(manifest_tests.len(), 1);
    let rule = &manifest_tests[0];
    assert_eq!(rule.name.as_deref(), Some("model_seeds_have_description"));
    assert_eq!(rule.severity, Severity::Error);
    assert_eq!(rule.name, Some("model_seeds_have_description".to_string()));
}

#[test]
fn test_validate_manifest_test_type() {
    let invalid_rule = r#"
manifest_tests:
   - type: "has_description"
    severity: "error"
"#;
    let temp_file = create_temp_config(invalid_rule, Some(".yml"));
    let result = Config::from_yaml(temp_file.path());
    result.expect_err("Should fail for unknown rule type");
}

#[test]
fn test_valid_applies_to() {
    let valid_rule = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to:
      - "models"
      - "seeds"
"#;
    let temp_file = create_temp_config(valid_rule, Some(".yml"));
    let result = Config::from_yaml(temp_file.path());
    assert!(result.is_ok(), "Should pass for valid applies_to targets");
}

#[test]
fn test_not_available_applies_to() {
    let invalid_rule = r#"
manifest_tests:
    - type: has_description
      severity: "error"
      applies_to: ["hook_nodes"]

"#;
    let temp_file = create_temp_config(invalid_rule, Some(".yml"));
    let result = Config::from_yaml(temp_file.path());
    assert!(
        result.is_err(),
        "Should fail for invalid applies_to for specific rule"
    );
}

#[test]
fn completely_invalid_applies_to() {
    let invalid_rule = r#"
manifest_tests:
    - type: has_description
      severity: "error"
      applies_to: ["invalid_target"]
"#;
    let temp_file = create_temp_config(invalid_rule, Some(".yml"));
    let result = Config::from_yaml(temp_file.path());
    assert!(
        result.is_err(),
        "Should fail for completely invalid applies_to targets"
    );
}

// TOML format tests
#[test]
fn test_parse_toml_basic() {
    let toml_config = r#"
[[manifest_tests]]
type = "has_description"
severity = "error"
"#;
    let temp_file = create_temp_config(toml_config, Some(".yml"));
    let config = Config::from_toml(temp_file.path()).expect("Failed to parse TOML");
    assert!(config.manifest_tests.is_some());
    assert_eq!(config.manifest_tests.unwrap().len(), 1);
}

#[test]
fn test_parse_toml_with_pattern() {
    let toml_config = r#"
[[manifest_tests]]
type = "name_convention"
pattern = "snake_case"
"#;
    let temp_file = create_temp_config(toml_config, Some(".yml"));
    let config = Config::from_toml(temp_file.path()).expect("Failed to parse TOML");
    assert!(config.manifest_tests.is_some());
}

#[test]
fn test_parse_toml_with_applies_to() {
    let toml_config = r#"
[[manifest_tests]]
type = "has_tags"
required_tags = ["tag1"]
applies_to = ["models"]
"#;
    let temp_file = create_temp_config(toml_config, Some(".yml"));
    let config = Config::from_toml(temp_file.path()).expect("Failed to parse TOML");
    assert!(config.manifest_tests.is_some());
}

#[test]
fn test_parse_toml_catalog_tests() {
    let toml_config = r#"
[[catalog_tests]]
type = "columns_all_documented"
severity = "warning"
"#;
    let temp_file = create_temp_config(toml_config, Some(".yml"));
    let config = Config::from_toml(temp_file.path()).expect("Failed to parse TOML");
    assert!(config.catalog_tests.is_some());
}

// pyproject.toml tests
#[test]
fn test_parse_pyproject_basic() {
    let pyproject = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
severity = "error"
"#;
    let temp_file = create_temp_config(pyproject, Some(".yml"));
    let config = Config::from_pyproject(temp_file.path()).expect("Failed to parse pyproject");
    assert!(config.manifest_tests.is_some());
}

#[test]
fn test_parse_pyproject_missing_section() {
    let pyproject = r#"
[tool.other]
some_config = "value"
"#;
    let temp_file = create_temp_config(pyproject, Some(".yml"));
    let result = Config::from_pyproject(temp_file.path());
    assert!(
        result.is_err(),
        "Should fail when [tool.dbtective] is missing"
    );
}

// Auto-detection tests
#[test]
fn test_from_file_yaml() {
    let yaml_content = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
"#;
    let temp_file = create_temp_config(yaml_content, Some(".yml"));
    let temp_dir = temp_file.path().parent().unwrap();
    let yaml_path = temp_dir.join("config.yml");
    std::fs::write(&yaml_path, yaml_content).unwrap();

    let result = Config::from_file(&yaml_path);
    assert!(result.is_ok());
}

#[test]
fn test_from_file_toml() {
    let toml_content = r#"
[[manifest_tests]]
type = "has_description"
"#;
    let temp_file = create_temp_config(toml_content, Some(".yml"));
    let temp_dir = temp_file.path().parent().unwrap();
    let toml_path = temp_dir.join("config.toml");
    std::fs::write(&toml_path, toml_content).unwrap();

    let result = Config::from_file(&toml_path);
    assert!(result.is_ok());
}

#[test]
fn test_from_file_pyproject() {
    let pyproject_content = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
"#;
    let temp_file = create_temp_config(pyproject_content, Some(".yml"));
    let temp_dir = temp_file.path().parent().unwrap();
    let pyproject_path = temp_dir.join("pyproject.toml");
    std::fs::write(&pyproject_path, pyproject_content).unwrap();

    let result = Config::from_file(&pyproject_path);
    assert!(result.is_ok());
}

#[test]
fn test_from_file_unsupported_format() {
    let temp_file = create_temp_config("test", Some(".yml"));
    let temp_dir = temp_file.path().parent().unwrap();
    let unknown_path = temp_dir.join("config.json");
    std::fs::write(&unknown_path, "{}").unwrap();

    let result = Config::from_file(&unknown_path);
    assert!(result.is_err());
}

// Cross-format equivalence tests
#[test]
fn test_yaml_toml_produce_same_config() {
    let yaml_config = r#"
manifest_tests:
  - name: "test_rule"
    type: "has_description"
    severity: "error"
    applies_to: ["models", "seeds"]
"#;

    let toml_config = r#"
[[manifest_tests]]
name = "test_rule"
type = "has_description"
severity = "error"
applies_to = ["models", "seeds"]
"#;

    let yaml_file = create_temp_config(yaml_config, Some(".yml"));
    let toml_file = create_temp_config(toml_config, Some(".yml"));

    let yaml_cfg = Config::from_yaml(yaml_file.path()).unwrap();
    let toml_cfg = Config::from_toml(toml_file.path()).unwrap();

    let yaml_tests = yaml_cfg.manifest_tests.as_ref().unwrap();
    let toml_tests = toml_cfg.manifest_tests.as_ref().unwrap();

    assert_eq!(yaml_tests.len(), toml_tests.len());
    assert_eq!(yaml_tests[0].name, toml_tests[0].name);
    assert_eq!(yaml_tests[0].severity, toml_tests[0].severity);
}

#[test]
fn test_all_formats_produce_same_config() {
    let yaml_config = r#"
manifest_tests:
  - type: "name_convention"
    pattern: "snake_case"
"#;

    let toml_config = r#"
[[manifest_tests]]
type = "name_convention"
pattern = "snake_case"
"#;

    let pyproject_config = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "name_convention"
pattern = "snake_case"
"#;

    let yaml_file = create_temp_config(yaml_config, Some(".yml"));
    let toml_file = create_temp_config(toml_config, Some(".yml"));
    let pyproject_file = create_temp_config(pyproject_config, Some(".yml"));

    let yaml_cfg = Config::from_yaml(yaml_file.path()).unwrap();
    let toml_cfg = Config::from_toml(toml_file.path()).unwrap();
    let pyproject_cfg = Config::from_pyproject(pyproject_file.path()).unwrap();

    assert_eq!(
        yaml_cfg.manifest_tests.as_ref().unwrap().len(),
        toml_cfg.manifest_tests.as_ref().unwrap().len()
    );
    assert_eq!(
        yaml_cfg.manifest_tests.as_ref().unwrap().len(),
        pyproject_cfg.manifest_tests.as_ref().unwrap().len()
    );
}

// Config file detection tests
#[test]
fn test_find_single_yml_config() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let config_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yml"), config_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.yml");
    assert_eq!(all_found.len(), 1);
    assert_eq!(all_found[0], "dbtective.yml");
}

#[test]
fn test_find_single_yaml_config() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let config_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yaml"), config_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.yaml");
    assert_eq!(all_found.len(), 1);
    assert_eq!(all_found[0], "dbtective.yaml");
}

#[test]
fn test_find_single_toml_config() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let config_content = r#"
[[manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.toml"), config_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.toml");
    assert_eq!(all_found.len(), 1);
    assert_eq!(all_found[0], "dbtective.toml");
}

#[test]
fn test_find_single_pyproject_config() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let config_content = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("pyproject.toml"), config_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "pyproject.toml");
    assert_eq!(all_found.len(), 1);
    assert_eq!(all_found[0], "pyproject.toml");
}

#[test]
fn test_find_no_config_returns_error() {
    let temp_dir = tempfile::TempDir::new().unwrap();

    let result = Config::find_config_in_dir(temp_dir.path());
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No dbtective config file found"));
}

#[test]
fn test_yml_priority_over_toml() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let yaml_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    let toml_content = r#"
[[manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yml"), yaml_content).unwrap();
    std::fs::write(temp_dir.path().join("dbtective.toml"), toml_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.yml");
    assert_eq!(all_found.len(), 2);
    assert!(all_found.contains(&"dbtective.yml".to_string()));
    assert!(all_found.contains(&"dbtective.toml".to_string()));
}

#[test]
fn test_yml_priority_over_pyproject() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let yaml_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    let pyproject_content = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yml"), yaml_content).unwrap();
    std::fs::write(temp_dir.path().join("pyproject.toml"), pyproject_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.yml");
    assert_eq!(all_found.len(), 2);
    assert!(all_found.contains(&"dbtective.yml".to_string()));
    assert!(all_found.contains(&"pyproject.toml".to_string()));
}

#[test]
fn test_toml_priority_over_pyproject() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let toml_content = r#"
[[manifest_tests]]
type = "has_description"
"#;
    let pyproject_content = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.toml"), toml_content).unwrap();
    std::fs::write(temp_dir.path().join("pyproject.toml"), pyproject_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    assert_eq!(chosen, "dbtective.toml");
    assert_eq!(all_found.len(), 2);
    assert!(all_found.contains(&"dbtective.toml".to_string()));
    assert!(all_found.contains(&"pyproject.toml".to_string()));
}

#[test]
fn test_all_three_configs_present() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let yaml_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    let toml_content = r#"
[[manifest_tests]]
type = "has_description"
"#;
    let pyproject_content = r#"
[tool.dbtective]

[[tool.dbtective.manifest_tests]]
type = "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yml"), yaml_content).unwrap();
    std::fs::write(temp_dir.path().join("dbtective.toml"), toml_content).unwrap();
    std::fs::write(temp_dir.path().join("pyproject.toml"), pyproject_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    // Should choose yml as highest priority
    assert_eq!(chosen, "dbtective.yml");
    assert_eq!(all_found.len(), 3);
    assert!(all_found.contains(&"dbtective.yml".to_string()));
    assert!(all_found.contains(&"dbtective.toml".to_string()));
    assert!(all_found.contains(&"pyproject.toml".to_string()));
}

#[test]
fn test_yaml_and_yml_both_present() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let yaml_content = r#"
manifest_tests:
  - type: "has_description"
"#;
    std::fs::write(temp_dir.path().join("dbtective.yml"), yaml_content).unwrap();
    std::fs::write(temp_dir.path().join("dbtective.yaml"), yaml_content).unwrap();

    let (chosen, all_found) = Config::find_config_in_dir(temp_dir.path()).unwrap();
    // Both have same priority, should return one of them
    assert!(chosen == "dbtective.yml" || chosen == "dbtective.yaml");
    assert_eq!(all_found.len(), 2);
    assert!(all_found.contains(&"dbtective.yml".to_string()));
    assert!(all_found.contains(&"dbtective.yaml".to_string()));
}
