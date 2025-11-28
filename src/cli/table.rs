use owo_colors::OwoColorize;
use tabled::Tabled;
use tabled::{
    settings::{location::Locator, Color, Style},
    Table,
};

use crate::core::config::severity::Severity;

#[derive(Tabled, PartialEq, Eq, Debug)]
pub struct CheckRow {
    #[tabled(rename = "Severity")]
    pub severity: String,
    #[tabled(rename = "Object")]
    pub object_type: String,
    #[tabled(rename = "Name")]
    pub rule_name: String,
    #[tabled(rename = "Finding")]
    pub message: String,
}

impl CheckRow {
    pub fn new(
        severity: &Severity,
        object_type: impl Into<String>,
        rule_name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let sev_str = severity.as_str().to_string();
        Self {
            severity: sev_str,
            object_type: object_type.into(),
            rule_name: rule_name.into(),
            message: message.into(),
        }
    }
}

pub fn show_results(
    results: &[(CheckRow, &Severity)],
    verbose: bool,
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
        let mut table = Table::new(results.iter().map(|(row, _)| row));
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
