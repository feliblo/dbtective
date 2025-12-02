use dbtective::core::config::{applies_to::RuleTarget, parse_config::Config, severity::Severity};
use std::io::Write;
use tempfile::NamedTempFile;

fn create_temp_config(content: &str) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    assert_eq!(applies_to.node_objects.len(), 2); // models, seeds
    assert_eq!(applies_to.source_objects.len(), 1); // sources
    assert_eq!(applies_to.node_objects.contains(&RuleTarget::Models), true);
    assert_eq!(applies_to.node_objects.contains(&RuleTarget::Seeds), true);
    assert_eq!(
        applies_to.source_objects.contains(&RuleTarget::Sources),
        true
    );
}

#[test]
fn test_applies_to_invalid_target_for_rule() {
    // has_tags cannot apply to unit_tests according to applies_to_options_for_rule
    let config = r#"
manifest_tests:
  - type: "has_tags"
    severity: "error"
    required_tags: ["pii"]
    applies_to: ["hook_nodes"]
"#;
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
    let result = Config::from_file(temp_file.path());

    assert!(result.is_err(), "Should fail for empty applies_to list");
}

#[test]
fn test_applies_to_catalog_tests() {
    let config = r#"
catalog_tests:
  - type: "has_description"
    severity: "warning"
    applies_to: ["models", "snapshots"]
"#;
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
  - type: "has_description"
    severity: "error"
    name: "catalog_models_must_have_description"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
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
  - type: "name_convention"
    severity: "warning"
    pattern: "^dim_|^fct_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 1);
    assert_eq!(catalog_tests[0].severity, Severity::Warning);
}

#[test]
fn test_catalog_multiple_rules() {
    let config = r#"
catalog_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
  - type: "name_convention"
    severity: "warning"
    pattern: "^int_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let catalog_tests = cfg.catalog_tests.unwrap();

    assert_eq!(catalog_tests.len(), 2);
}

#[test]
fn test_catalog_includes_excludes() {
    let config = r#"
catalog_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
    includes: ["marts/*"]
    excludes: ["marts/deprecated_*"]
"#;
    let temp_file = create_temp_config(config);
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
  - type: "has_description"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
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
  - type: "has_description"
    severity: "error"
    applies_to: ["models", "snapshots"]
  - type: "name_convention"
    severity: "warning"
    pattern: "^dim_|^fct_"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");

    assert_eq!(cfg.manifest_tests.unwrap().len(), 2);
    assert_eq!(cfg.catalog_tests.unwrap().len(), 2);
}

#[test]
fn test_empty_config() {
    let config = r#"
manifest_tests: []
catalog_tests: []
"#;
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");

    assert!(cfg.manifest_tests.is_some());
    assert!(cfg.catalog_tests.is_none());
}

#[test]
fn test_only_catalog_tests() {
    let config = r#"
catalog_tests:
  - type: "has_description"
    severity: "error"
    applies_to: ["models"]
"#;
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
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
    let temp_file = create_temp_config(config);
    let cfg = Config::from_file(temp_file.path()).expect("Failed to parse config");
    let manifest_tests = cfg.manifest_tests.unwrap();

    let applies_to = manifest_tests[0].applies_to.as_ref().unwrap();
    assert!(!applies_to.node_objects.is_empty()); // models
    assert!(!applies_to.source_objects.is_empty()); // sources
    assert!(!applies_to.macro_objects.is_empty()); // macros
    assert!(!applies_to.exposure_objects.is_empty()); // exposures
}
