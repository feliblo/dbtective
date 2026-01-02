//! Integration tests for includes/excludes pattern matching functionality.
//!
//! These tests verify the glob pattern matching with anchors works correctly
//! when filtering models during rule execution.

mod common;

use common::TestEnvironment;

fn config_with_includes(includes: &[&str]) -> String {
    let includes_yaml: Vec<String> = includes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    includes:
{}
"#,
        includes_yaml.join("\n")
    )
}

fn config_with_excludes(excludes: &[&str]) -> String {
    let excludes_yaml: Vec<String> = excludes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    excludes:
{}
"#,
        excludes_yaml.join("\n")
    )
}

fn config_with_includes_and_excludes(includes: &[&str], excludes: &[&str]) -> String {
    let includes_yaml: Vec<String> = includes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    let excludes_yaml: Vec<String> = excludes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    includes:
{}
    excludes:
{}
"#,
        includes_yaml.join("\n"),
        excludes_yaml.join("\n")
    )
}

// Helper to create a manifest with multiple models without descriptions
#[allow(clippy::too_many_lines)]
fn manifest_with_models(models: &[(&str, &str)]) -> String {
    let mut nodes_json = String::new();
    for (i, (name, path)) in models.iter().enumerate() {
        if i > 0 {
            nodes_json.push_str(",\n");
        }
        #[allow(clippy::format_push_string)]
        nodes_json.push_str(&format!(
            r#"    "model.test_project.{name}": {{
      "database": "analytics",
      "schema": "public",
      "name": "{name}",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "{path}",
      "original_file_path": "{path}",
      "unique_id": "model.test_project.{path}",
      "fqn": ["test_project", "{name}"],
      "alias": "{name}",
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "config": {{
        "enabled": true,
        "alias": null,
        "schema": null,
        "database": null,
        "tags": [],
        "meta": {{}},
        "group": null,
        "materialized": "view",
        "incremental_strategy": null,
        "persist_docs": {{}},
        "post-hook": [],
        "pre-hook": [],
        "quoting": {{}},
        "column_types": {{}},
        "full_refresh": null,
        "unique_key": null,
        "on_schema_change": "ignore",
        "on_configuration_change": "apply",
        "grants": {{}},
        "packages": [],
        "docs": {{"show": true, "node_color": null}},
        "contract": {{"enforced": false, "alias_types": true}}
      }},
      "tags": [],
      "description": "",
      "columns": {{}},
      "meta": {{}},
      "group": null,
      "docs": {{"show": true, "node_color": null}},
      "patch_path": null,
      "build_path": null,
      "unrendered_config": {{}},
      "created_at": 1704067200.0,
      "relation_name": "\"analytics\".\"public\".\"{name}\"",
      "raw_code": "SELECT 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": []}},
      "compiled_path": null,
      "contract": {{"enforced": false, "alias_types": true}},
      "access": "protected",
      "constraints": [],
      "version": null,
      "latest_version": null,
      "deprecation_date": null,
      "defer_relation": null
    }}"#
        ));
    }

    format!(
        r#"{{
  "metadata": {{
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {{}},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {{
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }}
  }},
  "nodes": {{
{nodes_json}
  }},
  "sources": {{}},
  "macros": {{}},
  "docs": {{}},
  "exposures": {{}},
  "metrics": {{}},
  "groups": {{}},
  "selectors": {{}},
  "disabled": {{}},
  "parent_map": {{}},
  "child_map": {{}},
  "group_map": {{}},
  "semantic_models": {{}},
  "unit_tests": {{}},
  "saved_queries": {{}}
}}"#
    )
}

#[test]
fn test_include_with_start_anchor_filters_correctly() {
    // ^models/staging/ should only include models in staging folder
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("stg_customers", "models/staging/stg_customers.sql"),
        ("dim_customers", "models/marts/dim_customers.sql"),
    ]);

    let config = config_with_includes(&["^models/staging/"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only find violations for staging models (2), not marts
    assert_eq!(
        findings.len(),
        2,
        "Should only run rule on 2 staging models, not mart model"
    );
}

#[test]
fn test_exclude_with_start_anchor_filters_correctly() {
    // ^tests/ should exclude all test files
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("test_stg_orders", "tests/test_stg_orders.sql"),
        ("test_stg_customers", "tests/test_stg_customers.sql"),
    ]);

    let config = config_with_excludes(&["^tests/"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only find violations for the model (1), not the tests
    assert_eq!(findings.len(), 1, "Should only run rule on 1 model");
}

#[test]
fn test_exclude_with_end_anchor_filters_by_extension() {
    // .yml$ should exclude YAML files
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("schema", "models/staging/schema.yml"),
    ]);

    let config = config_with_excludes(&[".yml$"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only find violations for SQL files
    assert_eq!(findings.len(), 1, "Should exclude .yml files");
}

#[test]
fn test_include_with_contains_pattern() {
    // Pattern without anchors should match anywhere in path
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("dim_orders", "models/marts/dim_orders.sql"),
        ("dim_customers", "models/marts/dim_customers.sql"),
    ]);

    let config = config_with_includes(&["orders"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should match both stg_orders and dim_orders (2 models)
    assert_eq!(
        findings.len(),
        2,
        "Should include models containing 'orders'"
    );
}

#[test]
fn test_exclude_with_contains_pattern() {
    // Pattern without anchors should exclude anywhere it matches
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("stg_customers", "models/staging/stg_customers.sql"),
        ("stg_products", "models/staging/stg_products.sql"),
    ]);

    let config = config_with_excludes(&["orders"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should exclude stg_orders, leaving 2 models
    assert_eq!(
        findings.len(),
        2,
        "Should exclude models containing 'orders'"
    );
}

#[test]
fn test_wildcard_star_does_not_cross_directories() {
    // ^models/*.sql$ should not match paths with subdirectories
    let manifest = manifest_with_models(&[
        ("model", "models/model.sql"),
        ("stg_orders", "models/staging/stg_orders.sql"),
    ]);

    let config = config_with_includes(&["^models/*.sql$"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only match models/model.sql (single * doesn't cross /)
    assert_eq!(
        findings.len(),
        1,
        "Single * should not match across directories"
    );
}

#[test]
fn test_double_star_crosses_directories() {
    // ^models/**/*.sql$ should match paths with subdirectories
    // Note: **/*.sql means "any subdirectory + file", so models/model.sql won't match
    let manifest = manifest_with_models(&[
        ("model", "models/model.sql"),
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("deep_model", "models/staging/layer2/deep_model.sql"),
    ]);

    let config = config_with_includes(&["^models/**/*.sql$"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should match only models in subdirectories (2), not models/model.sql
    assert_eq!(findings.len(), 2, "** should match across subdirectories");
}

#[test]
fn test_double_star_matches_any_depth() {
    // ^models/** should match any path starting with models/
    let manifest = manifest_with_models(&[
        ("model", "models/model.sql"),
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("deep_model", "models/staging/layer2/deep_model.sql"),
    ]);

    let config = config_with_includes(&["^models/**"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should match all 3 models
    assert_eq!(findings.len(), 3, "** should match any depth");
}

#[test]
fn test_exact_path_matching() {
    // Exact path should only match that specific file
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("stg_customers", "models/staging/stg_customers.sql"),
    ]);

    let config = config_with_includes(&["models/staging/stg_orders.sql"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only match the exact path
    assert_eq!(findings.len(), 1, "Should match only the exact path");
}

#[test]
fn test_combined_include_and_exclude() {
    // Include staging, but exclude orders
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("stg_customers", "models/staging/stg_customers.sql"),
        ("stg_products", "models/staging/stg_products.sql"),
        ("dim_customers", "models/marts/dim_customers.sql"),
    ]);

    let config = config_with_includes_and_excludes(&["^models/staging/"], &["orders"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should include staging (3) but exclude orders (1), leaving 2
    assert_eq!(
        findings.len(),
        2,
        "Should include staging except orders models"
    );
}

#[test]
fn test_both_anchors_for_exact_match() {
    // ^path$ should match only exact path
    let manifest = manifest_with_models(&[
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("prefixed", "prefix/models/staging/stg_orders.sql"),
        ("suffixed", "models/staging/stg_orders.sql.bak"),
    ]);

    let config = config_with_includes(&["^models/staging/stg_orders.sql$"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    // Should only match the exact path (no prefix, no suffix)
    assert_eq!(findings.len(), 1, "^pattern$ should match only exact path");
}

// Cross-platform path tests - patterns always use forward slashes
#[test]
fn test_forward_slash_patterns_work() {
    let manifest = manifest_with_models(&[("stg_orders", "models/staging/stg_orders.sql")]);

    let config = config_with_includes(&["models/staging/*.sql"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1, "Forward slash patterns should work");
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use super::*;

    // Helper to create a manifest with Windows-style paths (backslashes)
    fn manifest_with_windows_paths(models: Vec<(&str, &str)>) -> String {
        let mut nodes_json = String::new();
        for (i, (name, path)) in models.iter().enumerate() {
            if i > 0 {
                nodes_json.push_str(",\n");
            }
            // Use backslashes for Windows paths - must be escaped for JSON (\\)
            let windows_path = path.replace('/', r"\\");
            nodes_json.push_str(&format!(
                r#"    "model.test_project.{}": {{
      "database": "analytics",
      "schema": "public",
      "name": "{}",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "{}",
      "original_file_path": "{}",
      "unique_id": "model.test_project.{}",
      "fqn": ["test_project", "{}"],
      "alias": "{}",
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "config": {{
        "enabled": true,
        "alias": null,
        "schema": null,
        "database": null,
        "tags": [],
        "meta": {{}},
        "group": null,
        "materialized": "view",
        "incremental_strategy": null,
        "persist_docs": {{}},
        "post-hook": [],
        "pre-hook": [],
        "quoting": {{}},
        "column_types": {{}},
        "full_refresh": null,
        "unique_key": null,
        "on_schema_change": "ignore",
        "on_configuration_change": "apply",
        "grants": {{}},
        "packages": [],
        "docs": {{"show": true, "node_color": null}},
        "contract": {{"enforced": false, "alias_types": true}}
      }},
      "tags": [],
      "description": "",
      "columns": {{}},
      "meta": {{}},
      "group": null,
      "docs": {{"show": true, "node_color": null}},
      "patch_path": null,
      "build_path": null,
      "unrendered_config": {{}},
      "created_at": 1704067200.0,
      "relation_name": "\"analytics\".\"public\".\"{}\"",
      "raw_code": "SELECT 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": []}},
      "compiled_path": null,
      "contract": {{"enforced": false, "alias_types": true}},
      "access": "protected",
      "constraints": [],
      "version": null,
      "latest_version": null,
      "deprecation_date": null,
      "defer_relation": null
    }}"#,
                name, name, windows_path, windows_path, name, name, name, name
            ));
        }

        format!(
            r#"{{
  "metadata": {{
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {{}},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": {{
      "database": true,
      "schema": true,
      "identifier": true,
      "column": null
    }}
  }},
  "nodes": {{
{}
  }},
  "sources": {{}},
  "macros": {{}},
  "docs": {{}},
  "exposures": {{}},
  "metrics": {{}},
  "groups": {{}},
  "selectors": {{}},
  "disabled": {{}},
  "parent_map": {{}},
  "child_map": {{}},
  "group_map": {{}},
  "semantic_models": {{}},
  "unit_tests": {{}},
  "saved_queries": {{}}
}}"#,
            nodes_json
        )
    }

    #[test]
    fn test_windows_backslash_paths_with_forward_slash_patterns() {
        // Windows manifests may have backslash paths, but patterns use forward slashes
        // The includes_excludes logic should normalize paths
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("stg_customers", "models/staging/stg_customers.sql"),
        ]);

        // Pattern uses forward slashes (always)
        let config = config_with_includes(&["models/staging/*.sql"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            2,
            "Windows backslash paths should match forward-slash patterns"
        );
    }

    #[test]
    fn test_windows_paths_with_start_anchor() {
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("dim_customers", "models/marts/dim_customers.sql"),
        ]);

        let config = config_with_includes(&["^models/staging/"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            1,
            "Windows paths should work with start anchor patterns"
        );
    }

    #[test]
    fn test_windows_paths_with_double_star() {
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("deep_model", "models/staging/layer2/deep_model.sql"),
        ]);

        let config = config_with_includes(&["^models/**"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            2,
            "Windows paths should work with ** patterns"
        );
    }

    #[test]
    fn test_windows_paths_with_end_anchor() {
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("schema", "models/staging/schema.yml"),
        ]);

        let config = config_with_includes(&[".sql$"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            1,
            "Windows paths should work with end anchor patterns"
        );
    }

    #[test]
    fn test_windows_paths_with_contains_pattern() {
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("dim_orders", "models/marts/dim_orders.sql"),
            ("dim_customers", "models/marts/dim_customers.sql"),
        ]);

        let config = config_with_includes(&["orders"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            2,
            "Windows paths should work with contains patterns"
        );
    }

    #[test]
    fn test_windows_paths_exclude() {
        let manifest = manifest_with_windows_paths(vec![
            ("stg_orders", "models/staging/stg_orders.sql"),
            ("stg_customers", "models/staging/stg_customers.sql"),
        ]);

        let config = config_with_excludes(&["orders"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);

        assert_eq!(
            findings.len(),
            1,
            "Windows paths should work with exclude patterns"
        );
    }
}

/// Comprehensive test that validates all pattern matching capabilities in one test
/// This test creates a diverse set of models and validates pattern matching behavior
#[test]
fn test_comprehensive_pattern_matching() {
    let manifest = manifest_with_models(&[
        // Staging models
        ("stg_orders", "models/staging/stg_orders.sql"),
        ("stg_customers", "models/staging/stg_customers.sql"),
        ("stg_products", "models/staging/stg_products.sql"),
        // Staging subdirectory
        ("stg_payments", "models/staging/payments/stg_payments.sql"),
        // Marts models
        ("dim_customers", "models/marts/dim_customers.sql"),
        ("fct_orders", "models/marts/fct_orders.sql"),
        // Marts subdirectory
        ("fct_revenue", "models/marts/finance/fct_revenue.sql"),
        // Root level model
        ("root_model", "models/root_model.sql"),
        // Legacy/deprecated models
        ("legacy_model", "models/legacy/old_model.sql"),
        // Deep nested path
        (
            "deep_model",
            "models/marts/finance/reporting/quarterly/deep_model.sql",
        ),
    ]);

    // Test 1: Start anchor - only models starting with "models/staging/"
    {
        let config = config_with_includes(&["^models/staging/"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            4, // stg_orders, stg_customers, stg_products, stg_payments
            "^models/staging/ should match 4 models in staging (including subdirs)"
        );
    }

    // Test 2: Contains pattern - match "orders" anywhere
    {
        let config = config_with_includes(&["orders"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            2, // stg_orders, fct_orders
            "'orders' should match 2 models containing 'orders'"
        );
    }

    // Test 3: Single star - match files directly in models/staging/ (not subdirs)
    {
        let config = config_with_includes(&["^models/staging/*.sql$"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            3, // stg_orders, stg_customers, stg_products (not stg_payments which is in subdir)
            "^models/staging/*.sql$ should match 3 SQL files directly in staging"
        );
    }

    // Test 4: Double star - match any depth under models/marts/
    {
        let config = config_with_includes(&["^models/marts/**"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            4, // dim_customers, fct_orders, fct_revenue, deep_model
            "^models/marts/** should match 4 models in marts at any depth"
        );
    }

    // Test 5: Both anchors - exact path match
    {
        let config = config_with_includes(&["^models/staging/stg_orders.sql$"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            1, // Only stg_orders
            "^path$ should match exactly 1 model"
        );
    }

    // Test 6: Exclude with start anchor - exclude legacy folder
    {
        let config = config_with_excludes(&["^models/legacy/"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            9, // 10 total - 1 legacy_model = 9
            "^models/legacy/ exclude should leave 9 models"
        );
    }

    // Test 7: Combined include and exclude
    {
        let config = config_with_includes_and_excludes(
            &["^models/staging/"], // Include staging
            &["stg_products"],     // But exclude products
        );
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            3, // stg_orders, stg_customers, stg_payments (not stg_products)
            "Include staging except products should leave 3 models"
        );
    }

    // Test 8: Exclude with contains pattern
    {
        let config = config_with_excludes(&["_deprecated"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            10, // None contain _deprecated, so all 10 remain
            "Excluding '_deprecated' should leave all 10 models"
        );
    }

    // Test 9: Deep nested path matching with contains
    {
        let config = config_with_includes(&["quarterly"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            1, // Only deep_model
            "'quarterly' should match 1 deeply nested model"
        );
    }

    // Test 10: Multiple exclude patterns (exclude staging and legacy)
    {
        let config = config_with_excludes(&["^models/staging/", "^models/legacy/"]);
        let env = TestEnvironment::new(&manifest, &config);
        let findings = env.run_maniest_rules(false);
        assert_eq!(
            findings.len(),
            5, // 10 - 4 staging - 1 legacy = 5 (marts + root)
            "Excluding staging and legacy should leave 5 models"
        );
    }
}
