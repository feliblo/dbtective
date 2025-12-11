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
    duration: Option<std::time::Duration>,
) -> i32 {
    let mut exit_code = 0;
    if results.is_empty() {
        println!(
            "{} 🕵️",
            "All checks passed successfully! - dbtective off the case.".green(),
        );
    } else {
        println!("\n {}", "🕵️  dbtective detected some issues:".red());

        let sorted_results = sort_results(results);

        let table_rows: Vec<RuleResult> = sorted_results
            .iter()
            .map(|(row, _)| {
                let mut new_row = row.clone();
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
        exit_code = 1;
    }

    if verbose {
        if let Some(duration) = duration {
            println!("Analysis completed in: {duration:?}");
        }
    }

    exit_code
}

fn get_terminal_size() -> (usize, usize) {
    if let Some((TerminalWidth(width), TerminalHeight(height))) = terminal_size() {
        (width as usize, height as usize)
    } else {
        // Default size for non-interactive environments (like CI/GitHub Actions)
        (140, 40)
    }
}

/// Sort results by severity (FAIL before WARN), then by `object_type`, then by `rule_name`
fn sort_results<'a>(
    results: &'a [(RuleResult, &'a Severity)],
) -> Vec<&'a (RuleResult, &'a Severity)> {
    let mut sorted: Vec<_> = results.iter().collect();
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
