use crate::cli::table::CheckRow;
use crate::core::checks::common::has_description;
use crate::core::config::parse_config::SpecificRuleConfig;
use crate::core::config::severity::Severity;
use crate::core::config::Config;
use crate::core::manifest::Manifest;

/// Applies node checks to the manifest.
///
/// # Errors
/// This function may return an error if rule `applies_to` section is missing or if rule application fails.
/// However this would never happen as default `applies_to` are set when parsing the config.
/// And config checks are done prior to this function being called.
///
/// # Panics
/// This function will panic if `applies_to` is `None` for any rule.
#[must_use]
pub fn apply_node_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> Vec<(CheckRow, &'a Severity)> {
    let mut results = Vec::new();

    for node in manifest.nodes.values() {
        for rule in &config.manifest_tests {
            if let Some(applies) = &rule.applies_to {
                if applies.node_objects.contains(&node.ruletarget()) {
                    if verbose {
                        println!(
                            "Applying rule '{}' to node '{}'",
                            rule.get_name(),
                            node.get_name()
                        );
                    }
                    let check_row_result = match &rule.rule {
                        SpecificRuleConfig::HasDescription {} => {
                            has_description::has_description(node, rule)
                        }
                    };

                    if let Ok(check_row) = check_row_result {
                        results.push((check_row, &rule.severity));
                    }
                }
            }
        }
    }

    results
}
