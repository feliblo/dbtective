mod common;

use common::TestEnvironment;

// =============================================================================
// Helper: minimal manifest with configurable objects
// =============================================================================

fn base_manifest(nodes: &str, sources: &str, macros: &str, exposures: &str) -> String {
    format!(
        r#"{{
  "metadata": {{
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0"
  }},
  "nodes": {{{nodes}}},
  "sources": {{{sources}}},
  "macros": {{{macros}}},
  "exposures": {{{exposures}}},
  "semantic_models": {{}},
  "unit_tests": {{}},
  "parent_map": {{}},
  "child_map": {{}}
}}"#
    )
}

fn pp_json(patch_path: Option<&str>) -> String {
    patch_path.map_or_else(|| "null".to_string(), |p| format!("\"test_project://{p}\""))
}

fn model_node(name: &str, original_file_path: &str, patch_path: Option<&str>) -> String {
    let pp = pp_json(patch_path);
    format!(
        r#"
    "model.test.{name}": {{
      "name": "{name}",
      "resource_type": "model",
      "path": "{original_file_path}",
      "original_file_path": "{original_file_path}",
      "unique_id": "model.test.{name}",
      "description": "A model",
      "config": {{"materialized": "view"}},
      "raw_code": "SELECT 1",
      "patch_path": {pp}
    }}"#
    )
}

fn source_obj(name: &str, original_file_path: &str, patch_path: Option<&str>) -> String {
    let pp = pp_json(patch_path);
    format!(
        r#"
    "source.test.{name}": {{
      "name": "{name}",
      "package_name": "test_project",
      "original_file_path": "{original_file_path}",
      "patch_path": {pp},
      "unique_id": "source.test.{name}",
      "database": "my_db",
      "description": "A source"
    }}"#
    )
}

fn macro_obj(name: &str, original_file_path: &str, patch_path: Option<&str>) -> String {
    let pp = pp_json(patch_path);
    format!(
        r#"
    "macro.test.{name}": {{
      "name": "{name}",
      "package_name": "test_project",
      "original_file_path": "{original_file_path}",
      "patch_path": {pp},
      "macro_sql": "SELECT 1",
      "description": "A macro"
    }}"#
    )
}

fn exposure_obj(name: &str, original_file_path: &str, patch_path: Option<&str>) -> String {
    let pp = pp_json(patch_path);
    format!(
        r#"
    "exposure.test.{name}": {{
      "name": "{name}",
      "package_name": "test_project",
      "original_file_path": "{original_file_path}",
      "patch_path": {pp},
      "description": "An exposure",
      "depends_on": {{"macros": [], "nodes": []}}
    }}"#
    )
}

// =============================================================================
// SameDirectory mode — models
// =============================================================================

#[test]
fn test_same_directory_model_pass() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Same directory should pass");
}

#[test]
fn test_same_directory_model_fail() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/schema.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Different directory should fail");
    assert!(findings[0].0.message.contains("different directory"));
}

#[test]
fn test_same_directory_model_no_patch_path_skipped() {
    let manifest = base_manifest(
        &model_node("stg_orders", "models/staging/stg_orders.sql", None),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "No patch_path should be skipped");
}

// =============================================================================
// SameDirectory mode — sources
// =============================================================================

#[test]
fn test_same_directory_source_pass() {
    let manifest = base_manifest(
        "",
        &source_obj(
            "raw_orders",
            "models/sources/sources.yml",
            Some("models/sources/_sources.yml"),
        ),
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["sources"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Source same directory should pass");
}

#[test]
fn test_same_directory_source_fail() {
    let manifest = base_manifest(
        "",
        &source_obj(
            "raw_orders",
            "models/sources/sources.yml",
            Some("models/other.yml"),
        ),
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["sources"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Source different directory should fail");
}

// =============================================================================
// SameDirectory mode — macros
// =============================================================================

#[test]
fn test_same_directory_macro_pass() {
    let manifest = base_manifest(
        "",
        "",
        &macro_obj(
            "my_macro",
            "macros/my_macro.sql",
            Some("macros/_macros.yml"),
        ),
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["macros"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Macro same directory should pass");
}

#[test]
fn test_same_directory_macro_fail() {
    let manifest = base_manifest(
        "",
        "",
        &macro_obj(
            "my_macro",
            "macros/utils/my_macro.sql",
            Some("macros/schema.yml"),
        ),
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["macros"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Macro different directory should fail");
}

// =============================================================================
// SameDirectory mode — exposures
// =============================================================================

#[test]
fn test_same_directory_exposure_pass() {
    let manifest = base_manifest(
        "",
        "",
        "",
        &exposure_obj(
            "dashboard",
            "models/exposures/dashboard.yml",
            Some("models/exposures/_exposures.yml"),
        ),
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["exposures"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Exposure same directory should pass");
}

#[test]
fn test_same_directory_exposure_fail() {
    let manifest = base_manifest(
        "",
        "",
        "",
        &exposure_obj(
            "dashboard",
            "models/exposures/dashboard.yml",
            Some("models/other.yml"),
        ),
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["exposures"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Exposure different directory should fail"
    );
}

// =============================================================================
// RelativeSubdirectory mode
// =============================================================================

#[test]
fn test_relative_subdirectory_allowed_pass() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/properties/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
    mode: "relative_subdirectory"
    allowed_subdirectories:
      - "properties"
      - "schema"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Allowed subdirectory should pass");
}

#[test]
fn test_relative_subdirectory_second_allowed_pass() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/schema/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
    mode: "relative_subdirectory"
    allowed_subdirectories:
      - "properties"
      - "schema"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0, "Second allowed subdirectory should pass");
}

#[test]
fn test_relative_subdirectory_wrong_subdir_fail() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/docs/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
    mode: "relative_subdirectory"
    allowed_subdirectories:
      - "properties"
      - "schema"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Wrong subdirectory should fail");
    assert!(findings[0]
        .0
        .message
        .contains("not in an allowed subdirectory"));
}

#[test]
fn test_relative_subdirectory_same_dir_fails() {
    // In relative_subdirectory mode, same directory is NOT valid — must be in a subdirectory
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
    mode: "relative_subdirectory"
    allowed_subdirectories:
      - "properties"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Same directory should fail in relative_subdirectory mode"
    );
}

#[test]
fn test_relative_subdirectory_no_allowed_falls_back_same_dir() {
    // No allowed_subdirectories configured — falls back to same_directory behavior
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/_stg_orders.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
    mode: "relative_subdirectory"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "No allowed_subdirectories should fall back to same_directory"
    );
}

// =============================================================================
// Multiple objects in one manifest
// =============================================================================

#[test]
fn test_multiple_models_mixed_results() {
    let nodes = format!(
        "{},{}",
        model_node(
            "good_model",
            "models/staging/good.sql",
            Some("models/staging/_good.yml")
        ),
        model_node(
            "bad_model",
            "models/staging/bad.sql",
            Some("models/schema.yml")
        ),
    );
    let manifest = base_manifest(&nodes, "", "", "");
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Only the misplaced model should fail");
    assert!(findings[0].0.message.contains("bad_model"));
}

// =============================================================================
// applies_to filtering
// =============================================================================

#[test]
fn test_applies_to_models_only_ignores_sources() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/staging/_stg.yml"),
        ),
        &source_obj(
            "raw_orders",
            "models/sources/sources.yml",
            Some("models/other.yml"),
        ),
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Source violation should be ignored when applies_to is models only"
    );
}

// =============================================================================
// Severity levels
// =============================================================================

#[test]
fn test_severity_warning() {
    let manifest = base_manifest(
        &model_node(
            "stg_orders",
            "models/staging/stg_orders.sql",
            Some("models/schema.yml"),
        ),
        "",
        "",
        "",
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "warning"
    applies_to: ["models"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].1,
        dbtective::core::config::severity::Severity::Warning,
    );
}

// =============================================================================
// Multiple object types in a single rule
// =============================================================================

#[test]
fn test_all_object_types_checked() {
    let manifest = base_manifest(
        &model_node(
            "model1",
            "models/staging/model1.sql",
            Some("models/other.yml"),
        ),
        &source_obj(
            "source1",
            "models/sources/source1.yml",
            Some("models/other.yml"),
        ),
        &macro_obj(
            "macro1",
            "macros/utils/macro1.sql",
            Some("macros/other.yml"),
        ),
        &exposure_obj(
            "exposure1",
            "models/exposures/exposure1.yml",
            Some("models/other.yml"),
        ),
    );
    let config = r#"
manifest_tests:
  - name: "colocation"
    type: "property_file_colocation"
    severity: "error"
    applies_to: ["models", "sources", "macros", "exposures"]
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 4, "All four object types should fail");
}
