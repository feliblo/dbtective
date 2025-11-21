use owo_colors::OwoColorize;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::core::checks::RuleResult;

#[derive(Tabled)]
pub struct RuleResultDisplay {
    pub status: &'static str,
    pub node_type: String,
    pub message: String,
}

pub fn print_result_table(check_results: Vec<RuleResult>) {
    let display_rows: Vec<RuleResultDisplay> = check_results
        .into_iter()
        .filter(|rule_result| rule_result.code != 0)
        .map(|rule_result| RuleResultDisplay {
            status: rule_result.severity.as_str(),
            node_type: rule_result.node_type,
            message: rule_result.message,
        })
        .collect();
    if display_rows.is_empty() {
        println!(
            "{}",
            "🕵️  All checks passed successfully! — dbtective is off the case!".green()
        );
    } else {
        let mut table = Table::new(display_rows);
        table.with(Style::modern());
        println!("{table}");
    }
}
