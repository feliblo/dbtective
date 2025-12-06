use crate::cli::table::RuleResult;
use crate::core::checks::common::{
    check_name_convention, child_map::is_not_orphaned, has_description, has_tags,
};
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;

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
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .nodes
            .values()
            .flat_map(|node| manifest_tests.iter().map(move |rule| (node, rule)))
            .try_fold(Vec::new(), |mut acc, (node, rule)| -> anyhow::Result<_> {
                // `applies_to` filtering has to be done from the manifest node side (only it contains the path)
                let Some(applies) = rule.applies_to.as_ref() else {
                    return Ok(acc);
                };
                if !should_run_test(node, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    return Ok(acc);
                }

                if !applies.node_objects.contains(&node.ruletarget()) {
                    return Ok(acc);
                }

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
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(node, rule),
                    ManifestSpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(node, rule, pattern)?
                    }
                    ManifestSpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(node, rule, required_tags, criteria),
                    ManifestSpecificRuleConfig::IsNotOrphaned { allowed_references } => {
                        is_not_orphaned(node, rule, allowed_references, manifest)
                    }
                };

                if let Some(check_row) = check_row_result {
                    acc.push((check_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        // No manifest tests defined in the configuration => no results
        Vec::new()
    };

    Ok(results)
}
