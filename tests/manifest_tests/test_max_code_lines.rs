use crate::common::TestEnvironment;

#[test]
#[allow(clippy::too_many_lines)]
fn test_max_code_lines_models() {
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
    "model.test.short_model": {
      "database": "db",
      "schema": "public",
      "name": "short_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "short_model.sql",
      "original_file_path": "models/short_model.sql",
      "unique_id": "model.test.short_model",
      "fqn": ["test", "short_model"],
      "alias": "short_model",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Short model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT 1\nFROM table"
    },
    "model.test.long_model": {
      "database": "db",
      "schema": "public",
      "name": "long_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "long_model.sql",
      "original_file_path": "models/long_model.sql",
      "unique_id": "model.test.long_model",
      "fqn": ["test", "long_model"],
      "alias": "long_model",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Long model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT 1\nFROM table\nWHERE x = 1\nAND y = 2\nAND z = 3\nAND a = 4"
    },
    "model.test.empty_model": {
      "database": "db",
      "schema": "public",
      "name": "empty_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "empty_model.sql",
      "original_file_path": "models/empty_model.sql",
      "unique_id": "model.test.empty_model",
      "fqn": ["test", "empty_model"],
      "alias": "empty_model",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "ghi"},
      "tags": [],
      "description": "Empty model",
      "columns": {},
      "meta": {},
      "raw_code": ""
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

    // Config 1: max_lines: 5 - should find 2 violations (long_model exceeds, empty_model is empty)
    let max_lines_config = r#"
manifest_tests:
  - name: "models_max_5_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 5
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, max_lines_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 2);

    // Check for the long model violation
    let long_model_finding = findings.iter().find(|f| f.0.message.contains("long_model"));
    assert!(long_model_finding.is_some());
    let finding = &long_model_finding.unwrap().0;
    assert_eq!(finding.severity, "FAIL");
    assert_eq!(finding.object_type, "Model");
    assert_eq!(finding.rule_name, "models_max_5_lines");
    assert!(finding.message.contains("exceeds the maximum allowed"));

    // Check for the empty model violation
    let empty_model_finding = findings
        .iter()
        .find(|f| f.0.message.contains("empty_model"));
    assert!(empty_model_finding.is_some());
    let finding = &empty_model_finding.unwrap().0;
    assert_eq!(finding.severity, "FAIL");
    assert_eq!(finding.object_type, "Model");
    assert_eq!(finding.rule_name, "models_max_5_lines");
    assert!(finding.message.contains("is empty"));

    // Config 2: max_lines: 1 - very strict, should find 3 violations (all models)
    let strict_config = r#"
manifest_tests:
  - name: "models_max_1_line"
    type: max_code_lines
    severity: "warning"
    max_lines: 1
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, strict_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 3);
    for finding in &findings {
        assert_eq!(finding.0.severity, "WARN");
        assert_eq!(finding.0.object_type, "Model");
        assert_eq!(finding.0.rule_name, "models_max_1_line");
    }

    // Config 3: max_lines: 10 - lenient, should find only 1 violation (empty_model)
    let lenient_config = r#"
manifest_tests:
  - name: "models_max_10_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 10
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, lenient_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("empty_model"));
    assert!(findings[0].0.message.contains("is empty"));

    // Config 4: max_lines: 2 - exact limit for short_model, should pass
    let exact_limit_config = r#"
manifest_tests:
  - name: "models_max_2_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 2
    applies_to:
      - "models"
"#;

    let env = TestEnvironment::new(manifest, exact_limit_config);
    let findings = env.run_maniest_rules(false);

    // Should find 2 violations: long_model and empty_model (short_model has exactly 2 lines)
    assert_eq!(findings.len(), 2);
    let has_long = findings.iter().any(|f| f.0.message.contains("long_model"));
    let has_empty = findings.iter().any(|f| f.0.message.contains("empty_model"));
    assert!(has_long);
    assert!(has_empty);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_max_code_lines_snapshots() {
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
    "snapshot.test.short_snapshot": {
      "database": "db",
      "schema": "public",
      "name": "short_snapshot",
      "resource_type": "snapshot",
      "package_name": "test_project",
      "path": "short_snapshot.sql",
      "original_file_path": "snapshots/short_snapshot.sql",
      "unique_id": "snapshot.test.short_snapshot",
      "fqn": ["test", "short_snapshot"],
      "alias": "short_snapshot",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Short snapshot",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT *\nFROM source_table"
    },
    "snapshot.test.long_snapshot": {
      "database": "db",
      "schema": "public",
      "name": "long_snapshot",
      "resource_type": "snapshot",
      "package_name": "test_project",
      "path": "long_snapshot.sql",
      "original_file_path": "snapshots/long_snapshot.sql",
      "unique_id": "snapshot.test.long_snapshot",
      "fqn": ["test", "long_snapshot"],
      "alias": "long_snapshot",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Long snapshot",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT *\nFROM source_table\nWHERE x = 1\nAND y = 2\nAND z = 3\nAND a = 4\nAND b = 5"
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

    // Config 1: max_lines: 3 - should find 1 violation (long_snapshot exceeds)
    let max_lines_config = r#"
manifest_tests:
  - name: "snapshots_max_3_lines"
    type: max_code_lines
    severity: "warning"
    max_lines: 3
    applies_to:
      - "snapshots"
"#;

    let env = TestEnvironment::new(manifest, max_lines_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "WARN");
    assert_eq!(findings[0].0.object_type, "Snapshot");
    assert_eq!(findings[0].0.rule_name, "snapshots_max_3_lines");
    assert!(findings[0].0.message.contains("long_snapshot"));
    assert!(findings[0]
        .0
        .message
        .contains("exceeds the maximum allowed"));

    // Config 2: max_lines: 10 - lenient, should find no violations
    let lenient_config = r#"
manifest_tests:
  - name: "snapshots_max_10_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 10
    applies_to:
      - "snapshots"
"#;

    let env = TestEnvironment::new(manifest, lenient_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 0);

    // Config 3: max_lines: 1 - very strict, should find 2 violations (both snapshots)
    let strict_config = r#"
manifest_tests:
  - name: "snapshots_max_1_line"
    type: max_code_lines
    severity: "error"
    max_lines: 1
    applies_to:
      - "snapshots"
"#;

    let env = TestEnvironment::new(manifest, strict_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 2);
    for finding in &findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert_eq!(finding.0.object_type, "Snapshot");
        assert_eq!(finding.0.rule_name, "snapshots_max_1_line");
    }

    // Config 4: max_lines: 2 - exact limit for short_snapshot
    let exact_config = r#"
manifest_tests:
  - name: "snapshots_max_2_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 2
    applies_to:
      - "snapshots"
"#;

    let env = TestEnvironment::new(manifest, exact_config);
    let findings = env.run_maniest_rules(false);

    // Should find only 1 violation: long_snapshot (short_snapshot has exactly 2 lines)
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("long_snapshot"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_max_code_lines_macros() {
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
  "macros": {
    "macro.test.short_macro": {
      "name": "short_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/short_macro.sql",
      "original_file_path": "macros/short_macro.sql",
      "unique_id": "macro.test.short_macro",
      "macro_sql": "{% macro short_macro() %}\n  SELECT 1\n{% endmacro %}",
      "depends_on": {
        "macros": []
      },
      "description": "Short macro",
      "meta": {},
      "docs": {
        "show": true
      },
      "patch_path": null,
      "arguments": []
    },
    "macro.test.long_macro": {
      "name": "long_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/long_macro.sql",
      "original_file_path": "macros/long_macro.sql",
      "unique_id": "macro.test.long_macro",
      "macro_sql": "{% macro long_macro() %}\n  SELECT *\n  FROM table\n  WHERE x = 1\n  AND y = 2\n  AND z = 3\n  AND a = 4\n{% endmacro %}",
      "depends_on": {
        "macros": []
      },
      "description": "Long macro",
      "meta": {},
      "docs": {
        "show": true
      },
      "patch_path": null,
      "arguments": []
    }
  },
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

    // Config 1: max_lines: 5 - should find 1 violation (long_macro exceeds)
    let max_lines_config = r#"
manifest_tests:
  - name: "macros_max_5_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 5
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, max_lines_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].0.severity, "FAIL");
    assert_eq!(findings[0].0.object_type, "Macro");
    assert_eq!(findings[0].0.rule_name, "macros_max_5_lines");
    assert!(findings[0].0.message.contains("long_macro"));
    assert!(findings[0]
        .0
        .message
        .contains("exceeds the maximum allowed"));

    // Config 2: max_lines: 10 - lenient, should find no violations
    let lenient_config = r#"
manifest_tests:
  - name: "macros_max_10_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 10
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, lenient_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 0);

    // Config 3: max_lines: 8 - exact limit for long_macro (8 lines)
    let exact_config = r#"
manifest_tests:
  - name: "macros_max_8_lines"
    type: max_code_lines
    severity: "warning"
    max_lines: 8
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, exact_config);
    let findings = env.run_maniest_rules(false);

    // Should find no violations: long_macro has exactly 8 lines
    assert_eq!(findings.len(), 0);

    // Config 4: max_lines: 7 - just below long_macro's line count
    let below_limit_config = r#"
manifest_tests:
  - name: "macros_max_7_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 7
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, below_limit_config);
    let findings = env.run_maniest_rules(false);

    // Should find 1 violation: long_macro exceeds 7 lines
    assert_eq!(findings.len(), 1);
    assert!(findings[0].0.message.contains("long_macro"));

    // Config 5: max_lines: 2 - very strict, both macros fail
    let strict_config = r#"
manifest_tests:
  - name: "macros_max_2_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 2
    applies_to:
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, strict_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 2);
    let has_short = findings.iter().any(|f| f.0.message.contains("short_macro"));
    let has_long = findings.iter().any(|f| f.0.message.contains("long_macro"));
    assert!(has_short);
    assert!(has_long);
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_max_code_lines_all_types() {
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
    "model.test.compliant_model": {
      "database": "db",
      "schema": "public",
      "name": "compliant_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "compliant_model.sql",
      "original_file_path": "models/compliant_model.sql",
      "unique_id": "model.test.compliant_model",
      "fqn": ["test", "compliant_model"],
      "alias": "compliant_model",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "abc"},
      "tags": [],
      "description": "Compliant model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT 1"
    },
    "model.test.violating_model": {
      "database": "db",
      "schema": "public",
      "name": "violating_model",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "violating_model.sql",
      "original_file_path": "models/violating_model.sql",
      "unique_id": "model.test.violating_model",
      "fqn": ["test", "violating_model"],
      "alias": "violating_model",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "def"},
      "tags": [],
      "description": "Violating model",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT 1\nFROM table\nWHERE x = 1\nAND y = 2\nAND z = 3\nAND a = 4"
    },
    "snapshot.test.violating_snapshot": {
      "database": "db",
      "schema": "public",
      "name": "violating_snapshot",
      "resource_type": "snapshot",
      "package_name": "test_project",
      "path": "violating_snapshot.sql",
      "original_file_path": "snapshots/violating_snapshot.sql",
      "unique_id": "snapshot.test.violating_snapshot",
      "fqn": ["test", "violating_snapshot"],
      "alias": "violating_snapshot",
      "depends_on": {
        "nodes": []
      },
      "checksum": {"name": "sha256", "checksum": "ghi"},
      "tags": [],
      "description": "Violating snapshot",
      "columns": {},
      "meta": {},
      "raw_code": "SELECT *\nFROM source_table\nWHERE x = 1\nAND y = 2\nAND z = 3\nAND a = 4"
    }
  },
  "sources": {},
  "macros": {
    "macro.test.violating_macro": {
      "name": "violating_macro",
      "resource_type": "macro",
      "package_name": "test_project",
      "path": "macros/violating_macro.sql",
      "original_file_path": "macros/violating_macro.sql",
      "unique_id": "macro.test.violating_macro",
      "macro_sql": "{% macro violating_macro() %}\n  SELECT *\n  FROM table\n  WHERE x = 1\n  AND y = 2\n  AND z = 3\n{% endmacro %}",
      "depends_on": {
        "macros": []
      },
      "description": "Violating macro",
      "meta": {},
      "docs": {
        "show": true
      },
      "patch_path": null,
      "arguments": []
    }
  },
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

    // Config 1: max_lines: 3 - should find 3 violations (one for each violating object type)
    let max_lines_config = r#"
manifest_tests:
  - name: "all_objects_max_3_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 3
    applies_to:
      - "models"
      - "snapshots"
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, max_lines_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 3);

    // Check that we have one of each type
    let model_finding = findings.iter().find(|f| f.0.object_type == "Model");
    let snapshot_finding = findings.iter().find(|f| f.0.object_type == "Snapshot");
    let macro_finding = findings.iter().find(|f| f.0.object_type == "Macro");

    assert!(model_finding.is_some());
    assert!(snapshot_finding.is_some());
    assert!(macro_finding.is_some());

    // Verify all have the correct rule name and severity
    for finding in &findings {
        assert_eq!(finding.0.severity, "FAIL");
        assert_eq!(finding.0.rule_name, "all_objects_max_3_lines");
        assert!(finding.0.message.contains("exceeds the maximum allowed"));
    }

    // Config 2: max_lines: 10 - lenient, should find no violations
    let lenient_config = r#"
manifest_tests:
  - name: "all_objects_max_10_lines"
    type: max_code_lines
    severity: "warning"
    max_lines: 10
    applies_to:
      - "models"
      - "snapshots"
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, lenient_config);
    let findings = env.run_maniest_rules(false);

    assert_eq!(findings.len(), 0);

    // Config 3: Different applies_to - only models and snapshots
    let partial_config = r#"
manifest_tests:
  - name: "models_and_snapshots_max_5_lines"
    type: max_code_lines
    severity: "error"
    max_lines: 5
    applies_to:
      - "models"
      - "snapshots"
"#;

    let env = TestEnvironment::new(manifest, partial_config);
    let findings = env.run_maniest_rules(false);

    // Should find 2 violations: violating_model and violating_snapshot (macro not checked)
    assert_eq!(findings.len(), 2);
    let has_model = findings.iter().any(|f| f.0.object_type == "Model");
    let has_snapshot = findings.iter().any(|f| f.0.object_type == "Snapshot");
    let has_macro = findings.iter().any(|f| f.0.object_type == "Macro");
    assert!(has_model);
    assert!(has_snapshot);
    assert!(!has_macro);

    // Config 4: max_lines: 1 - very strict, most objects should fail
    let strict_config = r#"
manifest_tests:
  - name: "all_objects_max_1_line"
    type: max_code_lines
    severity: "warning"
    max_lines: 1
    applies_to:
      - "models"
      - "snapshots"
      - "macros"
"#;

    let env = TestEnvironment::new(manifest, strict_config);
    let findings = env.run_maniest_rules(false);

    // Should find 3 violations: violating_model, violating_snapshot, and violating_macro
    // (compliant_model has exactly 1 line so it passes)
    assert_eq!(findings.len(), 3);
    for finding in &findings {
        assert_eq!(finding.0.severity, "WARN");
        assert_eq!(finding.0.rule_name, "all_objects_max_1_line");
    }
    let has_compliant = findings
        .iter()
        .any(|f| f.0.message.contains("compliant_model"));
    assert!(!has_compliant); // compliant_model should not be in findings
}
