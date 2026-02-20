//! Integration tests for includes/excludes filtering in catalog and fallback rules.
//!
//! These tests verify that `should_run_test()` is properly called for all
//! catalog rule execution paths: catalog node rules, catalog source rules,
//! fallback node rules, and fallback source rules.
//!
//! Tests cover all relevant dbt object types: models, seeds, snapshots, sources.

use crate::common::TestEnvironment;

// =========================================================================
// Helpers
// =========================================================================

const MANIFEST_METADATA: &str = r#"
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
  }"#;

const MANIFEST_EMPTY_SECTIONS: &str = r#"
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
  "unit_tests": {}"#;

const CATALOG_METADATA: &str = r#"
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "env": {}
  }"#;

/// Create a manifest node JSON snippet with tags
fn manifest_node_with_tags(
    unique_id: &str,
    name: &str,
    resource_type: &str,
    original_file_path: &str,
    tags: &[&str],
) -> String {
    let escaped_path = original_file_path.replace('\\', "\\\\");
    let tags_json: Vec<String> = tags.iter().map(|t| format!("\"{t}\"")).collect();
    let tags_str = tags_json.join(", ");
    format!(
        r#"    "{unique_id}": {{
      "database": "analytics",
      "schema": "public",
      "name": "{name}",
      "resource_type": "{resource_type}",
      "package_name": "test_project",
      "path": "{name}.sql",
      "original_file_path": "{escaped_path}",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "{name}"],
      "alias": "{name}",
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "tags": [{tags_str}],
      "config": {{
        "enabled": true,
        "materialized": "view",
        "tags": []
      }},
      "description": "Test {name}",
      "columns": {{
        "id": {{"name": "id", "description": "", "tags": []}}
      }},
      "meta": {{}},
      "group": null,
      "docs": {{"show": true}},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {{}},
      "created_at": 1704067200.0,
      "config_call_dict": {{}},
      "relation_name": "analytics.public.{name}",
      "raw_code": "SELECT 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": []}},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {{"enforced": false, "checksum": null}}
    }}"#
    )
}

/// Create a manifest source JSON snippet with tags
fn manifest_source_with_tags(
    unique_id: &str,
    name: &str,
    source_name: &str,
    original_file_path: &str,
    tags: &[&str],
) -> String {
    let escaped_path = original_file_path.replace('\\', "\\\\");
    let tags_json: Vec<String> = tags.iter().map(|t| format!("\"{t}\"")).collect();
    let tags_str = tags_json.join(", ");
    format!(
        r#"    "{unique_id}": {{
      "database": "raw",
      "schema": "raw_data",
      "name": "{name}",
      "source_name": "{source_name}",
      "source_description": "Test source",
      "loader": "",
      "identifier": "{name}",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [{tags_str}],
      "path": "sources.yml",
      "original_file_path": "{escaped_path}",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "{source_name}", "{name}"],
      "config": {{"enabled": true}},
      "description": "Test source {name}",
      "columns": {{
        "id": {{"name": "id", "description": "", "tags": []}}
      }}
    }}"#
    )
}

/// Create a manifest node JSON snippet
fn manifest_node(
    unique_id: &str,
    name: &str,
    resource_type: &str,
    original_file_path: &str,
) -> String {
    let escaped_path = original_file_path.replace('\\', "\\\\");
    format!(
        r#"    "{unique_id}": {{
      "database": "analytics",
      "schema": "public",
      "name": "{name}",
      "resource_type": "{resource_type}",
      "package_name": "test_project",
      "path": "{name}.sql",
      "original_file_path": "{escaped_path}",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "{name}"],
      "alias": "{name}",
      "checksum": {{"name": "sha256", "checksum": "abc123"}},
      "tags": [],
      "config": {{
        "enabled": true,
        "materialized": "view",
        "tags": []
      }},
      "description": "Test {name}",
      "columns": {{
        "id": {{"name": "id", "description": "", "tags": []}}
      }},
      "meta": {{}},
      "group": null,
      "docs": {{"show": true}},
      "patch_path": null,
      "compiled_path": null,
      "build_path": null,
      "deferred": false,
      "unrendered_config": {{}},
      "created_at": 1704067200.0,
      "config_call_dict": {{}},
      "relation_name": "analytics.public.{name}",
      "raw_code": "SELECT 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {{"macros": [], "nodes": []}},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {{"enforced": false, "checksum": null}}
    }}"#
    )
}

/// Create a manifest source JSON snippet
fn manifest_source(
    unique_id: &str,
    name: &str,
    source_name: &str,
    original_file_path: &str,
) -> String {
    let escaped_path = original_file_path.replace('\\', "\\\\");
    format!(
        r#"    "{unique_id}": {{
      "database": "raw",
      "schema": "raw_data",
      "name": "{name}",
      "source_name": "{source_name}",
      "source_description": "Test source",
      "loader": "",
      "identifier": "{name}",
      "resource_type": "source",
      "package_name": "test_project",
      "tags": [],
      "path": "sources.yml",
      "original_file_path": "{escaped_path}",
      "unique_id": "{unique_id}",
      "fqn": ["test_project", "{source_name}", "{name}"],
      "config": {{"enabled": true}},
      "description": "Test source {name}",
      "columns": {{
        "id": {{"name": "id", "description": "", "tags": []}}
      }}
    }}"#
    )
}

/// Create a catalog node JSON snippet
fn catalog_node(unique_id: &str, name: &str) -> String {
    format!(
        r#"    "{unique_id}": {{
      "unique_id": "{unique_id}",
      "metadata": {{
        "type": "BASE TABLE",
        "schema": "public",
        "name": "{name}",
        "database": "analytics"
      }},
      "columns": {{
        "id": {{"type": "INTEGER", "name": "id", "index": 1}}
      }},
      "stats": {{}}
    }}"#
    )
}

/// Create a catalog source JSON snippet
fn catalog_source(unique_id: &str, name: &str) -> String {
    format!(
        r#"    "{unique_id}": {{
      "unique_id": "{unique_id}",
      "metadata": {{
        "type": "BASE TABLE",
        "schema": "raw_data",
        "name": "{name}",
        "database": "raw"
      }},
      "columns": {{
        "id": {{"type": "INTEGER", "name": "id", "index": 1}}
      }},
      "stats": {{}}
    }}"#
    )
}

/// Build a full manifest JSON from node and source snippets
fn build_manifest(nodes: &[String], sources: &[String]) -> String {
    format!(
        r#"{{
{MANIFEST_METADATA},
  "nodes": {{
{nodes}
  }},
  "sources": {{
{sources}
  }},
{MANIFEST_EMPTY_SECTIONS}
}}"#,
        nodes = nodes.join(",\n"),
        sources = sources.join(",\n")
    )
}

/// Build a full catalog JSON from node and source snippets
fn build_catalog(nodes: &[String], sources: &[String]) -> String {
    format!(
        r#"{{
{CATALOG_METADATA},
  "nodes": {{
{nodes}
  }},
  "sources": {{
{sources}
  }}
}}"#,
        nodes = nodes.join(",\n"),
        sources = sources.join(",\n")
    )
}

fn catalog_config_with_excludes(excludes: &[&str], applies_to: &[&str]) -> String {
    let excludes_yaml: Vec<String> = excludes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    let applies_yaml: Vec<String> = applies_to
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
{applies_to}
    excludes:
{excludes}
"#,
        applies_to = applies_yaml.join("\n"),
        excludes = excludes_yaml.join("\n")
    )
}

fn catalog_config_with_includes(includes: &[&str], applies_to: &[&str]) -> String {
    let includes_yaml: Vec<String> = includes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    let applies_yaml: Vec<String> = applies_to
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
{applies_to}
    includes:
{includes}
"#,
        applies_to = applies_yaml.join("\n"),
        includes = includes_yaml.join("\n")
    )
}

fn catalog_config_with_includes_and_excludes(
    includes: &[&str],
    excludes: &[&str],
    applies_to: &[&str],
) -> String {
    let includes_yaml: Vec<String> = includes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    let excludes_yaml: Vec<String> = excludes
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    let applies_yaml: Vec<String> = applies_to
        .iter()
        .map(|s| format!("      - \"{s}\""))
        .collect();
    format!(
        r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
{applies_to}
    includes:
{includes}
    excludes:
{excludes}
"#,
        applies_to = applies_yaml.join("\n"),
        includes = includes_yaml.join("\n"),
        excludes = excludes_yaml.join("\n")
    )
}

// =========================================================================
// Fallback Node Rules - Models
// =========================================================================

#[test]
fn test_fallback_node_excludes_model_by_directory() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw model, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_excludes_model_by_glob() {
    let manifest = build_manifest(
        &[
            // Note: **/* requires at least one subdirectory, so put the file deeper
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/subsystem/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw/**/*"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Glob exclude should work, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_includes_only_matching_models() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["models/marts"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Include should only match marts, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_combined_includes_excludes() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_customers",
                "mart_customers",
                "model",
                "models/marts/mart_customers.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let config =
        catalog_config_with_includes_and_excludes(&["models/marts"], &["mart_orders"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include marts but exclude mart_orders, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_customers"));
}

// =========================================================================
// Fallback Node Rules - Seeds
// =========================================================================

#[test]
fn test_fallback_node_excludes_seed() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "seed.test_project.raw_data",
                "raw_data",
                "seed",
                "seeds/raw_data.csv",
            ),
            manifest_node(
                "seed.test_project.ref_data",
                "ref_data",
                "seed",
                "seeds/reference/ref_data.csv",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["seeds/reference"], &["seeds"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude reference seed, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("raw_data"));
}

#[test]
fn test_fallback_node_includes_only_matching_seeds() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "seed.test_project.raw_data",
                "raw_data",
                "seed",
                "seeds/raw_data.csv",
            ),
            manifest_node(
                "seed.test_project.ref_data",
                "ref_data",
                "seed",
                "seeds/reference/ref_data.csv",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["seeds/reference"], &["seeds"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include only reference seed, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("ref_data"));
}

// =========================================================================
// Fallback Node Rules - Snapshots
// =========================================================================

#[test]
fn test_fallback_node_excludes_snapshot() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "snapshot.test_project.orders_snapshot",
                "orders_snapshot",
                "snapshot",
                "snapshots/orders_snapshot.sql",
            ),
            manifest_node(
                "snapshot.test_project.users_snapshot",
                "users_snapshot",
                "snapshot",
                "snapshots/legacy/users_snapshot.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["snapshots/legacy"], &["snapshots"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude legacy snapshot, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders_snapshot"));
}

// =========================================================================
// Fallback Source Rules
// =========================================================================

#[test]
fn test_fallback_source_excludes_by_directory() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/raw/sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                "models/reference/sources.yml",
            ),
        ],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw source, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

#[test]
fn test_fallback_source_includes_only_matching() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/raw/sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                "models/reference/sources.yml",
            ),
        ],
    );
    let config = catalog_config_with_includes(&["models/reference"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include only reference source, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

#[test]
fn test_fallback_source_combined_includes_excludes() {
    // Note: should_run_test matches against original_file_path, not source name.
    // So each source needs a different file path for excludes to differentiate them.
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources/raw_users.yml",
            ),
            manifest_source(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources/raw_orders.yml",
            ),
            manifest_source(
                "source.test_project.ext.events",
                "events",
                "ext",
                "models/sources/external.yml",
            ),
        ],
    );
    let config = catalog_config_with_includes_and_excludes(
        &["models/sources/raw"],
        &["raw_orders"],
        &["sources"],
    );
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include raw sources but exclude raw_orders, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("users"));
}

// =========================================================================
// Catalog Node Rules (with actual catalog)
// =========================================================================

#[test]
fn test_catalog_node_excludes_model_by_directory() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw model, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_excludes_model_by_glob() {
    let manifest = build_manifest(
        &[
            // Note: **/* requires at least one subdirectory, so put the file deeper
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/subsystem/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw/**/*"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Glob exclude should work in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_includes_only_matching_models() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
            catalog_node("model.test_project.stg_orders", "stg_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["models/marts"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Include should only match marts in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_excludes_seed() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "seed.test_project.raw_data",
                "raw_data",
                "seed",
                "seeds/raw_data.csv",
            ),
            manifest_node(
                "seed.test_project.ref_data",
                "ref_data",
                "seed",
                "seeds/reference/ref_data.csv",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("seed.test_project.raw_data", "raw_data"),
            catalog_node("seed.test_project.ref_data", "ref_data"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["seeds/reference"], &["seeds"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude reference seed in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("raw_data"));
}

#[test]
fn test_catalog_node_excludes_snapshot() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "snapshot.test_project.orders_snap",
                "orders_snap",
                "snapshot",
                "snapshots/orders_snap.sql",
            ),
            manifest_node(
                "snapshot.test_project.legacy_snap",
                "legacy_snap",
                "snapshot",
                "snapshots/legacy/legacy_snap.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("snapshot.test_project.orders_snap", "orders_snap"),
            catalog_node("snapshot.test_project.legacy_snap", "legacy_snap"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["snapshots/legacy"], &["snapshots"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude legacy snapshot in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders_snap"));
}

// =========================================================================
// Catalog Source Rules (with actual catalog)
// =========================================================================

#[test]
fn test_catalog_source_excludes_by_directory() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/raw/sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                "models/reference/sources.yml",
            ),
        ],
    );
    let catalog = build_catalog(
        &[],
        &[
            catalog_source("source.test_project.raw.users", "users"),
            catalog_source("source.test_project.ref.products", "products"),
        ],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["sources"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw source in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

#[test]
fn test_catalog_source_includes_only_matching() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/raw/sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                "models/reference/sources.yml",
            ),
        ],
    );
    let catalog = build_catalog(
        &[],
        &[
            catalog_source("source.test_project.raw.users", "users"),
            catalog_source("source.test_project.ref.products", "products"),
        ],
    );
    let config = catalog_config_with_includes(&["models/reference"], &["sources"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include only reference source in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

// =========================================================================
// Windows-style paths with dbt/ prefix - Fallback
// =========================================================================

#[test]
fn test_fallback_node_excludes_dbt_prefix_windows_path() {
    // Simulates Windows manifest: original_file_path = "dbt/models\raw\raw_orders.sql"
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                r"dbt/models\raw\raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                r"dbt/models\marts\mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw model with dbt/ prefix Windows path, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_includes_dbt_prefix_windows_path() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                r"dbt/models\raw\raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                r"dbt/models\marts\mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                r"dbt/models\staging\stg_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["models/marts"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Include should work with dbt/ prefix Windows paths, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_excludes_dbt_prefix_glob_pattern() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                r"dbt/models\raw\subsystem\raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                r"dbt/models\marts\mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw/**/*"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Glob exclude should work with dbt/ prefix, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_source_excludes_dbt_prefix_windows_path() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                r"dbt/models\raw\sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                r"dbt/models\reference\sources.yml",
            ),
        ],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw source with dbt/ prefix Windows path, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

// =========================================================================
// Windows-style paths with dbt/ prefix - Catalog
// =========================================================================

#[test]
fn test_catalog_node_excludes_dbt_prefix_windows_path() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                r"dbt/models\raw\raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                r"dbt/models\marts\mart_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw model with dbt/ prefix in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_source_excludes_dbt_prefix_windows_path() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                r"dbt/models\raw\sources.yml",
            ),
            manifest_source(
                "source.test_project.ref.products",
                "products",
                "ref",
                r"dbt/models\reference\sources.yml",
            ),
        ],
    );
    let catalog = build_catalog(
        &[],
        &[
            catalog_source("source.test_project.raw.users", "users"),
            catalog_source("source.test_project.ref.products", "products"),
        ],
    );
    let config = catalog_config_with_excludes(&["models/raw"], &["sources"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should exclude raw source with dbt/ prefix in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("products"));
}

// =========================================================================
// Mixed object types
// =========================================================================

#[test]
fn test_fallback_excludes_apply_across_object_types() {
    // Models, seeds, and snapshots in the same manifest, all with issues
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "seed.test_project.raw_seed",
                "raw_seed",
                "seed",
                "seeds/raw/raw_seed.csv",
            ),
            manifest_node(
                "seed.test_project.ref_seed",
                "ref_seed",
                "seed",
                "seeds/ref/ref_seed.csv",
            ),
            manifest_node(
                "snapshot.test_project.snap_a",
                "snap_a",
                "snapshot",
                "snapshots/active/snap_a.sql",
            ),
            manifest_node(
                "snapshot.test_project.snap_l",
                "snap_l",
                "snapshot",
                "snapshots/legacy/snap_l.sql",
            ),
        ],
        &[],
    );

    // Exclude raw models, raw seeds, and legacy snapshots
    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
      - "seeds"
      - "snapshots"
    excludes:
      - "models/raw"
      - "seeds/raw"
      - "snapshots/legacy"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        3,
        "Should have 3 findings (mart_orders, ref_seed, snap_a), got: {findings:?}"
    );

    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(
        names.iter().any(|m| m.contains("mart_orders")),
        "Should find mart_orders"
    );
    assert!(
        names.iter().any(|m| m.contains("ref_seed")),
        "Should find ref_seed"
    );
    assert!(
        names.iter().any(|m| m.contains("snap_a")),
        "Should find snap_a"
    );
}

#[test]
fn test_fallback_includes_apply_across_models_and_sources() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
        ],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/marts/sources.yml",
            ),
            manifest_source(
                "source.test_project.ext.events",
                "events",
                "ext",
                "models/raw/sources.yml",
            ),
        ],
    );

    let config = catalog_config_with_includes(&["models/marts"], &["models", "sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "Should only include objects in models/marts, got: {findings:?}"
    );
    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(
        names.iter().any(|m| m.contains("mart_orders")),
        "Should find mart_orders"
    );
    assert!(
        names.iter().any(|m| m.contains("users")),
        "Should find users source (in marts)"
    );
}

// =========================================================================
// Edge cases
// =========================================================================

#[test]
fn test_fallback_no_excludes_all_findings_returned() {
    let manifest = build_manifest(
        &[
            manifest_node("model.test_project.a", "a", "model", "models/a.sql"),
            manifest_node("model.test_project.b", "b", "model", "models/b.sql"),
        ],
        &[],
    );
    let config = r#"
catalog_tests:
  - name: "columns_have_description"
    type: "columns_have_description"
    severity: "error"
    applies_to:
      - "models"
"#;
    let env = TestEnvironment::new(&manifest, config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "No excludes should return all findings, got: {findings:?}"
    );
}

#[test]
fn test_fallback_exclude_all_no_findings() {
    let manifest = build_manifest(
        &[
            manifest_node("model.test_project.a", "a", "model", "models/a.sql"),
            manifest_node("model.test_project.b", "b", "model", "models/b.sql"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        0,
        "Excluding all models should return 0 findings, got: {findings:?}"
    );
}

#[test]
fn test_fallback_exclude_by_filename_only() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.orders",
                "orders",
                "model",
                "models/marts/orders.sql",
            ),
            manifest_node(
                "model.test_project.customers",
                "customers",
                "model",
                "models/marts/customers.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["orders"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Filename exclude should work, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("customers"));
}

#[test]
fn test_catalog_node_exclude_by_filename_only() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.orders",
                "orders",
                "model",
                "models/marts/orders.sql",
            ),
            manifest_node(
                "model.test_project.customers",
                "customers",
                "model",
                "models/marts/customers.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.orders", "orders"),
            catalog_node("model.test_project.customers", "customers"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["orders"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Filename exclude should work in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("customers"));
}

// =========================================================================
// Multiple exclude patterns
// =========================================================================

#[test]
fn test_fallback_multiple_exclude_patterns() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["models/raw", "models/staging"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Multiple excludes should work, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

// =========================================================================
// name: prefix tests — Fallback rules
// =========================================================================

#[test]
fn test_fallback_node_exclude_by_name() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["name:raw_orders"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: exclude should filter out raw_orders, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_include_by_name() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["name:mart_orders"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: include should only match mart_orders, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_include_by_name_glob() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_customers",
                "stg_customers",
                "model",
                "models/staging/stg_customers.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["name:stg_*"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "name:stg_* should match both staging models, got: {findings:?}"
    );
    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("stg_orders")));
    assert!(names.iter().any(|m| m.contains("stg_customers")));
}

#[test]
fn test_fallback_source_exclude_by_name() {
    // Sources sharing the same file path — name: filtering differentiates them
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources.yml",
            ),
            manifest_source(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources.yml",
            ),
        ],
    );
    let config = catalog_config_with_excludes(&["name:users"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: exclude should filter out users source, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders"));
}

#[test]
fn test_fallback_source_include_by_name() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources.yml",
            ),
            manifest_source(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources.yml",
            ),
            manifest_source(
                "source.test_project.raw.products",
                "products",
                "raw",
                "models/sources.yml",
            ),
        ],
    );
    let config = catalog_config_with_includes(&["name:orders"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: include should only match orders source, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders"));
}

// =========================================================================
// tag: prefix tests — Fallback rules
// =========================================================================

#[test]
fn test_fallback_node_exclude_by_tag() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
                &["deprecated"],
            ),
            manifest_node_with_tags(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
                &["active"],
            ),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["tag:deprecated"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "tag: exclude should filter out deprecated model, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_fallback_node_include_by_tag() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
                &["raw"],
            ),
            manifest_node_with_tags(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
                &["finance"],
            ),
            manifest_node_with_tags(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
                &["finance"],
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["tag:finance"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "tag: include should match both finance models, got: {findings:?}"
    );
    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("mart_orders")));
    assert!(names.iter().any(|m| m.contains("stg_orders")));
}

#[test]
fn test_fallback_node_include_by_tag_glob() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.pii_model",
                "pii_model",
                "model",
                "models/pii_model.sql",
                &["pii_email"],
            ),
            manifest_node_with_tags(
                "model.test_project.pii_phone_model",
                "pii_phone_model",
                "model",
                "models/pii_phone_model.sql",
                &["pii_phone"],
            ),
            manifest_node_with_tags(
                "model.test_project.safe_model",
                "safe_model",
                "model",
                "models/safe_model.sql",
                &["public"],
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["tag:pii_*"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "tag:pii_* should match both PII models, got: {findings:?}"
    );
    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("pii_model")));
    assert!(names.iter().any(|m| m.contains("pii_phone_model")));
}

#[test]
fn test_fallback_source_exclude_by_tag() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source_with_tags(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources.yml",
                &["deprecated"],
            ),
            manifest_source_with_tags(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources.yml",
                &["active"],
            ),
        ],
    );
    let config = catalog_config_with_excludes(&["tag:deprecated"], &["sources"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "tag: exclude should filter out deprecated source, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders"));
}

// =========================================================================
// Mixed name: + tag: + path patterns — Fallback rules
// =========================================================================

#[test]
fn test_fallback_mixed_path_include_name_exclude() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_customers",
                "mart_customers",
                "model",
                "models/marts/mart_customers.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes_and_excludes(
        &["models/marts"],
        &["name:mart_orders"],
        &["models"],
    );
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include marts but exclude mart_orders by name, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_customers"));
}

#[test]
fn test_fallback_mixed_tag_include_name_exclude() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.model_a",
                "model_a",
                "model",
                "models/model_a.sql",
                &["finance"],
            ),
            manifest_node_with_tags(
                "model.test_project.model_b",
                "model_b",
                "model",
                "models/model_b.sql",
                &["finance"],
            ),
            manifest_node_with_tags(
                "model.test_project.model_c",
                "model_c",
                "model",
                "models/model_c.sql",
                &["marketing"],
            ),
        ],
        &[],
    );
    let config =
        catalog_config_with_includes_and_excludes(&["tag:finance"], &["name:model_b"], &["models"]);
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include finance-tagged but exclude model_b by name, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("model_a"));
}

#[test]
fn test_fallback_mixed_path_include_tag_exclude() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
                &["deprecated"],
            ),
            manifest_node_with_tags(
                "model.test_project.mart_customers",
                "mart_customers",
                "model",
                "models/marts/mart_customers.sql",
                &["active"],
            ),
        ],
        &[],
    );
    let config = catalog_config_with_includes_and_excludes(
        &["models/marts"],
        &["tag:deprecated"],
        &["models"],
    );
    let env = TestEnvironment::new(&manifest, &config);
    let findings = env
        .run_catalog_fallback_rules(false)
        .expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "Should include marts but exclude deprecated-tagged, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_customers"));
}

// =========================================================================
// name: and tag: prefix tests — Catalog rules (with actual catalog)
// =========================================================================

#[test]
fn test_catalog_node_exclude_by_name() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["name:raw_orders"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: exclude should work in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_include_by_name() {
    let manifest = build_manifest(
        &[
            manifest_node(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
            ),
            manifest_node(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
            ),
            manifest_node(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
            catalog_node("model.test_project.stg_orders", "stg_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["name:mart_orders"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: include should work in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_exclude_by_tag() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
                &["deprecated"],
            ),
            manifest_node_with_tags(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
                &["active"],
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_excludes(&["tag:deprecated"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "tag: exclude should work in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("mart_orders"));
}

#[test]
fn test_catalog_node_include_by_tag() {
    let manifest = build_manifest(
        &[
            manifest_node_with_tags(
                "model.test_project.raw_orders",
                "raw_orders",
                "model",
                "models/raw/raw_orders.sql",
                &["raw"],
            ),
            manifest_node_with_tags(
                "model.test_project.mart_orders",
                "mart_orders",
                "model",
                "models/marts/mart_orders.sql",
                &["finance"],
            ),
            manifest_node_with_tags(
                "model.test_project.stg_orders",
                "stg_orders",
                "model",
                "models/staging/stg_orders.sql",
                &["finance"],
            ),
        ],
        &[],
    );
    let catalog = build_catalog(
        &[
            catalog_node("model.test_project.raw_orders", "raw_orders"),
            catalog_node("model.test_project.mart_orders", "mart_orders"),
            catalog_node("model.test_project.stg_orders", "stg_orders"),
        ],
        &[],
    );
    let config = catalog_config_with_includes(&["tag:finance"], &["models"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        2,
        "tag: include should match both finance models in catalog mode, got: {findings:?}"
    );
    let names: Vec<&str> = findings.iter().map(|(r, _)| r.message.as_str()).collect();
    assert!(names.iter().any(|m| m.contains("mart_orders")));
    assert!(names.iter().any(|m| m.contains("stg_orders")));
}

#[test]
fn test_catalog_source_exclude_by_name() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources.yml",
            ),
            manifest_source(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources.yml",
            ),
        ],
    );
    let catalog = build_catalog(
        &[],
        &[
            catalog_source("source.test_project.raw.users", "users"),
            catalog_source("source.test_project.raw.orders", "orders"),
        ],
    );
    let config = catalog_config_with_excludes(&["name:users"], &["sources"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "name: exclude should work for sources in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders"));
}

#[test]
fn test_catalog_source_exclude_by_tag() {
    let manifest = build_manifest(
        &[],
        &[
            manifest_source_with_tags(
                "source.test_project.raw.users",
                "users",
                "raw",
                "models/sources.yml",
                &["deprecated"],
            ),
            manifest_source_with_tags(
                "source.test_project.raw.orders",
                "orders",
                "raw",
                "models/sources.yml",
                &["active"],
            ),
        ],
    );
    let catalog = build_catalog(
        &[],
        &[
            catalog_source("source.test_project.raw.users", "users"),
            catalog_source("source.test_project.raw.orders", "orders"),
        ],
    );
    let config = catalog_config_with_excludes(&["tag:deprecated"], &["sources"]);
    let env = TestEnvironment::new_with_catalog(&manifest, &catalog, &config);
    let findings = env.run_catalog_rules(false).expect("should not error");

    assert_eq!(
        findings.len(),
        1,
        "tag: exclude should work for sources in catalog mode, got: {findings:?}"
    );
    assert!(findings[0].0.message.contains("orders"));
}
