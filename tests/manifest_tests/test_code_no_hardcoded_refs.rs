use crate::common::TestEnvironment;

#[test]
fn test_code_no_hardcoded_refs_passes_with_refs() {
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
    "model.test.good_model": {
      "database": "db",
      "schema": "public",
      "name": "good_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "good_model.sql",
      "original_file_path": "models/good_model.sql",
      "unique_id": "model.test.good_model",
      "fqn": ["test", "good_model"],
      "alias": "good_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Good model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id, b.name FROM {{ ref('users') }} a JOIN {{ ref('orders') }} b ON a.id = b.user_id"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_no_hardcoded_refs_fails_with_schema_dot_table() {
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
    "model.test.bad_model": {
      "database": "db",
      "schema": "public",
      "name": "bad_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "bad_model.sql",
      "original_file_path": "models/bad_model.sql",
      "unique_id": "model.test.bad_model",
      "fqn": ["test", "bad_model"],
      "alias": "bad_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Bad model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id, name FROM analytics.orders WHERE active = true"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Model");
    assert_eq!(findings[0].0.rule_name, "no_hardcoded_refs");
    assert!(findings[0].0.message.contains("analytics.orders"));
    assert!(findings[0].0.message.contains("hardcoded table reference"));
}

#[test]
fn test_code_no_hardcoded_refs_fails_with_join_hardcoded() {
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
    "model.test.join_model": {
      "database": "db",
      "schema": "public",
      "name": "join_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "join_model.sql",
      "original_file_path": "models/join_model.sql",
      "unique_id": "model.test.join_model",
      "fqn": ["test", "join_model"],
      "alias": "join_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with hardcoded join",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM {{ ref('users') }} a LEFT JOIN raw.customers c ON a.id = c.user_id"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "warning"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert!(findings[0].0.message.contains("raw.customers"));
}

#[test]
fn test_code_no_hardcoded_refs_commented_ref_passes() {
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
    "model.test.commented_model": {
      "database": "db",
      "schema": "public",
      "name": "commented_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "commented_model.sql",
      "original_file_path": "models/commented_model.sql",
      "unique_id": "model.test.commented_model",
      "fqn": ["test", "commented_model"],
      "alias": "commented_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with commented hardcoded ref",
      "columns": {},
      "meta": {},
      "raw_code": "-- FROM analytics.orders\n/* JOIN raw.customers ON ... */\nSELECT id FROM {{ ref('orders') }}"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_no_hardcoded_refs_three_part_ref_fails() {
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
    "model.test.three_part_model": {
      "database": "db",
      "schema": "public",
      "name": "three_part_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "three_part_model.sql",
      "original_file_path": "models/three_part_model.sql",
      "unique_id": "model.test.three_part_model",
      "fqn": ["test", "three_part_model"],
      "alias": "three_part_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with three-part ref",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM db.analytics.orders WHERE 1=1"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("db.analytics.orders"));
}

#[test]
fn test_code_no_hardcoded_refs_multiline_from_fails() {
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
    "model.test.multiline_model": {
      "database": "db",
      "schema": "public",
      "name": "multiline_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "multiline_model.sql",
      "original_file_path": "models/multiline_model.sql",
      "unique_id": "model.test.multiline_model",
      "fqn": ["test", "multiline_model"],
      "alias": "multiline_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with multiline FROM",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id\nFROM\n  analytics.orders\nWHERE 1=1"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("analytics.orders"));
}

#[test]
fn test_code_no_hardcoded_refs_cte_passes() {
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
    "model.test.cte_model": {
      "database": "db",
      "schema": "public",
      "name": "cte_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "cte_model.sql",
      "original_file_path": "models/cte_model.sql",
      "unique_id": "model.test.cte_model",
      "fqn": ["test", "cte_model"],
      "alias": "cte_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with CTE",
      "columns": {},
      "meta": {},
      "raw_code": "WITH my_cte AS (\n  SELECT * FROM {{ ref('users') }}\n)\nSELECT id FROM my_cte WHERE active = true"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_no_hardcoded_refs_mixed_models() {
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
    "model.test.good_model": {
      "database": "db",
      "schema": "public",
      "name": "good_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "good_model.sql",
      "original_file_path": "models/good_model.sql",
      "unique_id": "model.test.good_model",
      "fqn": ["test", "good_model"],
      "alias": "good_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Good model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM {{ ref('orders') }}"
    },
    "model.test.bad_model": {
      "database": "db",
      "schema": "public",
      "name": "bad_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "bad_model.sql",
      "original_file_path": "models/bad_model.sql",
      "unique_id": "model.test.bad_model",
      "fqn": ["test", "bad_model"],
      "alias": "bad_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Bad model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM analytics.orders a JOIN raw.customers b ON a.id = b.id"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // Only bad_model should fail
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("bad_model"));
}

#[test]
fn test_code_no_hardcoded_refs_no_code_skips() {
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
    "model.test.no_code_model": {
      "database": "db",
      "schema": "public",
      "name": "no_code_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "no_code_model.sql",
      "original_file_path": "models/no_code_model.sql",
      "unique_id": "model.test.no_code_model",
      "fqn": ["test", "no_code_model"],
      "alias": "no_code_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model without code",
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

    let config = r#"
manifest_tests:
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_code_no_hardcoded_refs_quoted_identifiers_fail() {
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
    "model.test.quoted_model": {
      "database": "db",
      "schema": "public",
      "name": "quoted_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "quoted_model.sql",
      "original_file_path": "models/quoted_model.sql",
      "unique_id": "model.test.quoted_model",
      "fqn": ["test", "quoted_model"],
      "alias": "quoted_model",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Model with quoted identifiers",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM \"analytics\".\"orders\" WHERE 1=1"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_code_no_hardcoded_refs_combination_three_part_and_quoted() {
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
    "model.test.bracket_quoted": {
      "database": "db",
      "schema": "public",
      "name": "bracket_quoted",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "bracket_quoted.sql",
      "original_file_path": "models/bracket_quoted.sql",
      "unique_id": "model.test.bracket_quoted",
      "fqn": ["test", "bracket_quoted"],
      "alias": "bracket_quoted",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "aaa"},
      "tags": [],
      "description": "Bracket-quoted identifiers",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM [analytics].[orders] WHERE 1=1"
    },
    "model.test.backtick_quoted": {
      "database": "db",
      "schema": "public",
      "name": "backtick_quoted",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "backtick_quoted.sql",
      "original_file_path": "models/backtick_quoted.sql",
      "unique_id": "model.test.backtick_quoted",
      "fqn": ["test", "backtick_quoted"],
      "alias": "backtick_quoted",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "bbb"},
      "tags": [],
      "description": "Backtick-quoted identifiers",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM `analytics`.`orders` WHERE 1=1"
    },
    "model.test.three_part_quoted": {
      "database": "db",
      "schema": "public",
      "name": "three_part_quoted",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "three_part_quoted.sql",
      "original_file_path": "models/three_part_quoted.sql",
      "unique_id": "model.test.three_part_quoted",
      "fqn": ["test", "three_part_quoted"],
      "alias": "three_part_quoted",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "ccc"},
      "tags": [],
      "description": "Three-part quoted identifiers",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM \"mydb\".\"analytics\".\"orders\" WHERE 1=1"
    },
    "model.test.three_part_join": {
      "database": "db",
      "schema": "public",
      "name": "three_part_join",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "three_part_join.sql",
      "original_file_path": "models/three_part_join.sql",
      "unique_id": "model.test.three_part_join",
      "fqn": ["test", "three_part_join"],
      "alias": "three_part_join",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "ddd"},
      "tags": [],
      "description": "Three-part ref in a JOIN",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT a.id FROM {{ ref('users') }} a LEFT JOIN prod.raw.customers c ON a.id = c.user_id"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);

    // All 4 models should fail
    assert_eq!(findings.len(), 4);

    let messages: Vec<&str> = findings.iter().map(|f| f.0.message.as_str()).collect();
    assert!(messages.iter().any(|m| m.contains("[analytics].[orders]")));
    assert!(messages.iter().any(|m| m.contains("`analytics`.`orders`")));
    assert!(messages
        .iter()
        .any(|m| m.contains("\"mydb\".\"analytics\".\"orders\"")));
    assert!(messages.iter().any(|m| m.contains("prod.raw.customers")));
}

#[test]
fn test_code_no_hardcoded_refs_bracket_three_part_fails() {
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
    "model.test.bracket_three_part": {
      "database": "db",
      "schema": "public",
      "name": "bracket_three_part",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "bracket_three_part.sql",
      "original_file_path": "models/bracket_three_part.sql",
      "unique_id": "model.test.bracket_three_part",
      "fqn": ["test", "bracket_three_part"],
      "alias": "bracket_three_part",
      "depends_on": { "nodes": [] },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Bracket-quoted three-part ref",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT id FROM [mydb].[analytics].[orders] WHERE 1=1"
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
  - name: "no_hardcoded_refs"
    type: code_no_hardcoded_refs
    severity: "error"
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, config);
    let findings = env.run_manifest_rules(false);
    assert_eq!(findings.len(), 1);
    assert!(findings[0]
        .0
        .message
        .contains("[mydb].[analytics].[orders]"));
}
