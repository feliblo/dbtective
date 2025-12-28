use crate::cli::table::RuleResult;
use crate::core::config::applies_to::RuleTargetable;
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;
use crate::core::rules::rule_config::{
    check_name_convention, child_map::is_not_orphaned, has_contract_enforced, has_description,
    has_metadata_keys, has_refs, has_tags, has_unique_test, max_code_lines,
};

use crate::core::config::severity::Severity;
use crate::core::config::{includes_excludes::should_run_test, Config};
use crate::core::manifest::Manifest;

/// Applies node rules to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
pub fn apply_manifest_node_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
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
                    return Ok(acc);
                }

                if !applies.node_objects.contains(&node.ruletarget()) {
                    return Ok(acc);
                }

                if let Some(allowed_materializations) = &rule.model_materializations {
                    if let Some(node_materialization) = node.get_materialization() {
                        if !allowed_materializations.contains(node_materialization) {
                            return Ok(acc);
                        }
                    }
                }

                let rule_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(node, rule),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(node, rule, convention)
                    }
                    ManifestSpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(node, rule, required_tags, criteria),
                    ManifestSpecificRuleConfig::IsNotOrphaned { allowed_references } => {
                        is_not_orphaned(node, rule, allowed_references, manifest)
                    }
                    ManifestSpecificRuleConfig::HasUniqueTest { allowed_test_names } => {
                        has_unique_test(node, rule, manifest, allowed_test_names)
                    }
                    ManifestSpecificRuleConfig::HasContractEnforced {} => {
                        has_contract_enforced(node, rule)
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(node, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::MaxCodeLines { max_lines } => {
                        max_code_lines(node, rule, *max_lines)
                    }
                    ManifestSpecificRuleConfig::HasRefs {} => has_refs(node, rule),
                };

                if let Some(rule_row) = rule_row_result {
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        // No manifest tests defined in the configuration => no results
        Vec::new()
    };

    Ok(results)
}
