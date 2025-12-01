use crate::cli::table::RuleResult;
use crate::core::checks::common::{check_name_convention, has_description, has_tags};
use crate::core::config::parse_config::SpecificRuleConfig;

use crate::core::config::severity::Severity;
use crate::core::config::{includes_excludes::should_run_test, Config};
use crate::core::manifest::Manifest;
use owo_colors::OwoColorize;

/// Applies node checks to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
pub fn apply_node_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let mut results = Vec::new();
    for node in manifest.nodes.values() {
        for rule in &config.manifest_tests {
            if let Some(applies) = &rule.applies_to {
                if !should_run_test(&node, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Skipping rule '{}' for node '{}' due to include/exclude filters",
                                rule.get_name(),
                                node.get_name()
                            )
                            .blue()
                        );
                    }
                    continue;
                }

                // applies_to: object based filtering
                if applies.node_objects.contains(&node.ruletarget()) {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Applying rule '{}' to node '{}'",
                                rule.get_name(),
                                node.get_name()
                            )
                            .blue()
                        );
                    }
                    let check_row_result = match &rule.rule {
                        SpecificRuleConfig::HasDescription {} => has_description(node, rule),
                        SpecificRuleConfig::NameConvention { pattern } => {
                            check_name_convention(node, rule, pattern)?
                        }
                        SpecificRuleConfig::HasTags {
                            required_tags,
                            criteria,
                        } => has_tags(node, rule, required_tags, criteria),
                    };

                    if let Some(check_row) = check_row_result {
                        results.push((check_row, &rule.severity));
                    }
                }
            }
        }
    }

    Ok(results)
}
