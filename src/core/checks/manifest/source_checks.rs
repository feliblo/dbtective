use crate::core::checks::common::{has_description, name_convention};
use crate::{
    cli::table::RuleResult,
    core::{
        config::{
            includes_excludes::should_run_test, parse_config::SpecificRuleConfig,
            severity::Severity, Config,
        },
        manifest::Manifest,
    },
};
use owo_colors::OwoColorize;

pub fn apply_source_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> Vec<(RuleResult, &'a Severity)> {
    let mut results = Vec::new();
    for source in manifest.sources.values() {
        for rule in &config.manifest_tests {
            if !should_run_test(&source, rule.includes.as_ref(), rule.excludes.as_ref()) {
                if verbose {
                    println!(
                        "{}",
                        format!(
                            "Skipping rule '{}' for source '{}' due to include/exclude filters",
                            rule.get_name(),
                            source.get_name()
                        )
                        .blue()
                    );
                }
                continue;
            }

            // applies_to: object based filtering
            if let Some(applies) = &rule.applies_to {
                if !applies.source_objects.contains(&source.ruletarget()) {
                    continue;
                }
            }

            let check_row_result = match &rule.rule {
                SpecificRuleConfig::HasDescription {} => {
                    has_description::has_description(source, rule)
                }
                SpecificRuleConfig::NameConvention { pattern } => {
                    name_convention::check_name_convention(source, rule, pattern)
                }
            };

            if let Ok(check_row) = check_row_result {
                results.push((check_row, &rule.severity));
            }
        }
    }

    results
}
