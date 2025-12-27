mod common;

use common::TestEnvironment;
#[allow(clippy::too_many_lines)]
const fn manifest_with_materializations() -> &'static str {
    r#"{
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
    "model.test_project.table_model": {
      "database": "analytics",
      "schema": "public",
      "name": "table_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "table_model.sql",
      "original_file_path": "models/table_model.sql",
      "unique_id": "model.test_project.table_model",
      "fqn": ["test_project", "table_model"],
      "alias": "table_model",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": {
        "enabled": true,
        "materialized": "table",
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
      "relation_name": "analytics.public.table_model",
      "raw_code": "select 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null, "alias_types": false}
    },
    "model.test_project.view_model": {
      "database": "analytics",
      "schema": "public",
      "name": "view_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "view_model.sql",
      "original_file_path": "models/view_model.sql",
      "unique_id": "model.test_project.view_model",
      "fqn": ["test_project", "view_model"],
      "alias": "view_model",
      "checksum": {"name": "sha256", "checksum": "def456"},
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
      "relation_name": "analytics.public.view_model",
      "raw_code": "select 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null, "alias_types": false}
    },
    "model.test_project.incremental_model": {
      "database": "analytics",
      "schema": "public",
      "name": "incremental_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "incremental_model.sql",
      "original_file_path": "models/incremental_model.sql",
      "unique_id": "model.test_project.incremental_model",
      "fqn": ["test_project", "incremental_model"],
      "alias": "incremental_model",
      "checksum": {"name": "sha256", "checksum": "ghi789"},
      "config": {
        "enabled": true,
        "materialized": "incremental",
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
      "relation_name": "analytics.public.incremental_model",
      "raw_code": "select 1",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null, "alias_types": false}
    },
    "seed.test_project.my_seed": {
      "database": "analytics",
      "schema": "public",
      "name": "my_seed",
      "resource_type": "seed",
      "package_name": "test_project",
      "path": "my_seed.csv",
      "original_file_path": "seeds/my_seed.csv",
      "unique_id": "seed.test_project.my_seed",
      "fqn": ["test_project", "my_seed"],
      "alias": "my_seed",
      "checksum": {"name": "sha256", "checksum": "seed123"},
      "config": {
        "enabled": true,
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
      "relation_name": "analytics.public.my_seed",
      "raw_code": "",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null, "alias_types": false}
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
}"#
}

#[test]
fn test_materialization_filter_table_only() {
    let config = r#"
manifest_tests:
  - name: "table_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    model_materializations:
      - "table"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1, "Expected 1 finding only for table model");
}

#[test]
fn test_materialization_filter_view_only() {
    let config = r#"
manifest_tests:
  - name: "view_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    model_materializations:
      - "view"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1, "Expected 1 finding for view model only");
}

#[test]
fn test_materialization_filter_multiple() {
    let config = r#"
manifest_tests:
  - name: "table_and_view_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    model_materializations:
      - "table"
      - "view"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        2,
        "Expected 2 findings for table and view models"
    );
}

#[test]
fn test_materialization_filter_incremental() {
    let config = r#"
manifest_tests:
  - name: "incremental_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    model_materializations:
      - "incremental"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding for incremental model only"
    );
}

#[test]
fn test_no_materialization_filter_applies_to_all_models() {
    let config = r#"
manifest_tests:
  - name: "all_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        3,
        "Expected 3 findings for all models when no materialization filter"
    );
}

#[test]
fn test_materialization_filter_with_other_objects() {
    let config = r#"
manifest_tests:
  - name: "models_and_seeds_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
      - "seeds"
    model_materializations:
      - "table"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        2,
        "Expected 2 findings: 1 for table model, 1 for seed (seeds ignore materialization filter)"
    );
}

#[test]
fn test_materialization_filter_no_match() {
    let config = r#"
manifest_tests:
  - name: "ephemeral_models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    model_materializations:
      - "ephemeral"
"#;

    let env = TestEnvironment::new(manifest_with_materializations(), config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(
        findings.len(),
        0,
        "Expected 0 findings when no models match the materialization filter"
    );
}
