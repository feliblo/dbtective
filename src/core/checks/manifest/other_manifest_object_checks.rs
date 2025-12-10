use crate::core::checks::rules::{
    check_name_convention, has_description, has_tags, has_unique_test, is_not_orphaned,
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
use owo_colors::OwoColorize;

// I don't like the duplication of code in this. But otherwise complex trait functions would be needed.
// It can be refactored later if needed, but honestly I need a bit more Rust experience to do that well.
// If you read this and have ideas, please open an issue or PR, I'm curious to see better implementations!

/// Applies macro checks to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
/// //
pub fn apply_manifest_object_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    Ok([
        apply_source_checks(manifest, config, verbose)?,
        apply_macro_checks(manifest, config, verbose)?,
        apply_exposure_checks(manifest, config, verbose)?,
        apply_semantic_model_checks(manifest, config, verbose)?,
        apply_unit_test_checks(manifest, config, verbose)?,
    ]
    .into_iter()
    .flatten()
    .collect())
}

/// Applies source checks to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_source_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .sources
            .values()
            .flat_map(|source| manifest_tests.iter().map(move |rule| (source, rule)))
            .try_fold(Vec::new(), |mut acc, (source, rule)| -> anyhow::Result<_> {
                if verbose {
                    println!(
                        "{}",
                        format!(
                            "Applying rule '{}' to source '{}'",
                            rule.get_name(),
                            source.get_name()
                        )
                        .blue()
                    );
                }

                if !should_run_test(source, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.source_objects.contains(&source.ruletarget()) {
                        if verbose {
                            println!(
                                "{}",
                                format!(
                                    "Skipping rule '{}' for source '{}' due to applies_to filter",
                                    rule.get_name(),
                                    source.get_name()
                                )
                                .blue()
                            );
                        }
                        return Ok(acc);
                    }
                }

                let check_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(source, rule),
                    ManifestSpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(source, rule, pattern)?
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
                    // These can't be implemented for exposures
                    ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc), // Models only
                };

                if let Some(check_row) = check_row_result {
                    acc.push((check_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies unit test checks to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_macro_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .macros
            .values()
            .flat_map(|macro_obj| manifest_tests.iter().map(move |rule| (macro_obj, rule)))
            .try_fold(
                Vec::new(),
                |mut acc, (macro_obj, rule)| -> anyhow::Result<_> {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Applying rule '{}' to macro '{}'",
                                rule.get_name(),
                                macro_obj.get_name()
                            )
                            .blue()
                        );
                    }

                    if !should_run_test(macro_obj, rule.includes.as_ref(), rule.excludes.as_ref()) {
                        if verbose {
                            println!(
                                "{}",
                                format!(
                                "Skipping rule '{}' for macro '{}' due to include/exclude filters",
                                rule.get_name(),
                                macro_obj.get_name()
                            )
                                .blue()
                            );
                        }
                        return Ok(acc);
                    }

                    if let Some(applies) = &rule.applies_to {
                        if !applies.macro_objects.contains(&macro_obj.ruletarget()) {
                            return Ok(acc);
                        }
                    }

                    let check_row_result = match &rule.rule {
                        ManifestSpecificRuleConfig::HasDescription {} => {
                            has_description(macro_obj, rule)
                        }
                        ManifestSpecificRuleConfig::NameConvention { pattern } => {
                            check_name_convention(macro_obj, rule, pattern)?
                        }
                        // These can't be implemented for macros
                        ManifestSpecificRuleConfig::HasTags { .. }
                        | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                        | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                        | ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc), //
                    };

                    if let Some(check_row) = check_row_result {
                        acc.push((check_row, &rule.severity));
                    }

                    Ok(acc)
                },
            )?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies exposure checks to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_exposure_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .exposures
            .values()
            .flat_map(|exposure| manifest_tests.iter().map(move |rule| (exposure, rule)))
            .try_fold(Vec::new(), |mut acc, (exposure, rule)| -> anyhow::Result<_> {
                if verbose {
                    println!(
                        "{}",
                        format!(
                            "Applying rule '{}' to exposure '{}'",
                            rule.get_name(),
                            exposure.get_name()
                        )
                        .blue()
                    );
                }

                if !should_run_test(exposure, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Skipping rule '{}' for exposure '{}' due to include/exclude filters",
                                rule.get_name(),
                                exposure.get_name()
                            )
                            .blue()
                        );
                    }
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.exposure_objects.contains(&exposure.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let check_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => {
                        has_description(exposure, rule)
                    }
                    ManifestSpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(exposure, rule, pattern)?
                    }
                    ManifestSpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(exposure, rule, required_tags, criteria),
                    // These can't be implemented for exposures
                    ManifestSpecificRuleConfig::IsNotOrphaned { .. } |
                    ManifestSpecificRuleConfig::HasUniqueTest { .. } |
                    ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc),
                };

                if let Some(check_row) = check_row_result {
                    acc.push((check_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies semantic model checks to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_semantic_model_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .semantic_models
            .values()
            .flat_map(|sm| manifest_tests.iter().map(move |rule| (sm, rule)))
            .try_fold(Vec::new(), |mut acc, (sm, rule)| -> anyhow::Result<_> {
                if verbose {
                    println!(
                        "{}",
                        format!(
                            "Applying rule '{}' to semantic model '{}'",
                            rule.get_name(),
                            sm.get_name()
                        )
                        .blue()
                    );
                }

                if !should_run_test(sm, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Skipping rule '{}' for semantic model '{}' due to include/exclude filters",
                                rule.get_name(),
                                sm.get_name()
                            )
                            .blue()
                        );
                    }
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.semantic_model_objects.contains(&sm.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let check_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(sm, rule),
                    ManifestSpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(sm, rule, pattern)?
                    }
                    // These can't be implemented for semantic models
                    ManifestSpecificRuleConfig::HasTags { .. }  |
                    ManifestSpecificRuleConfig::IsNotOrphaned { .. } |
                    ManifestSpecificRuleConfig::HasUniqueTest { .. } |
                    ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc),
                };

                if let Some(check_row) = check_row_result {
                    acc.push((check_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies unit test checks to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_unit_test_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .unit_tests
            .values()
            .flat_map(|ut| manifest_tests.iter().map(move |rule| (ut, rule)))
            .try_fold(Vec::new(), |mut acc, (ut, rule)| -> anyhow::Result<_> {
                if verbose {
                    println!(
                        "{}",
                        format!(
                            "Applying rule '{}' to unit test '{}'",
                            rule.get_name(),
                            ut.get_name()
                        )
                        .blue()
                    );
                }

                if !should_run_test(ut, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    if verbose {
                        println!(
                            "{}",
                            format!(
                                "Skipping rule '{}' for unit test '{}' due to include/exclude filters",
                                rule.get_name(),
                                ut.get_name()
                            )
                            .blue()
                        );
                    }
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.unit_test_objects.contains(&ut.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let check_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {} => has_description(ut, rule),
                    ManifestSpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(ut, rule, pattern)?
                    }
                    // Unit Tests do not have tags & Don't implement TagAble
                    ManifestSpecificRuleConfig::HasTags { .. } |
                    ManifestSpecificRuleConfig::IsNotOrphaned { .. } |
                    ManifestSpecificRuleConfig::HasUniqueTest { .. } |
                    ManifestSpecificRuleConfig::HasContractEnforced {} => return Ok(acc),
                };

                if let Some(check_row) = check_row_result {
                    acc.push((check_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}
