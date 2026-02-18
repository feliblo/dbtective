use crate::cli::table::RuleResult;
use crate::core::config::applies_to::RuleTargetable;
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;
use crate::core::rules::rule_config::{
    check_allowed_subfolders, check_name_convention, child_map::is_not_orphaned,
    code_contains_refs, has_contract_enforced, has_description, has_forbidden_code,
    has_metadata_keys, has_refs, has_tags, has_unique_test, max_code_lines,
    max_downstream_dependencies, max_joins, max_upstream_dependencies,
};

use crate::core::config::severity::Severity;
use crate::core::config::{includes_excludes::should_run_test, Config};
use crate::core::manifest::Manifest;

/// Applies node rules to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
#[allow(clippy::too_many_lines)]
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
                    ManifestSpecificRuleConfig::HasDescription {
                        ref min_length,
                        ref forbidden_substrings,
                    } => has_description(node, rule, *min_length, forbidden_substrings.as_deref()),
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
                    ManifestSpecificRuleConfig::HasContractEnforced { ref access_level } => {
                        has_contract_enforced(node, rule, access_level.as_ref())
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(node, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::MaxCodeLines { max_lines } => {
                        max_code_lines(node, rule, *max_lines)
                    }
                    ManifestSpecificRuleConfig::HasForbiddenCode {
                        forbidden_patterns,
                        case_sensitive,
                    } => has_forbidden_code(node, rule, forbidden_patterns, *case_sensitive),
                    ManifestSpecificRuleConfig::HasRefs {} => has_refs(node, rule),
                    ManifestSpecificRuleConfig::CodeContainsRefs {} => {
                        code_contains_refs(node, rule)
                    }
                    ManifestSpecificRuleConfig::MaxJoins {
                        max_joins: max_joins_allowed,
                    } => max_joins(node, rule, *max_joins_allowed),
                    ManifestSpecificRuleConfig::AllowedSubfolders {
                        allowed_subfolders,
                        path_prefix,
                        path_postfix,
                    } => check_allowed_subfolders(
                        node,
                        rule,
                        allowed_subfolders,
                        path_prefix.as_ref(),
                        path_postfix.as_ref(),
                    ),
                    ManifestSpecificRuleConfig::MaxUpstreamDependencies {
                        max_upstream,
                        exclude_types,
                    } => max_upstream_dependencies(
                        node,
                        rule,
                        *max_upstream,
                        exclude_types,
                        manifest,
                    ),
                    ManifestSpecificRuleConfig::MaxDownstreamDependencies {
                        max_downstream,
                        exclude_types,
                    } => max_downstream_dependencies(
                        node,
                        rule,
                        *max_downstream,
                        exclude_types,
                        manifest,
                    ),
                    // These only apply to sources
                    ManifestSpecificRuleConfig::SourcesHaveLoader {}
                    | ManifestSpecificRuleConfig::SourcesHaveFreshness {} => return Ok(acc),
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
