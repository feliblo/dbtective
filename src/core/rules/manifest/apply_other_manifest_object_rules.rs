use crate::core::rules::rule_config::{
    check_name_convention, has_description, has_metadata_keys, has_refs, has_tags, has_unique_test,
    is_not_orphaned, max_code_lines,
};
use crate::{
    cli::table::RuleResult,
    core::{
        config::{
            includes_excludes::should_run_test, manifest_rule::ManifestSpecificRuleConfig,
            severity::Severity, Config,
        },
        manifest::Manifest,
    },
};

// I don't like the duplication of code in this. But otherwise complex trait functions would be needed.
// It can be refactored later if needed, but honestly I need a bit more Rust experience to do that well.
// If you read this and have ideas, please open an issue or PR, I'm curious to see better implementations!

/// Applies macro rules to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
/// //
pub fn apply_manifest_object_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    Ok([
        apply_source_rules(manifest, config, verbose)?,
        apply_macro_rules(manifest, config, verbose)?,
        apply_exposure_rules(manifest, config, verbose)?,
        apply_semantic_model_rules(manifest, config, verbose)?,
        apply_unit_test_rules(manifest, config, verbose)?,
    ]
    .into_iter()
    .flatten()
    .collect())
}

/// Applies source rules to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_source_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .sources
            .values()
            .flat_map(|source| manifest_tests.iter().map(move |rule| (source, rule)))
            .try_fold(Vec::new(), |mut acc, (source, rule)| -> anyhow::Result<_> {
                if !should_run_test(source, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.source_objects.contains(&source.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let rule_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(source, rule),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(source, rule, convention)
                    }
                    ManifestSpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(source, rule, required_tags, criteria),
                    ManifestSpecificRuleConfig::IsNotOrphaned { allowed_references } => {
                        is_not_orphaned(source, rule, allowed_references, manifest)
                    }
                    ManifestSpecificRuleConfig::HasUniqueTest { allowed_test_names } => {
                        has_unique_test(source, rule, manifest, allowed_test_names)
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(source, rule, required_keys, custom_message.as_ref()),

                    // These can't be implemented for exposures
                    ManifestSpecificRuleConfig::HasRefs {}
                    | ManifestSpecificRuleConfig::MaxCodeLines { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc), // Models only
                };

                if let Some(rule_row) = rule_row_result {
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies unit test rules to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_macro_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .macros
            .values()
            .flat_map(|macro_obj| manifest_tests.iter().map(move |rule| (macro_obj, rule)))
            .try_fold(
                Vec::new(),
                |mut acc, (macro_obj, rule)| -> anyhow::Result<_> {
                    if !should_run_test(macro_obj, rule.includes.as_ref(), rule.excludes.as_ref()) {
                        return Ok(acc);
                    }

                    if let Some(applies) = &rule.applies_to {
                        if !applies.macro_objects.contains(&macro_obj.ruletarget()) {
                            return Ok(acc);
                        }
                    }

                    let rule_row_result = match &rule.rule {
                        ManifestSpecificRuleConfig::HasDescription {} => {
                            has_description(macro_obj, rule)
                        }
                        ManifestSpecificRuleConfig::NameConvention { convention } => {
                            check_name_convention(macro_obj, rule, convention)
                        }
                        ManifestSpecificRuleConfig::HasMetadataKeys {
                            required_keys,
                            custom_message,
                        } => has_metadata_keys(
                            macro_obj,
                            rule,
                            required_keys,
                            custom_message.as_ref(),
                        ),
                        ManifestSpecificRuleConfig::MaxCodeLines { max_lines } => {
                            max_code_lines(macro_obj, rule, *max_lines)
                        }
                        // These can't be implemented for macros
                        ManifestSpecificRuleConfig::HasTags { .. }
                        | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                        | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                        | ManifestSpecificRuleConfig::HasRefs {}
                        | ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc), //
                    };

                    if let Some(rule_row) = rule_row_result {
                        acc.push((rule_row, &rule.severity));
                    }

                    Ok(acc)
                },
            )?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies exposure rules to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_exposure_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .exposures
            .values()
            .flat_map(|exposure| manifest_tests.iter().map(move |rule| (exposure, rule)))
            .try_fold(
                Vec::new(),
                |mut acc, (exposure, rule)| -> anyhow::Result<_> {
                    if !should_run_test(exposure, rule.includes.as_ref(), rule.excludes.as_ref()) {
                        return Ok(acc);
                    }

                    if let Some(applies) = &rule.applies_to {
                        if !applies.exposure_objects.contains(&exposure.ruletarget()) {
                            return Ok(acc);
                        }
                    }

                    let rule_row_result = match &rule.rule {
                        ManifestSpecificRuleConfig::HasDescription {} => {
                            has_description(exposure, rule)
                        }
                        ManifestSpecificRuleConfig::NameConvention { convention } => {
                            check_name_convention(exposure, rule, convention)
                        }
                        ManifestSpecificRuleConfig::HasTags {
                            required_tags,
                            criteria,
                        } => has_tags(exposure, rule, required_tags, criteria),
                        ManifestSpecificRuleConfig::HasMetadataKeys {
                            required_keys,
                            custom_message,
                        } => has_metadata_keys(
                            exposure,
                            rule,
                            required_keys,
                            custom_message.as_ref(),
                        ),
                        ManifestSpecificRuleConfig::HasRefs {} => has_refs(exposure, rule),
                        // These can't be implemented for exposures
                        ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                        | ManifestSpecificRuleConfig::MaxCodeLines { .. }
                        | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                        | ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc),
                    };

                    if let Some(rule_row) = rule_row_result {
                        acc.push((rule_row, &rule.severity));
                    }

                    Ok(acc)
                },
            )?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies semantic model rules to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_semantic_model_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .semantic_models
            .values()
            .flat_map(|sm| manifest_tests.iter().map(move |rule| (sm, rule)))
            .try_fold(Vec::new(), |mut acc, (sm, rule)| -> anyhow::Result<_> {
                if !should_run_test(sm, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.semantic_model_objects.contains(&sm.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let rule_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(sm, rule),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(sm, rule, convention)
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(sm, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::HasRefs {} => has_refs(sm, rule),
                    // These can't be implemented for semantic models
                    ManifestSpecificRuleConfig::HasTags { .. }
                    | ManifestSpecificRuleConfig::MaxCodeLines { .. }
                    | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                    | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc),
                };

                if let Some(rule_row) = rule_row_result {
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies unit test rules to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_unit_test_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .unit_tests
            .values()
            .flat_map(|ut| manifest_tests.iter().map(move |rule| (ut, rule)))
            .try_fold(Vec::new(), |mut acc, (ut, rule)| -> anyhow::Result<_> {
                if !should_run_test(ut, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.unit_test_objects.contains(&ut.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let rule_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(ut, rule),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(ut, rule, convention)
                    }

                    // Unit Tests do not implement the following rules
                    ManifestSpecificRuleConfig::MaxCodeLines { .. }
                    | ManifestSpecificRuleConfig::HasTags { .. }
                    | ManifestSpecificRuleConfig::HasRefs {}
                    | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                    | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced {}
                    | ManifestSpecificRuleConfig::HasMetadataKeys { .. } => return Ok(acc),
                };

                if let Some(rule_row) = rule_row_result {
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}
