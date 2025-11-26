use crate::core::{checks::common::has_description::CheckRow, config::Severity};
use tabled::{
    settings::{location::Locator, Color, Style},
    Table,
};

pub fn print_node_checks_table(results: &[(CheckRow, &Severity)]) {
    let mut table = Table::new(results.iter().map(|(row, _)| row));
    table
        .with(Style::rounded())
        .modify(Locator::content("FAIL"), Color::BG_RED)
        .modify(Locator::content("WARN"), Color::BG_YELLOW);

    println!("{table}");
}
