use crate::core::checks::common::{check_name_convention, has_description, has_tags};
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

/// Applies macro checks to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
pub fn apply_manifest_object_checks<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let mut results = Vec::new();
    let mut source_results = apply_source_checks(manifest, config, verbose)?;
    results.append(&mut source_results);

    let mut macro_results = apply_macro_checks(manifest, config, verbose)?;
    results.append(&mut macro_results);

    let mut exposure_results = apply_exposure_checks(manifest, config, verbose)?;
    results.append(&mut exposure_results);

    let mut semantic_model_results = apply_semantic_model_checks(manifest, config, verbose)?;
    results.append(&mut semantic_model_results);
    let mut unit_test_results = apply_unit_test_checks(manifest, config, verbose)?;
    results.append(&mut unit_test_results);
    Ok(results)
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
    let mut results = Vec::new();
    if let Some(manifest_tests) = &config.manifest_tests {
        for source in manifest.sources.values() {
            for rule in manifest_tests {
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
                    SpecificRuleConfig::HasDescription {} => has_description(source, rule),
                    SpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(source, rule, pattern)?
                    }
                    SpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(source, rule, required_tags, criteria),
                };

                if let Some(check_row) = check_row_result {
                    results.push((check_row, &rule.severity));
                }
            }
        }
    }

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
    let mut results = Vec::new();
    if let Some(manifest_tests) = &config.manifest_tests {
        for macro_obj in manifest.macros.values() {
            for rule in manifest_tests {
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
                if !should_run_test(&macro_obj, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    continue;
                }

                // applies_to: object based filtering
                if let Some(applies) = &rule.applies_to {
                    if !applies.macro_objects.contains(&macro_obj.ruletarget()) {
                        continue;
                    }
                }

                let check_row_result = match &rule.rule {
                    SpecificRuleConfig::HasDescription {} => has_description(macro_obj, rule),
                    SpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(macro_obj, rule, pattern)?
                    }
                    // Macros do not have tags & Don't implement TagAble
                    SpecificRuleConfig::HasTags { .. } => continue,
                };

                if let Some(check_row) = check_row_result {
                    results.push((check_row, &rule.severity));
                }
            }
        }
    }
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
    let mut results = Vec::new();
    if let Some(manifest_tests) = &config.manifest_tests {
        for exposure in manifest.exposures.values() {
            for rule in manifest_tests {
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
                if !should_run_test(&exposure, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    continue;
                }

                // applies_to: object based filtering
                if let Some(applies) = &rule.applies_to {
                    if !applies.exposure_objects.contains(&exposure.ruletarget()) {
                        continue;
                    }
                }

                let check_row_result = match &rule.rule {
                    SpecificRuleConfig::HasDescription {} => has_description(exposure, rule),
                    SpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(exposure, rule, pattern)?
                    }
                    SpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(exposure, rule, required_tags, criteria),
                };

                if let Some(check_row) = check_row_result {
                    results.push((check_row, &rule.severity));
                }
            }
        }
    }

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
    let mut results = Vec::new();
    if let Some(manifest_tests) = &config.manifest_tests {
        for sm in manifest.semantic_models.values() {
            for rule in manifest_tests {
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
                if !should_run_test(&sm, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    continue;
                }

                // applies_to: object based filtering
                if let Some(applies) = &rule.applies_to {
                    if !applies.semantic_model_objects.contains(&sm.ruletarget()) {
                        continue;
                    }
                }

                let check_row_result = match &rule.rule {
                    SpecificRuleConfig::HasDescription {} => has_description(sm, rule),
                    SpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(sm, rule, pattern)?
                    }
                    // Semantic Models do not have tags & Don't implement TagAble
                    SpecificRuleConfig::HasTags { .. } => continue,
                };

                if let Some(check_row) = check_row_result {
                    results.push((check_row, &rule.severity));
                }
            }
        }
    }
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
    let mut results = Vec::new();
    for ut in manifest.unit_tests.values() {
        if let Some(manifest_tests) = &config.manifest_tests {
            for rule in manifest_tests {
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
                if !should_run_test(&ut, rule.includes.as_ref(), rule.excludes.as_ref()) {
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
                    continue;
                }

                // applies_to: object based filtering
                if let Some(applies) = &rule.applies_to {
                    if !applies.unit_test_objects.contains(&ut.ruletarget()) {
                        continue;
                    }
                }

                let check_row_result = match &rule.rule {
                    SpecificRuleConfig::HasDescription {} => has_description(ut, rule),
                    SpecificRuleConfig::NameConvention { pattern } => {
                        check_name_convention(ut, rule, pattern)?
                    }
                    // Unit Tests do not have tags & Don't implement TagAble
                    SpecificRuleConfig::HasTags { .. } => continue,
                };

                if let Some(check_row) = check_row_result {
                    results.push((check_row, &rule.severity));
                }
            }
        }
    }

    Ok(results)
}
