use owo_colors::OwoColorize;
use tabled::Tabled;
use tabled::{
    settings::{location::Locator, Color, Style},
    Table,
};

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

pub fn show_results(
    results: &[(RuleResult, &Severity)],
    verbose: bool,
    entry_point: &str,
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
        let clickable_rows: Vec<RuleResult> = results
            .iter()
            .map(|(row, _)| {
                let mut new_row = row.clone();
                if let Some(ref path) = row.relative_path {
                    let entry = entry_point.trim_end_matches('/');
                    let path = path.trim_start_matches('/');
                    let full_path = format!("{entry}/{path}");

                    let abs_path = Path::new(&full_path)
                        .canonicalize()
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or(full_path);
                    let file_url = format!("file://{abs_path}");

                    new_row.message = format!(
                        "\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",
                        url = file_url,
                        text = row.message
                    );
                }
                new_row
            })
            .collect();

        let mut table = Table::new(&clickable_rows);
        table
            .with(Style::modern())
            .modify(Locator::content("FAIL"), Color::BG_RED)
            .modify(Locator::content("WARN"), Color::BG_YELLOW);
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
