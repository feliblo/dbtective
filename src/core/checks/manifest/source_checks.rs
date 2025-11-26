use crate::core::checks::common::has_description;
use crate::{
    cli::table::CheckRow,
    core::{
        config::{parse_config::SpecificRuleConfig, severity::Severity, Config},
        manifest::Manifest,
    },
};

pub fn apply_source_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> Vec<(CheckRow, &'a Severity)> {
    let mut results = Vec::new();

    for source in manifest.sources.values() {
        for rule in &config.manifest_tests {
            if verbose {
                println!(
                    "Applying rule '{}' to source '{}'",
                    rule.get_name(),
                    source.get_name()
                );
            }
            let check_row_result = match &rule.rule {
                SpecificRuleConfig::HasDescription {} => {
                    has_description::has_description(source, rule)
                }
            };

            if let Ok(check_row) = check_row_result {
                results.push((check_row, &rule.severity));
            }
        }
    }

    results
}
