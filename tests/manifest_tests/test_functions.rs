use crate::common::TestEnvironment;

/// Minimal manifest with a single function that has all fields populated.
/// Use `replace()` on this to tweak individual fields for specific tests.
const FUNCTION_MANIFEST: &str = r#"{
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
  "unit_tests": {},
  "functions": {
    "function.dbtective.is_positive_int": {
      "returns": { "data_type": "integer", "description": null },
      "database": "dbt",
      "schema": "public_udf_schema",
      "name": "is_positive_int",
      "resource_type": "function",
      "package_name": "test_project",
      "path": "is_positive_int.sql",
      "original_file_path": "functions/is_positive_int.sql",
      "unique_id": "function.dbtective.is_positive_int",
      "fqn": ["dbtective", "is_positive_int"],
      "alias": "is_positive_int",
      "checksum": {
        "name": "sha256",
        "checksum": "37e488ff327a3e35ab6f9d7962f4e6f3fdfc98104a50e4e47328df609d4b0e3b"
      },
      "config": {
        "enabled": true,
        "alias": null,
        "schema": "udf_schema",
        "database": "dbt",
        "tags": [],
        "meta": {},
        "group": null,
        "materialized": "function",
        "incremental_strategy": null,
        "batch_size": null,
        "lookback": 1,
        "begin": null,
        "persist_docs": {},
        "post-hook": [],
        "pre-hook": [],
        "quoting": {},
        "column_types": {},
        "full_refresh": null,
        "unique_key": null,
        "on_schema_change": "ignore",
        "on_configuration_change": "apply",
        "grants": {},
        "packages": [],
        "docs": { "show": true, "node_color": null },
        "contract": { "enforced": false, "alias_types": true },
        "event_time": null,
        "concurrent_batches": null,
        "type": "scalar",
        "volatility": "deterministic",
        "runtime_version": null,
        "entry_point": null
      },
      "tags": ["udf", "data_quality"],
      "description": "My UDF that returns 1 if a string represents a naked positive integer.",
      "columns": {},
      "meta": { "owner": "data-team" },
      "group": null,
      "docs": { "show": true, "node_color": null },
      "patch_path": "dbtective://functions/schema.yml",
      "build_path": null,
      "unrendered_config": {
        "schema": "udf_schema",
        "database": "dbt",
        "volatility": "deterministic"
      },
      "created_at": 1772188537.320818,
      "relation_name": null,
      "raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')",
      "doc_blocks": [],
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "functions": [],
      "depends_on": { "macros": [], "nodes": ["model.dbtective.users"] },
      "compiled_path": null,
      "contract": {
        "enforced": false,
        "alias_types": true,
        "checksum": null
      },
      "arguments": [
        {
          "name": "a_string",
          "data_type": "text",
          "description": "The string to check",
          "default_value": "'1'"
        }
      ]
    }
  }
}"#;

// ─── has_description ────────────────────────────────────────────────────────

#[test]
fn test_function_has_description_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_has_description_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert_eq!(findings[0].0.object_type, "Function");
    assert_eq!(findings[0].0.severity, "FAIL");
}

#[test]
fn test_function_description_min_length_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": "Short""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_long_desc"
    type: "has_description"
    min_length: 10
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("too short"));
}

#[test]
fn test_function_description_forbidden_substrings_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": "TODO: describe this function""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_no_placeholder"
    type: "has_description"
    forbidden_substrings: ["TODO", "placeholder"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("forbidden substring"));
}

// ─── name_convention ────────────────────────────────────────────────────────

#[test]
fn test_function_name_convention_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_snake_case"
    type: "name_convention"
    pattern: "snake_case"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_name_convention_fails() {
    let manifest =
        FUNCTION_MANIFEST.replace(r#""name": "is_positive_int""#, r#""name": "IsPositiveInt""#);

    let config = r#"
manifest_tests:
  - name: "functions_snake_case"
    type: "name_convention"
    pattern: "snake_case"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── has_tags ───────────────────────────────────────────────────────────────

#[test]
fn test_function_has_tags_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_need_udf_tag"
    type: "has_tags"
    required_tags: ["udf"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_has_tags_fails() {
    let config = r#"
manifest_tests:
  - name: "functions_need_missing_tag"
    type: "has_tags"
    required_tags: ["nonexistent_tag"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

#[test]
fn test_function_has_tags_any_criteria_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_any_tag"
    type: "has_tags"
    required_tags: ["nonexistent", "udf"]
    criteria: "any"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

// ─── has_metadata_keys ──────────────────────────────────────────────────────

#[test]
fn test_function_has_metadata_keys_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_have_owner"
    type: "has_metadata_keys"
    required_keys: ["owner"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_has_metadata_keys_fails() {
    let config = r#"
manifest_tests:
  - name: "functions_have_domain"
    type: "has_metadata_keys"
    required_keys: ["domain"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── has_refs ───────────────────────────────────────────────────────────────

#[test]
fn test_function_has_refs_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_have_refs"
    type: "has_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    // The default manifest has depends_on.nodes with a model ref
    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_has_refs_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""depends_on": { "macros": [], "nodes": ["model.dbtective.users"] }"#,
        r#""depends_on": { "macros": [], "nodes": [] }"#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_refs"
    type: "has_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── code_max_lines ─────────────────────────────────────────────────────────

#[test]
fn test_function_code_max_lines_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_max_lines"
    type: "code_max_lines"
    max_lines: 10
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_code_max_lines_fails() {
    let long_code = (0..20)
        .map(|i| format!("SELECT {i}"))
        .collect::<Vec<_>>()
        .join("\\n");
    let manifest = FUNCTION_MANIFEST.replace(
        r#""raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')""#,
        &format!(r#""raw_code": "{long_code}""#),
    );

    let config = r#"
manifest_tests:
  - name: "functions_max_lines"
    type: "code_max_lines"
    max_lines: 5
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── code_forbidden_patterns ────────────────────────────────────────────────

#[test]
fn test_function_code_forbidden_patterns_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_no_select_star"
    type: "code_forbidden_patterns"
    forbidden_patterns: ["SELECT *"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_code_forbidden_patterns_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')""#,
        r#""raw_code": "SELECT * FROM users""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_no_select_star"
    type: "code_forbidden_patterns"
    forbidden_patterns: ["SELECT *"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── code_contains_refs ─────────────────────────────────────────────────────

#[test]
fn test_function_code_contains_refs_passes() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')""#,
        r#""raw_code": "SELECT * FROM {{ ref('users') }}""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_code_has_refs"
    type: "code_contains_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_code_contains_refs_fails() {
    let config = r#"
manifest_tests:
  - name: "functions_code_has_refs"
    type: "code_contains_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    // The default manifest raw_code has no ref()/source() calls
    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── code_no_hardcoded_refs ─────────────────────────────────────────────────

#[test]
fn test_function_code_no_hardcoded_refs_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_no_hardcoded"
    type: "code_no_hardcoded_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    // The default raw_code has no schema.table patterns
    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_code_no_hardcoded_refs_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')""#,
        r#""raw_code": "SELECT id FROM analytics.orders WHERE active = true""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_no_hardcoded"
    type: "code_no_hardcoded_refs"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── code_max_joins ─────────────────────────────────────────────────────────

#[test]
fn test_function_code_max_joins_passes() {
    let config = r#"
manifest_tests:
  - name: "functions_max_joins"
    type: "code_max_joins"
    max_joins: 3
    severity: "error"
    applies_to:
      - "functions"
"#;

    // Default raw_code has no JOINs
    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_code_max_joins_fails() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""raw_code": "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')""#,
        r#""raw_code": "SELECT a.id FROM users a JOIN orders b ON a.id = b.uid JOIN products c ON b.pid = c.id JOIN categories d ON c.cid = d.id""#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_max_joins"
    type: "code_max_joins"
    max_joins: 2
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── allowed_subfolders ─────────────────────────────────────────────────────

#[test]
fn test_function_allowed_subfolders_passes() {
    // The function is at functions/is_positive_int.sql
    // The allowed_subfolders rule checks subdirectories within the resource type folder
    // Since the file is directly in functions/, we need a subfolder structure for it to pass
    let manifest = FUNCTION_MANIFEST
        .replace(
            r#""original_file_path": "functions/is_positive_int.sql""#,
            r#""original_file_path": "functions/math/is_positive_int.sql""#,
        )
        .replace(
            r#""patch_path": "dbtective://functions/schema.yml""#,
            r#""patch_path": "dbtective://functions/math/schema.yml""#,
        );

    let config = r#"
manifest_tests:
  - name: "functions_in_allowed_folders"
    type: "allowed_subfolders"
    allowed_subfolders: ["math", "string"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Expected no findings, but got: {findings:?}"
    );
}

#[test]
fn test_function_allowed_subfolders_fails() {
    // The function is at functions/is_positive_int.sql (directly in functions/, not in a subfolder)
    let config = r#"
manifest_tests:
  - name: "functions_in_allowed_folders"
    type: "allowed_subfolders"
    allowed_subfolders: ["math", "string"]
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── includes / excludes ────────────────────────────────────────────────────

#[test]
fn test_function_includes_filters_correctly() {
    // Include only functions in a specific path that doesn't match our function
    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
    includes:
      - "^functions/math/"
"#;

    // The function is at functions/is_positive_int.sql - doesn't match the include
    let env = TestEnvironment::new(FUNCTION_MANIFEST, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Function should be filtered out by includes"
    );
}

#[test]
fn test_function_includes_matches() {
    // Remove description so the rule would fail if the function is included
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
    includes:
      - "^functions/"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Function should match include pattern and fail"
    );
    assert_eq!(findings[0].0.object_type, "Function");
}

#[test]
fn test_function_excludes_filters_correctly() {
    // Remove description so the rule would fail if the function is NOT excluded
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
    excludes:
      - "is_positive_int"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Function should be excluded by excludes pattern"
    );
}

#[test]
fn test_function_excludes_does_not_match() {
    // Remove description so the rule would fail
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
    excludes:
      - "^functions/math/"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        1,
        "Function should NOT be excluded and should fail"
    );
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── applies_to filtering ───────────────────────────────────────────────────

#[test]
fn test_function_not_checked_when_not_in_applies_to() {
    // Remove description so it would fail if tested
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "models_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(
        findings.len(),
        0,
        "Function should not be checked when applies_to only has models"
    );
}

// ─── multiple functions ─────────────────────────────────────────────────────

#[test]
#[allow(clippy::too_many_lines)]
fn test_multiple_functions_mixed_results() {
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
  "unit_tests": {},
  "functions": {
    "function.proj.good_func": {
      "returns": { "data_type": "integer", "description": null },
      "database": "dbt",
      "schema": "public",
      "name": "good_func",
      "resource_type": "function",
      "package_name": "test_project",
      "path": "good_func.sql",
      "original_file_path": "functions/good_func.sql",
      "unique_id": "function.proj.good_func",
      "fqn": ["proj", "good_func"],
      "alias": "good_func",
      "checksum": {"name": "sha256", "checksum": "abc"},
      "config": {
        "enabled": true,
        "materialized": "function",
        "on_schema_change": "ignore",
        "post-hook": [],
        "pre-hook": [],
        "packages": []
      },
      "tags": [],
      "description": "A well-described function that does something useful.",
      "columns": {},
      "meta": {},
      "group": null,
      "docs": { "show": true, "node_color": null },
      "patch_path": null,
      "build_path": null,
      "unrendered_config": {},
      "created_at": 1.0,
      "relation_name": null,
      "raw_code": "SELECT 1",
      "doc_blocks": [],
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "functions": [],
      "depends_on": { "macros": [], "nodes": [] },
      "compiled_path": null,
      "contract": { "enforced": false, "alias_types": true, "checksum": null },
      "arguments": []
    },
    "function.proj.bad_func": {
      "returns": { "data_type": "text", "description": null },
      "database": "dbt",
      "schema": "public",
      "name": "bad_func",
      "resource_type": "function",
      "package_name": "test_project",
      "path": "bad_func.sql",
      "original_file_path": "functions/bad_func.sql",
      "unique_id": "function.proj.bad_func",
      "fqn": ["proj", "bad_func"],
      "alias": "bad_func",
      "checksum": {"name": "sha256", "checksum": "def"},
      "config": {
        "enabled": true,
        "materialized": "function",
        "on_schema_change": "ignore",
        "post-hook": [],
        "pre-hook": [],
        "packages": []
      },
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "group": null,
      "docs": { "show": true, "node_color": null },
      "patch_path": null,
      "build_path": null,
      "unrendered_config": {},
      "created_at": 1.0,
      "relation_name": null,
      "raw_code": "SELECT 1",
      "doc_blocks": [],
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "functions": [],
      "depends_on": { "macros": [], "nodes": [] },
      "compiled_path": null,
      "contract": { "enforced": false, "alias_types": true, "checksum": null },
      "arguments": []
    }
  }
}"#;

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1, "Only bad_func should fail: {findings:?}");
    assert_eq!(findings[0].0.object_type, "Function");
    assert!(findings[0].0.message.contains("bad_func"));
}

// ─── functions alongside other objects ──────────────────────────────────────

#[test]
#[allow(clippy::too_many_lines)]
fn test_function_rule_does_not_affect_other_objects() {
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
    "model.test.my_model": {
      "database": "db",
      "schema": "public",
      "name": "my_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "model.sql",
      "original_file_path": "models/model.sql",
      "unique_id": "model.test.my_model",
      "fqn": ["test", "my_model"],
      "alias": "my_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "",
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
  "unit_tests": {},
  "functions": {
    "function.test.my_func": {
      "returns": { "data_type": "integer", "description": null },
      "database": "dbt",
      "schema": "public",
      "name": "my_func",
      "resource_type": "function",
      "package_name": "test_project",
      "path": "my_func.sql",
      "original_file_path": "functions/my_func.sql",
      "unique_id": "function.test.my_func",
      "fqn": ["test", "my_func"],
      "alias": "my_func",
      "checksum": {"name": "sha256", "checksum": "def"},
      "config": {
        "enabled": true,
        "materialized": "function",
        "on_schema_change": "ignore",
        "post-hook": [],
        "pre-hook": [],
        "packages": []
      },
      "tags": [],
      "description": "",
      "columns": {},
      "meta": {},
      "group": null,
      "docs": { "show": true, "node_color": null },
      "patch_path": null,
      "build_path": null,
      "unrendered_config": {},
      "created_at": 1.0,
      "relation_name": null,
      "raw_code": "SELECT 1",
      "doc_blocks": [],
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "functions": [],
      "depends_on": { "macros": [], "nodes": [] },
      "compiled_path": null,
      "contract": { "enforced": false, "alias_types": true, "checksum": null },
      "arguments": []
    }
  }
}"#;

    // Rule only targets functions, NOT models
    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    // Only the function should fail, not the model (even though model also has empty desc)
    assert_eq!(
        findings.len(),
        1,
        "Expected 1 finding, but got: {findings:?}"
    );
    assert_eq!(findings[0].0.object_type, "Function");
}

// ─── severity ───────────────────────────────────────────────────────────────

#[test]
fn test_function_warning_severity() {
    let manifest = FUNCTION_MANIFEST.replace(
        r#""description": "My UDF that returns 1 if a string represents a naked positive integer.""#,
        r#""description": """#,
    );

    let config = r#"
manifest_tests:
  - name: "functions_have_description"
    type: "has_description"
    severity: "warning"
    applies_to:
      - "functions"
"#;

    let env = TestEnvironment::new(&manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
}
