use owo_colors::OwoColorize;
use tabled::settings::object::Columns;
use tabled::Tabled;
use tabled::{
    settings::{location::Locator, Color, Style, Width},
    Table,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

use crate::core::config::severity::Severity;
use std::path::Path;

#[derive(Tabled, PartialEq, Eq, Debug, Clone)]
pub struct RuleResult {
    #[tabled(rename = "Severity")]
    pub severity: String,
    #[tabled(rename = "Object")]
    pub object_type: String,
    #[tabled(rename = "Name")]
    pub rule_name: String,
    #[tabled(rename = "Finding")]
    pub message: String,
    #[tabled(skip)]
    pub relative_path: Option<String>,
}

impl RuleResult {
    pub fn new(
        severity: &Severity,
        object_type: impl Into<String>,
        rule_name: impl Into<String>,
        message: impl Into<String>,
        relative_path: Option<String>,
    ) -> Self {
        let sev_str = severity.as_str().to_string();
        Self {
            severity: sev_str,
            object_type: object_type.into(),
            rule_name: rule_name.into(),
            message: message.into(),
            relative_path,
        }
    }
}

pub fn show_results_and_exit(
    results: &[(RuleResult, &Severity)],
    verbose: bool,
    entry_point: &str,
    disable_hyperlinks: bool,
    hide_warnings: bool,
    duration: Option<std::time::Duration>,
) -> i32 {
    let error_count = results
        .iter()
        .filter(|(_, sev)| **sev == Severity::Error)
        .count();
    let warning_count = results
        .iter()
        .filter(|(_, sev)| **sev == Severity::Warning)
        .count();

    let filtered_results: Vec<_> = if hide_warnings {
        results
            .iter()
            .filter(|(_, sev)| **sev == Severity::Error)
            .collect()
    } else {
        results.iter().collect()
    };

    if filtered_results.is_empty() {
        println!(
            "{} 🕵️",
            "All rules passed successfully! - dbtective off the case.".green(),
        );
        // Show summary if there were hidden warnings
        if warning_count > 0 && hide_warnings {
            print_summary(error_count, warning_count);
        }
    } else {
        println!("\n {}", "🕵️  dbtective detected some issues:".red());

        let sorted_results = sort_results(&filtered_results);

        let table_rows: Vec<RuleResult> = sorted_results
            .iter()
            .map(|(row, _)| {
                let mut new_row = (*row).clone();
                if disable_hyperlinks {
                    return new_row;
                }
                // Add file hyperlinks to message if relative_path is present
                if let Some(ref path) = row.relative_path {
                    let entry = entry_point.trim_end_matches('/');
                    let path = path.trim_start_matches('/');
                    let full_path = format!("{entry}/{path}");

                    let abs_path = Path::new(&full_path)
                        .canonicalize()
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or(full_path);

                    // Convert to proper file URL format
                    // On Windows, paths like C:\foo need to become file:///C:/foo to follow RFC 8089
                    // canonicalize() on Windows may return extended-length paths with \\?\ prefix
                    let file_url = if cfg!(windows) {
                        let path_with_slashes = abs_path.replace('\\', "/");
                        let clean_path = path_with_slashes
                            .strip_prefix("//?/")
                            .or_else(|| path_with_slashes.strip_prefix("//./"))
                            .unwrap_or(&path_with_slashes);
                        format!("file:///{clean_path}")
                    } else {
                        format!("file://{abs_path}")
                    };

                    new_row.message = format!(
                        "\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",
                        url = file_url,
                        text = row.message
                    );
                }
                new_row
            })
            .collect();

        let (width, _) = get_terminal_size();
        let mut table = Table::new(&table_rows);
        let message_column_width = if width > 60 { width - 60 } else { width / 2 };
        table
            .with(Style::modern())
            .modify(Locator::content("FAIL"), Color::BG_RED)
            .modify(Locator::content("WARN"), Color::BG_YELLOW)
            .modify(Columns::last(), Width::wrap(message_column_width));

        println!("{table}");
        print_summary(error_count, warning_count);
    }

    if verbose {
        if let Some(duration) = duration {
            println!("Analysis completed in: {duration:?}");
        }
    }

    i32::from(error_count > 0)
}

fn get_terminal_size() -> (usize, usize) {
    if let Some((TerminalWidth(width), TerminalHeight(height))) = terminal_size() {
        (width as usize, height as usize)
    } else {
        // Default size for non-interactive environments (like CI/GitHub Actions)
        (140, 40)
    }
}

fn print_summary(error_count: usize, warning_count: usize) {
    let error_str = if error_count == 1 { "error" } else { "errors" };
    let warning_str = if warning_count == 1 {
        "warning"
    } else {
        "warnings"
    };

    let error_part = if error_count > 0 {
        format!("{error_count} {error_str}").red().to_string()
    } else {
        format!("{error_count} {error_str}")
    };

    let warning_part = if warning_count > 0 {
        format!("{warning_count} {warning_str}")
            .yellow()
            .to_string()
    } else {
        format!("{warning_count} {warning_str}")
    };

    println!("\ndbtective found: {error_part}, {warning_part}");
}

/// Sort results by severity (FAIL before WARN), then by `object_type`, then by `rule_name`
fn sort_results<'a>(
    results: &'a [&'a (RuleResult, &'a Severity)],
) -> Vec<&'a (RuleResult, &'a Severity)> {
    let mut sorted: Vec<_> = results.to_vec();
    sorted.sort_by(|(a, sev_a), (b, sev_b)| {
        sev_a
            .as_code()
            .cmp(&sev_b.as_code())
            .reverse()
            .then_with(|| a.object_type.cmp(&b.object_type))
            .then_with(|| a.rule_name.cmp(&b.rule_name))
    });
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_error_result(object_type: &str, rule_name: &str, message: &str) -> RuleResult {
        RuleResult::new(&Severity::Error, object_type, rule_name, message, None)
    }

    fn make_warning_result(object_type: &str, rule_name: &str, message: &str) -> RuleResult {
        RuleResult::new(&Severity::Warning, object_type, rule_name, message, None)
    }

    #[test]
    fn test_rule_result_new_error() {
        let result = RuleResult::new(
            &Severity::Error,
            "Model",
            "has_description",
            "Missing description",
            Some("models/test.sql".to_string()),
        );

        assert_eq!(result.severity, "FAIL");
        assert_eq!(result.object_type, "Model");
        assert_eq!(result.rule_name, "has_description");
        assert_eq!(result.message, "Missing description");
        assert_eq!(result.relative_path, Some("models/test.sql".to_string()));
    }

    #[test]
    fn test_rule_result_new_warning() {
        let result = RuleResult::new(
            &Severity::Warning,
            "Source",
            "naming_convention",
            "Name doesn't match convention",
            None,
        );

        assert_eq!(result.severity, "WARN");
        assert_eq!(result.object_type, "Source");
        assert_eq!(result.rule_name, "naming_convention");
        assert_eq!(result.message, "Name doesn't match convention");
        assert_eq!(result.relative_path, None);
    }

    #[test]
    fn test_sort_results_errors_before_warnings() {
        let warning = make_warning_result("Model", "rule_a", "warning message");
        let error = make_error_result("Model", "rule_b", "error message");

        let results: Vec<(RuleResult, &Severity)> =
            vec![(warning, &Severity::Warning), (error, &Severity::Error)];

        let refs: Vec<_> = results.iter().collect();
        let sorted = sort_results(&refs);

        assert_eq!(sorted[0].0.severity, "FAIL");
        assert_eq!(sorted[1].0.severity, "WARN");
    }

    #[test]
    fn test_sort_results_by_object_type() {
        let error_source = make_error_result("Source", "rule_a", "message");
        let error_model = make_error_result("Model", "rule_a", "message");

        let results: Vec<(RuleResult, &Severity)> = vec![
            (error_source, &Severity::Error),
            (error_model, &Severity::Error),
        ];

        let refs: Vec<_> = results.iter().collect();
        let sorted = sort_results(&refs);

        assert_eq!(sorted[0].0.object_type, "Model");
        assert_eq!(sorted[1].0.object_type, "Source");
    }

    #[test]
    fn test_sort_results_by_rule_name() {
        let error_b = make_error_result("Model", "rule_b", "message");
        let error_a = make_error_result("Model", "rule_a", "message");

        let results: Vec<(RuleResult, &Severity)> =
            vec![(error_b, &Severity::Error), (error_a, &Severity::Error)];

        let refs: Vec<_> = results.iter().collect();
        let sorted = sort_results(&refs);

        assert_eq!(sorted[0].0.rule_name, "rule_a");
        assert_eq!(sorted[1].0.rule_name, "rule_b");
    }

    #[test]
    fn test_show_results_exit_code_zero_when_empty() {
        let results: Vec<(RuleResult, &Severity)> = vec![];
        let exit_code = show_results_and_exit(&results, false, ".", true, false, None);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_show_results_exit_code_zero_for_warnings_only() {
        let warning = make_warning_result("Model", "rule_a", "warning message");
        let results: Vec<(RuleResult, &Severity)> = vec![(warning, &Severity::Warning)];

        let exit_code = show_results_and_exit(&results, false, ".", true, false, None);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_show_results_exit_code_one_for_errors() {
        let error = make_error_result("Model", "rule_a", "error message");
        let results: Vec<(RuleResult, &Severity)> = vec![(error, &Severity::Error)];

        let exit_code = show_results_and_exit(&results, false, ".", true, false, None);
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn test_show_results_exit_code_one_for_mixed_results() {
        let error = make_error_result("Model", "rule_a", "error message");
        let warning = make_warning_result("Model", "rule_b", "warning message");
        let results: Vec<(RuleResult, &Severity)> =
            vec![(error, &Severity::Error), (warning, &Severity::Warning)];

        let exit_code = show_results_and_exit(&results, false, ".", true, false, None);
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn test_show_results_hide_warnings_still_returns_zero_for_warnings_only() {
        let warning = make_warning_result("Model", "rule_a", "warning message");
        let results: Vec<(RuleResult, &Severity)> = vec![(warning, &Severity::Warning)];

        // hide_warnings = true, but exit code should still be 0 (no errors)
        let exit_code = show_results_and_exit(&results, false, ".", true, true, None);
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn test_show_results_hide_warnings_with_errors() {
        let error = make_error_result("Model", "rule_a", "error message");
        let warning = make_warning_result("Model", "rule_b", "warning message");
        let results: Vec<(RuleResult, &Severity)> =
            vec![(error, &Severity::Error), (warning, &Severity::Warning)];

        // hide_warnings = true, exit code should be 1 (has errors)
        let exit_code = show_results_and_exit(&results, false, ".", true, true, None);
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn test_get_terminal_size_returns_default_in_test() {
        let (width, height) = get_terminal_size();
        // In test environment, terminal_size() usually returns None
        // so we should get the default values
        assert!(width > 0);
        assert!(height > 0);
    }
}
