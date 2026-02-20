mod common;

use common::TestEnvironment;
use dbtective::cli::structured_output::write_output;

const MANIFEST: &str = r#"{
  "metadata": {
    "dbt_schema_version": "https://schemas.getdbt.com/dbt/manifest/v12.json",
    "dbt_version": "1.10.0",
    "generated_at": "2025-01-01T00:00:00.000000Z",
    "invocation_id": "test-invocation",
    "env": {},
    "project_name": "test_project",
    "adapter_type": "postgres",
    "quoting": { "database": true, "schema": true, "identifier": true, "column": null }
  },
  "nodes": {
    "model.test_project.orders": {
      "database": "analytics",
      "schema": "public",
      "name": "orders",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "orders.sql",
      "original_file_path": "models/orders.sql",
      "unique_id": "model.test_project.orders",
      "fqn": ["test_project", "orders"],
      "alias": "orders",
      "checksum": {"name": "sha256", "checksum": "abc123"},
      "config": { "enabled": true, "materialized": "view", "tags": [] },
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
      "relation_name": "analytics.public.orders",
      "raw_code": "select * from raw_orders",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
    },
    "model.test_project.customers": {
      "database": "analytics",
      "schema": "public",
      "name": "customers",
      "resource_type": "model",
      "package_name": "test_project",
      "path": "customers.sql",
      "original_file_path": "models/customers.sql",
      "unique_id": "model.test_project.customers",
      "fqn": ["test_project", "customers"],
      "alias": "customers",
      "checksum": {"name": "sha256", "checksum": "def456"},
      "config": { "enabled": true, "materialized": "table", "tags": [] },
      "tags": [],
      "description": "Customer dimension table",
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
      "relation_name": "analytics.public.customers",
      "raw_code": "select * from raw_customers",
      "language": "sql",
      "refs": [],
      "sources": [],
      "metrics": [],
      "depends_on": {"macros": [], "nodes": []},
      "compiled_code": null,
      "extra_ctes_injected": false,
      "extra_ctes": [],
      "contract": {"enforced": false, "checksum": null}
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

const CONFIG_WITH_FAILURES: &str = r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
  - name: "naming_convention"
    type: "name_convention"
    severity: "warning"
    pattern: "pascal_case"
"#;

const CONFIG_WITH_CUSTOM_CATEGORY: &str = r#"
manifest_tests:
  - name: "has_description_default"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    includes:
      - "orders"
  - name: "has_description_custom"
    type: "has_description"
    severity: "warning"
    category: "custom_docs"
    applies_to:
      - "models"
    includes:
      - "orders"
"#;

const CONFIG_ALL_PASS: &str = r#"
manifest_tests:
  - name: "has_description"
    type: "has_description"
    severity: "error"
    applies_to:
      - "models"
    includes:
      - "customers"
"#;

// ── JSON output tests ──

#[test]
fn test_json_output_is_valid_json() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.get("metadata").is_some());
    assert!(parsed.get("summary").is_some());
    assert!(parsed.get("results").is_some());
}

#[test]
fn test_json_output_metadata_fields() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let metadata = &parsed["metadata"];

    assert!(metadata["dbtective_version"].as_str().is_some());
    assert!(metadata["timestamp"].as_str().is_some());
    assert_eq!(metadata["project_name"], "test_project");
    assert_eq!(metadata["manifest_path"], "target/manifest.json");
    assert!(metadata["execution_time_seconds"].as_f64().is_some());
}

#[test]
fn test_json_output_summary_counts() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let summary = &parsed["summary"];

    // orders has no description (error) + both models fail PascalCase (warnings)
    assert_eq!(summary["errors"], 1);
    assert_eq!(summary["warnings"], 2);
    assert_eq!(summary["total"], 3);
    assert_eq!(summary["pass"], false);
}

#[test]
fn test_json_output_results_contain_expected_fields() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let results = parsed["results"].as_array().unwrap();

    for result in results {
        // Every result must have these fields
        assert!(result.get("severity").is_some());
        assert!(result.get("object_type").is_some());
        assert!(result.get("rule_name").is_some());
        assert!(result.get("message").is_some());

        // severity must be "error" or "warning"
        let sev = result["severity"].as_str().unwrap();
        assert!(
            sev == "error" || sev == "warning",
            "unexpected severity: {sev}"
        );
    }
}

#[test]
fn test_json_output_pass_when_no_failures() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_ALL_PASS);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["summary"]["total"], 0);
    assert_eq!(parsed["summary"]["pass"], true);
    assert!(parsed["results"].as_array().unwrap().is_empty());
}

#[test]
fn test_json_output_relative_path_present() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let results = parsed["results"].as_array().unwrap();

    // At least one result should have a relative_path
    let has_path = results.iter().any(|r| r.get("relative_path").is_some());
    assert!(has_path, "Expected at least one result with relative_path");
}

// ── CSV output tests ──

#[test]
fn test_csv_output_has_header() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    let header = csv.lines().next().unwrap();
    assert!(header.contains("dbtective_version"));
    assert!(header.contains("timestamp"));
    assert!(header.contains("project_name"));
    assert!(header.contains("manifest_path"));
    assert!(header.contains("execution_time_seconds"));
    assert!(header.contains("severity"));
    assert!(header.contains("object_type"));
    assert!(header.contains("rule_name"));
    assert!(header.contains("message"));
    assert!(header.contains("relative_path"));
    assert!(header.contains("total_errors"));
    assert!(header.contains("total_warnings"));
    assert!(header.contains("pass"));
}

#[test]
fn test_csv_output_row_count_matches_results() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let result_count = output.results.len();
    let csv = output.to_csv();

    let line_count = csv.trim().lines().count();
    // header + one row per result
    assert_eq!(line_count, 1 + result_count);
}

#[test]
fn test_csv_output_metadata_on_every_row() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    for line in csv.trim().lines().skip(1) {
        // Every data row should contain the project name
        assert!(
            line.contains("test_project"),
            "Row missing project_name: {line}"
        );
    }
}

#[test]
fn test_csv_output_empty_when_all_pass() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_ALL_PASS);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    // Header only, no data rows
    assert_eq!(csv.trim().lines().count(), 1);
}

#[test]
fn test_csv_output_contains_error_and_warning_rows() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    let data_rows: Vec<&str> = csv.trim().lines().skip(1).collect();
    let has_error = data_rows.iter().any(|r| r.contains(",error,"));
    let has_warning = data_rows.iter().any(|r| r.contains(",warning,"));

    assert!(has_error, "Expected at least one error row in CSV");
    assert!(has_warning, "Expected at least one warning row in CSV");
}

// ── NDJSON output tests ──

#[test]
fn test_ndjson_output_each_line_is_valid_json() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    for (i, line) in ndjson.trim().lines().enumerate() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
        assert!(parsed.is_ok(), "Line {i} is not valid JSON: {line}");
    }
}

#[test]
fn test_ndjson_output_line_count_matches_results() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let result_count = output.results.len();
    let ndjson = output.to_ndjson();

    let line_count = ndjson.trim().lines().count();
    assert_eq!(line_count, result_count);
}

#[test]
fn test_ndjson_output_metadata_on_every_line() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    for line in ndjson.trim().lines() {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();

        // Every row must have metadata fields
        assert!(
            parsed.get("dbtective_version").is_some(),
            "Missing dbtective_version"
        );
        assert!(parsed.get("timestamp").is_some(), "Missing timestamp");
        assert!(parsed.get("project_name").is_some(), "Missing project_name");
        assert!(
            parsed.get("manifest_path").is_some(),
            "Missing manifest_path"
        );
        assert!(
            parsed.get("execution_time_seconds").is_some(),
            "Missing execution_time_seconds"
        );

        // Summary fields
        assert!(parsed.get("total").is_some(), "Missing total");
        assert!(parsed.get("total_errors").is_some(), "Missing total_errors");
        assert!(
            parsed.get("total_warnings").is_some(),
            "Missing total_warnings"
        );
        assert!(parsed.get("pass").is_some(), "Missing pass");

        // Result fields
        assert!(parsed.get("severity").is_some(), "Missing severity");
        assert!(parsed.get("object_type").is_some(), "Missing object_type");
        assert!(parsed.get("rule_name").is_some(), "Missing rule_name");
        assert!(parsed.get("message").is_some(), "Missing message");
    }
}

#[test]
fn test_ndjson_output_empty_when_all_pass() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_ALL_PASS);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    assert!(ndjson.trim().is_empty());
}

#[test]
fn test_ndjson_output_summary_values_consistent() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    let lines: Vec<serde_json::Value> = ndjson
        .trim()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    // All rows must have the same metadata and summary values
    let first = &lines[0];
    for line in &lines[1..] {
        assert_eq!(line["total"], first["total"]);
        assert_eq!(line["total_errors"], first["total_errors"]);
        assert_eq!(line["total_warnings"], first["total_warnings"]);
        assert_eq!(line["pass"], first["pass"]);
        assert_eq!(line["timestamp"], first["timestamp"]);
        assert_eq!(line["project_name"], first["project_name"]);
    }
}

// ── Cross-format consistency tests ──

#[test]
fn test_all_formats_agree_on_result_count() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);

    let json_count = output.results.len();
    let csv_count = output.to_csv().trim().lines().count() - 1; // minus header
    let ndjson_count = output.to_ndjson().trim().lines().count();

    assert_eq!(json_count, csv_count);
    assert_eq!(json_count, ndjson_count);
}

#[test]
fn test_all_formats_agree_on_summary() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);

    // JSON summary
    let json: serde_json::Value = serde_json::from_str(&output.to_json()).unwrap();
    let json_errors = json["summary"]["errors"].as_u64().unwrap();
    let json_warnings = json["summary"]["warnings"].as_u64().unwrap();

    // NDJSON first line
    let first_ndjson_line: serde_json::Value =
        serde_json::from_str(output.to_ndjson().lines().next().unwrap()).unwrap();
    let ndjson_errors = first_ndjson_line["total_errors"].as_u64().unwrap();
    let ndjson_warnings = first_ndjson_line["total_warnings"].as_u64().unwrap();

    assert_eq!(json_errors, ndjson_errors);
    assert_eq!(json_warnings, ndjson_warnings);
}

// ── write_output tests ──

#[test]
fn test_write_output_to_file_json() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let file_path = env.temp_dir.path().join("results.json");
    write_output(&json, Some(file_path.to_str().unwrap())).unwrap();

    let contents = std::fs::read_to_string(&file_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert!(parsed.get("metadata").is_some());
    assert!(parsed.get("summary").is_some());
    assert!(parsed.get("results").is_some());
}

#[test]
fn test_write_output_to_file_csv() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    let file_path = env.temp_dir.path().join("results.csv");
    write_output(&csv, Some(file_path.to_str().unwrap())).unwrap();

    let contents = std::fs::read_to_string(&file_path).unwrap();
    assert!(contents.starts_with("dbtective_version,"));
    assert!(contents.lines().count() > 1);
}

#[test]
fn test_write_output_to_file_ndjson() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    let file_path = env.temp_dir.path().join("results.ndjson");
    write_output(&ndjson, Some(file_path.to_str().unwrap())).unwrap();

    let contents = std::fs::read_to_string(&file_path).unwrap();
    for line in contents.trim().lines() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
        assert!(parsed.is_ok(), "Written NDJSON line is not valid JSON");
    }
}

// ── Category tests ──

#[test]
fn test_json_output_results_contain_category() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let results = parsed["results"].as_array().unwrap();

    for result in results {
        assert!(
            result.get("category").is_some(),
            "Result missing category field"
        );
        let category = result["category"].as_str().unwrap();
        assert!(!category.is_empty(), "Category should not be empty");
    }
}

#[test]
fn test_json_output_default_categories() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let results = parsed["results"].as_array().unwrap();

    for result in results {
        let rule_name = result["rule_name"].as_str().unwrap();
        let category = result["category"].as_str().unwrap();

        match rule_name {
            "has_description" | "has_description_default" => {
                assert_eq!(category, "documentation");
            }
            "naming_convention" => {
                assert_eq!(category, "naming");
            }
            _ => {} // Other rules have their own categories
        }
    }
}

#[test]
fn test_csv_output_contains_category_header() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let csv = output.to_csv();

    let header = csv.lines().next().unwrap();
    assert!(
        header.contains("category"),
        "CSV header missing category column"
    );
}

#[test]
fn test_ndjson_output_contains_category() {
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_FAILURES);
    let output = env.run_structured_output(false);
    let ndjson = output.to_ndjson();

    for line in ndjson.trim().lines() {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(
            parsed.get("category").is_some(),
            "NDJSON line missing category field"
        );
    }
}

#[test]
fn test_same_rule_type_different_categories() {
    // Two has_description rules: one with default category, one with custom category
    let env = TestEnvironment::new(MANIFEST, CONFIG_WITH_CUSTOM_CATEGORY);
    let output = env.run_structured_output(false);
    let json = output.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let results = parsed["results"].as_array().unwrap();

    assert!(
        results.len() >= 2,
        "Expected at least 2 results, got {}",
        results.len()
    );

    let categories: Vec<&str> = results
        .iter()
        .map(|r| r["category"].as_str().unwrap())
        .collect();

    assert!(
        categories.contains(&"documentation"),
        "Expected default category 'documentation', got: {categories:?}",
    );
    assert!(
        categories.contains(&"custom_docs"),
        "Expected custom category 'custom_docs', got: {categories:?}",
    );
}
