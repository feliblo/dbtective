use crate::cli::table::RuleResult;
use crate::core::config::severity::Severity;
use serde::Serialize;
use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::time::Duration;

// --- JSON format (nested: metadata + summary + results) ---

#[derive(Serialize)]
pub struct StructuredOutput {
    pub metadata: Metadata,
    pub summary: Summary,
    pub results: Vec<StructuredResult>,
}

#[derive(Serialize)]
pub struct Metadata {
    pub dbtective_version: String,
    pub timestamp: String,
    pub project_name: String,
    pub manifest_path: String,
    pub catalog_path: String,
    pub execution_time_seconds: f64,
}

#[derive(Serialize)]
pub struct Summary {
    pub total: usize,
    pub errors: usize,
    pub warnings: usize,
    pub pass: bool,
}

#[derive(Serialize)]
pub struct StructuredResult {
    pub severity: String,
    pub object_type: String,
    pub rule_name: String,
    pub category: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

// --- Flat format for CSV/NDJSON (metadata + summary on every row) ---
#[derive(Serialize)]
pub struct FlatResult {
    pub dbtective_version: String,
    pub timestamp: String,
    pub project_name: String,
    pub manifest_path: String,
    pub catalog_path: String,
    pub execution_time_seconds: f64,
    pub total: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub pass: bool,
    pub severity: String,
    pub object_type: String,
    pub rule_name: String,
    pub category: String,
    pub message: String,
    pub relative_path: String,
}

impl StructuredOutput {
    /// Build a `StructuredOutput` from rule results.
    ///
    /// Summary always contains true counts. The `results` array respects `hide_warnings`.
    pub fn from_results(
        results: &[(RuleResult, &Severity)],
        started_at: &str,
        duration: Duration,
        hide_warnings: bool,
        project_name: &str,
        manifest_path: &str,
        catalog_path: &str,
    ) -> Self {
        let errors = results
            .iter()
            .filter(|(_, s)| **s == Severity::Error)
            .count();
        let warnings = results
            .iter()
            .filter(|(_, s)| **s == Severity::Warning)
            .count();

        let filtered: Vec<_> = if hide_warnings {
            results
                .iter()
                .filter(|(_, s)| **s == Severity::Error)
                .collect()
        } else {
            results.iter().collect()
        };

        let structured_results: Vec<StructuredResult> = filtered
            .iter()
            .map(|(r, s)| StructuredResult {
                severity: severity_label(s),
                object_type: r.object_type.clone(),
                rule_name: r.rule_name.clone(),
                category: r.category.clone(),
                message: r.message.clone(),
                relative_path: r.relative_path.clone(),
            })
            .collect();

        Self {
            metadata: Metadata {
                dbtective_version: env!("CARGO_PKG_VERSION").to_owned(),
                timestamp: started_at.to_owned(),
                project_name: project_name.to_owned(),
                manifest_path: manifest_path.to_owned(),
                catalog_path: catalog_path.to_owned(),
                execution_time_seconds: duration.as_secs_f64(),
            },
            summary: Summary {
                total: errors + warnings,
                errors,
                warnings,
                pass: errors == 0,
            },
            results: structured_results,
        }
    }

    /// Render as pretty-printed JSON.
    ///
    /// # Panics
    ///
    /// Panics if serialization fails (should not happen with valid data).
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize JSON output")
    }

    /// Convert to flat rows for CSV/NDJSON.
    pub fn to_flat_results(&self) -> Vec<FlatResult> {
        self.results
            .iter()
            .map(|r| FlatResult {
                dbtective_version: self.metadata.dbtective_version.clone(),
                timestamp: self.metadata.timestamp.clone(),
                project_name: self.metadata.project_name.clone(),
                manifest_path: self.metadata.manifest_path.clone(),
                catalog_path: self.metadata.catalog_path.clone(),
                execution_time_seconds: self.metadata.execution_time_seconds,
                total: self.summary.total,
                total_errors: self.summary.errors,
                total_warnings: self.summary.warnings,
                pass: self.summary.pass,
                severity: r.severity.clone(),
                object_type: r.object_type.clone(),
                rule_name: r.rule_name.clone(),
                category: r.category.clone(),
                message: r.message.clone(),
                relative_path: r.relative_path.clone().unwrap_or_default(),
            })
            .collect()
    }

    /// Render as newline-delimited JSON (one flat row per line).
    ///
    /// # Panics
    ///
    /// Panics if serialization fails (should not happen with valid data).
    pub fn to_ndjson(&self) -> String {
        let flat = self.to_flat_results();
        let mut output = String::new();
        for row in &flat {
            let line = serde_json::to_string(row).expect("Failed to serialize NDJSON row");
            let _ = writeln!(output, "{line}");
        }
        output
    }

    pub fn to_csv(&self) -> String {
        let flat = self.to_flat_results();
        let mut output = String::from(
            "dbtective_version,timestamp,project_name,manifest_path,catalog_path,execution_time_seconds,total,total_errors,total_warnings,pass,severity,object_type,rule_name,category,message,relative_path\n",
        );
        for row in &flat {
            let _ = writeln!(
                output,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                csv_escape(&row.dbtective_version),
                csv_escape(&row.timestamp),
                csv_escape(&row.project_name),
                csv_escape(&row.manifest_path),
                csv_escape(&row.catalog_path),
                row.execution_time_seconds,
                row.total,
                row.total_errors,
                row.total_warnings,
                row.pass,
                csv_escape(&row.severity),
                csv_escape(&row.object_type),
                csv_escape(&row.rule_name),
                csv_escape(&row.category),
                csv_escape(&row.message),
                csv_escape(&row.relative_path),
            );
        }
        output
    }
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn severity_label(severity: &Severity) -> String {
    match severity {
        Severity::Error => "error".to_string(),
        Severity::Warning => "warning".to_string(),
    }
}

/// Write content to a file or stdout.
///
/// # Errors
///
/// Returns an error if the output file cannot be created or written to.
pub fn write_output(content: &str, output_file: Option<&str>) -> anyhow::Result<()> {
    if let Some(path) = output_file {
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        print!("{content}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_error(rule_name: &str, message: &str, path: Option<&str>) -> (RuleResult, Severity) {
        (
            RuleResult::new(
                &Severity::Error,
                "Model",
                rule_name,
                message,
                path.map(str::to_owned),
            ),
            Severity::Error,
        )
    }

    fn make_warning(rule_name: &str, message: &str, path: Option<&str>) -> (RuleResult, Severity) {
        (
            RuleResult::new(
                &Severity::Warning,
                "Source",
                rule_name,
                message,
                path.map(str::to_owned),
            ),
            Severity::Warning,
        )
    }

    fn as_refs(results: &[(RuleResult, Severity)]) -> Vec<(RuleResult, &Severity)> {
        results.iter().map(|(r, s)| (r.clone(), s)).collect()
    }

    fn build_output(
        results: &[(RuleResult, &Severity)],
        started_at: &str,
        duration: Duration,
        hide_warnings: bool,
    ) -> StructuredOutput {
        StructuredOutput::from_results(
            results,
            started_at,
            duration,
            hide_warnings,
            "test_project",
            "target/manifest.json",
            "target/catalog.json",
        )
    }

    #[test]
    fn test_empty_results_json() {
        let results: Vec<(RuleResult, Severity)> = vec![];
        let refs = as_refs(&results);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(42),
            false,
        );

        assert_eq!(output.summary.total, 0);
        assert_eq!(output.summary.errors, 0);
        assert_eq!(output.summary.warnings, 0);
        assert!(output.summary.pass);
        assert!(output.results.is_empty());

        let json = output.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("metadata").is_some());
        assert!(parsed.get("summary").is_some());
        assert!(parsed.get("results").is_some());
    }

    #[test]
    fn test_metadata_fields() {
        let owned = vec![make_error("rule_a", "msg", None)];
        let refs = as_refs(&owned);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(42),
            false,
        );

        assert_eq!(output.metadata.project_name, "test_project");
        assert_eq!(output.metadata.manifest_path, "target/manifest.json");
        assert_eq!(output.metadata.timestamp, "2026-01-01T00:00:00Z");
        assert!(!output.metadata.dbtective_version.is_empty());
    }

    #[test]
    fn test_mixed_results_summary() {
        let owned = vec![
            make_error(
                "has_description",
                "Missing description",
                Some("models/orders.sql"),
            ),
            make_warning("naming_convention", "Bad name", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::from_secs(1), false);

        assert_eq!(output.summary.total, 2);
        assert_eq!(output.summary.errors, 1);
        assert_eq!(output.summary.warnings, 1);
        assert!(!output.summary.pass);
        assert_eq!(output.results.len(), 2);
        assert_eq!(output.results[0].severity, "error");
        assert_eq!(output.results[1].severity, "warning");
        assert_eq!(
            output.results[0].relative_path,
            Some("models/orders.sql".to_string())
        );
        assert!(output.results[1].relative_path.is_none());
    }

    #[test]
    fn test_hide_warnings_filters_results() {
        let owned = vec![
            make_error("has_description", "Missing", None),
            make_warning("naming", "Bad name", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, true);

        // Summary reports true counts
        assert_eq!(output.summary.warnings, 1);
        assert_eq!(output.summary.errors, 1);
        // Results array only has errors
        assert_eq!(output.results.len(), 1);
        assert_eq!(output.results[0].severity, "error");
    }

    #[test]
    fn test_json_round_trip() {
        let owned = vec![make_error(
            "has_description",
            "Missing",
            Some("models/test.sql"),
        )];
        let refs = as_refs(&owned);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(100),
            false,
        );
        let json = output.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["metadata"]["project_name"], "test_project");
        assert_eq!(parsed["metadata"]["manifest_path"], "target/manifest.json");
        assert_eq!(parsed["summary"]["errors"], 1);
        assert_eq!(parsed["summary"]["pass"], false);
        assert_eq!(parsed["results"][0]["severity"], "error");
        assert_eq!(parsed["results"][0]["relative_path"], "models/test.sql");
    }

    #[test]
    fn test_json_omits_null_relative_path() {
        let owned = vec![make_error("has_description", "Missing", None)];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let json = output.to_json();

        // The "relative_path" key should not appear when None
        assert!(!json.contains("relative_path"));
    }

    #[test]
    fn test_ndjson_line_count() {
        let owned = vec![
            make_error("rule_a", "msg a", None),
            make_warning("rule_b", "msg b", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let ndjson = output.to_ndjson();
        let lines: Vec<&str> = ndjson.trim().lines().collect();

        assert_eq!(lines.len(), 2);
        // Each line must be valid JSON
        for line in &lines {
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(line);
            assert!(parsed.is_ok(), "Invalid JSON line: {line}");
        }
    }

    #[test]
    fn test_ndjson_contains_metadata_per_row() {
        let owned = vec![
            make_error("rule_a", "msg a", None),
            make_warning("rule_b", "msg b", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(50),
            false,
        );
        let ndjson = output.to_ndjson();

        for line in ndjson.trim().lines() {
            let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
            assert_eq!(parsed["timestamp"], "2026-01-01T00:00:00Z");
            assert_eq!(parsed["project_name"], "test_project");
            assert_eq!(parsed["total"], 2);
            assert_eq!(parsed["total_errors"], 1);
            assert_eq!(parsed["total_warnings"], 1);
            assert_eq!(parsed["pass"], false);
        }
    }

    #[test]
    fn test_csv_header_and_row_count() {
        let owned = vec![
            make_error("rule_a", "msg a", Some("models/a.sql")),
            make_warning("rule_b", "msg b", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let csv = output.to_csv();
        let lines: Vec<&str> = csv.trim().lines().collect();

        assert_eq!(lines.len(), 3); // header + 2 rows
        assert!(lines[0].starts_with("dbtective_version,"));
        assert!(lines[1].contains("error"));
        assert!(lines[2].contains("warning"));
    }

    #[test]
    fn test_csv_escapes_commas_in_message() {
        let owned = vec![make_error(
            "rule_a",
            "Missing description, please add one",
            None,
        )];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let csv = output.to_csv();

        // The message with a comma should be quoted
        assert!(csv.contains("\"Missing description, please add one\""));
    }

    #[test]
    fn test_csv_escapes_quotes_in_message() {
        let owned = vec![make_error(
            "rule_a",
            "Model \"orders\" is missing a description",
            None,
        )];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let csv = output.to_csv();

        // Quotes inside values should be doubled
        assert!(csv.contains("\"Model \"\"orders\"\" is missing a description\""));
    }

    #[test]
    fn test_csv_metadata_on_every_row() {
        let owned = vec![
            make_error("rule_a", "msg a", None),
            make_warning("rule_b", "msg b", None),
        ];
        let refs = as_refs(&owned);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(50),
            false,
        );
        let csv = output.to_csv();

        // Every data row should contain the project name
        for line in csv.trim().lines().skip(1) {
            assert!(line.contains("test_project"));
        }
    }

    #[test]
    fn test_flat_results_relative_path_defaults_to_empty() {
        let owned = vec![make_error("rule_a", "msg", None)];
        let refs = as_refs(&owned);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let flat = output.to_flat_results();

        assert_eq!(flat[0].relative_path, "");
    }

    #[test]
    fn test_empty_results_ndjson() {
        let results: Vec<(RuleResult, Severity)> = vec![];
        let refs = as_refs(&results);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let ndjson = output.to_ndjson();

        assert!(ndjson.trim().is_empty());
    }

    #[test]
    fn test_empty_results_csv_has_header_only() {
        let results: Vec<(RuleResult, Severity)> = vec![];
        let refs = as_refs(&results);
        let output = build_output(&refs, "2026-01-01T00:00:00Z", Duration::ZERO, false);
        let csv = output.to_csv();
        assert_eq!(csv.trim().lines().count(), 1);
    }

    #[test]
    fn test_timestamp_propagated() {
        let owned = vec![make_error("rule_a", "msg", None)];
        let refs = as_refs(&owned);
        let ts = "2026-06-15T09:30:00.123456Z";
        let output = build_output(&refs, ts, Duration::ZERO, false);

        assert_eq!(output.metadata.timestamp, ts);
        let json = output.to_json();
        assert!(json.contains(ts));
    }

    #[test]
    fn test_execution_time_precision() {
        let owned = vec![make_error("rule_a", "msg", None)];
        let refs = as_refs(&owned);
        let output = build_output(
            &refs,
            "2026-01-01T00:00:00Z",
            Duration::from_millis(1234),
            false,
        );

        // 1234ms = 1.234s
        assert!((output.metadata.execution_time_seconds - 1.234).abs() < f64::EPSILON);
    }

    #[test]
    fn test_csv_escape_no_special_chars() {
        assert_eq!(csv_escape("hello"), "hello");
    }

    #[test]
    fn test_csv_escape_with_comma() {
        assert_eq!(csv_escape("a,b"), "\"a,b\"");
    }

    #[test]
    fn test_csv_escape_with_quotes() {
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_csv_escape_with_newline() {
        assert_eq!(csv_escape("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_write_output_to_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output.json");
        let path_str = path.to_str().unwrap();

        write_output("test content", Some(path_str)).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "test content");
    }
}
