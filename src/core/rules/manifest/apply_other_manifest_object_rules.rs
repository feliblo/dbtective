use crate::core::config::applies_to::RuleTargetable;
use crate::core::rules::rule_config::{
    check_allowed_subfolders, check_name_convention, code_contains_refs, code_forbidden_patterns,
    code_max_joins, code_max_lines, code_no_hardcoded_refs, exposure_parents_materialized,
    has_description, has_freshness, has_loader, has_metadata_keys, has_refs, has_required_tests,
    has_tags, has_unique_test, is_not_orphaned, max_downstream_dependencies,
    max_upstream_dependencies, property_file_colocation,
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
        apply_function_rules(manifest, config, verbose)?,
    ]
    .into_iter()
    .flatten()
    .collect())
}

/// Applies source rules to the manifest.
///
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
#[allow(clippy::too_many_lines)]
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
                    ManifestSpecificRuleConfig::HasDescription {
                        ref min_length,
                        ref forbidden_substrings,
                    } => {
                        has_description(source, rule, *min_length, forbidden_substrings.as_deref())
                    }
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
                    ManifestSpecificRuleConfig::HasRequiredTests { required_tests } => {
                        for mut rule_row in
                            has_required_tests(source, rule, manifest, required_tests)
                        {
                            rule_row.category = rule.get_category().to_string();
                            acc.push((rule_row, &rule.severity));
                        }
                        None
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(source, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::AllowedSubfolders {
                        allowed_subfolders,
                        path_prefix,
                        path_postfix,
                    } => check_allowed_subfolders(
                        source,
                        rule,
                        allowed_subfolders,
                        path_prefix.as_ref(),
                        path_postfix.as_ref(),
                    ),

                    ManifestSpecificRuleConfig::SourcesHaveLoader {} => has_loader(source, rule),
                    ManifestSpecificRuleConfig::SourcesHaveFreshness {} => {
                        has_freshness(source, rule)
                    }
                    ManifestSpecificRuleConfig::MaxUpstreamDependencies {
                        max_upstream,
                        exclude_types,
                    } => max_upstream_dependencies(
                        source,
                        rule,
                        *max_upstream,
                        exclude_types,
                        manifest,
                    ),
                    ManifestSpecificRuleConfig::MaxDownstreamDependencies {
                        max_downstream,
                        exclude_types,
                    } => max_downstream_dependencies(
                        source,
                        rule,
                        *max_downstream,
                        exclude_types,
                        manifest,
                    ),

                    ManifestSpecificRuleConfig::PropertyFileColocation {
                        mode,
                        allowed_subdirectories,
                    } => property_file_colocation(
                        source,
                        rule,
                        mode,
                        allowed_subdirectories.as_ref(),
                    ),

                    // These can't be implemented for sources
                    ManifestSpecificRuleConfig::HasRefs {}
                    | ManifestSpecificRuleConfig::CodeMaxLines { .. }
                    | ManifestSpecificRuleConfig::CodeForbiddenPatterns { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                    | ManifestSpecificRuleConfig::CodeContainsRefs {}
                    | ManifestSpecificRuleConfig::CodeMaxJoins { .. }
                    | ManifestSpecificRuleConfig::CodeNoHardcodedRefs {}
                    | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                    | ManifestSpecificRuleConfig::ExposureParentsMaterialized { .. } => {
                        return Ok(acc)
                    }
                };

                if let Some(mut rule_row) = rule_row_result {
                    rule_row.category = rule.get_category().to_string();
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
#[allow(clippy::too_many_lines)]
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
                        ManifestSpecificRuleConfig::HasDescription {
                            ref min_length,
                            ref forbidden_substrings,
                        } => has_description(
                            macro_obj,
                            rule,
                            *min_length,
                            forbidden_substrings.as_deref(),
                        ),
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
                        ManifestSpecificRuleConfig::CodeMaxLines { max_lines } => {
                            code_max_lines(macro_obj, rule, *max_lines)
                        }
                        ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                            forbidden_patterns,
                            case_sensitive,
                        } => code_forbidden_patterns(
                            macro_obj,
                            rule,
                            forbidden_patterns,
                            *case_sensitive,
                        ),
                        ManifestSpecificRuleConfig::CodeContainsRefs {} => {
                            code_contains_refs(macro_obj, rule)
                        }
                        ManifestSpecificRuleConfig::CodeMaxJoins {
                            max_joins: code_max_joins_allowed,
                        } => code_max_joins(macro_obj, rule, *code_max_joins_allowed),
                        ManifestSpecificRuleConfig::CodeNoHardcodedRefs {} => {
                            code_no_hardcoded_refs(macro_obj, rule)
                        }
                        ManifestSpecificRuleConfig::AllowedSubfolders {
                            allowed_subfolders,
                            path_prefix,
                            path_postfix,
                        } => check_allowed_subfolders(
                            macro_obj,
                            rule,
                            allowed_subfolders,
                            path_prefix.as_ref(),
                            path_postfix.as_ref(),
                        ),
                        ManifestSpecificRuleConfig::PropertyFileColocation {
                            mode,
                            allowed_subdirectories,
                        } => property_file_colocation(
                            macro_obj,
                            rule,
                            mode,
                            allowed_subdirectories.as_ref(),
                        ),
                        // These can't be implemented for macros
                        ManifestSpecificRuleConfig::HasTags { .. }
                        | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                        | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                        | ManifestSpecificRuleConfig::HasRequiredTests { .. }
                        | ManifestSpecificRuleConfig::HasRefs {}
                        | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                        | ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. }
                        | ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. }
                        | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                        | ManifestSpecificRuleConfig::ExposureParentsMaterialized { .. }
                        | ManifestSpecificRuleConfig::SourcesHaveLoader {}
                        | ManifestSpecificRuleConfig::SourcesHaveFreshness {} => return Ok(acc),
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
#[allow(clippy::too_many_lines)]
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
                        ManifestSpecificRuleConfig::HasDescription {
                            ref min_length,
                            ref forbidden_substrings,
                        } => has_description(
                            exposure,
                            rule,
                            *min_length,
                            forbidden_substrings.as_deref(),
                        ),
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
                        ManifestSpecificRuleConfig::AllowedSubfolders {
                            allowed_subfolders,
                            path_prefix,
                            path_postfix,
                        } => check_allowed_subfolders(
                            exposure,
                            rule,
                            allowed_subfolders,
                            path_prefix.as_ref(),
                            path_postfix.as_ref(),
                        ),
                        ManifestSpecificRuleConfig::PropertyFileColocation {
                            mode,
                            allowed_subdirectories,
                        } => property_file_colocation(
                            exposure,
                            rule,
                            mode,
                            allowed_subdirectories.as_ref(),
                        ),
                        ManifestSpecificRuleConfig::ExposureParentsMaterialized {
                            ref allowed_materializations,
                        } => {
                            for mut rule_row in exposure_parents_materialized(
                                exposure,
                                rule,
                                allowed_materializations,
                                manifest,
                            ) {
                                rule_row.category = rule.get_category().to_string();
                                acc.push((rule_row, &rule.severity));
                            }
                            None
                        }
                        // These can't be implemented for exposures
                        ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                        | ManifestSpecificRuleConfig::CodeMaxLines { .. }
                        | ManifestSpecificRuleConfig::CodeForbiddenPatterns { .. }
                        | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                        | ManifestSpecificRuleConfig::HasRequiredTests { .. }
                        | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                        | ManifestSpecificRuleConfig::CodeContainsRefs {}
                        | ManifestSpecificRuleConfig::CodeMaxJoins { .. }
                        | ManifestSpecificRuleConfig::CodeNoHardcodedRefs {}
                        | ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. }
                        | ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. }
                        | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                        | ManifestSpecificRuleConfig::SourcesHaveLoader {}
                        | ManifestSpecificRuleConfig::SourcesHaveFreshness {} => return Ok(acc),
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
                    ManifestSpecificRuleConfig::HasDescription {
                        ref min_length,
                        ref forbidden_substrings,
                    } => has_description(sm, rule, *min_length, forbidden_substrings.as_deref()),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(sm, rule, convention)
                    }
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(sm, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::HasRefs {} => has_refs(sm, rule),
                    ManifestSpecificRuleConfig::AllowedSubfolders {
                        allowed_subfolders,
                        path_prefix,
                        path_postfix,
                    } => check_allowed_subfolders(
                        sm,
                        rule,
                        allowed_subfolders,
                        path_prefix.as_ref(),
                        path_postfix.as_ref(),
                    ),
                    // These can't be implemented for semantic models
                    ManifestSpecificRuleConfig::HasTags { .. }
                    | ManifestSpecificRuleConfig::CodeMaxLines { .. }
                    | ManifestSpecificRuleConfig::CodeForbiddenPatterns { .. }
                    | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                    | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                    | ManifestSpecificRuleConfig::HasRequiredTests { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                    | ManifestSpecificRuleConfig::CodeContainsRefs {}
                    | ManifestSpecificRuleConfig::CodeMaxJoins { .. }
                    | ManifestSpecificRuleConfig::CodeNoHardcodedRefs {}
                    | ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                    | ManifestSpecificRuleConfig::ExposureParentsMaterialized { .. }
                    | ManifestSpecificRuleConfig::SourcesHaveLoader {}
                    | ManifestSpecificRuleConfig::SourcesHaveFreshness {}
                    | ManifestSpecificRuleConfig::PropertyFileColocation { .. } => return Ok(acc),
                };

                if let Some(mut rule_row) = rule_row_result {
                    rule_row.category = rule.get_category().to_string();
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
                    ManifestSpecificRuleConfig::HasDescription {
                        ref min_length,
                        ref forbidden_substrings,
                    } => has_description(ut, rule, *min_length, forbidden_substrings.as_deref()),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(ut, rule, convention)
                    }
                    ManifestSpecificRuleConfig::AllowedSubfolders {
                        allowed_subfolders,
                        path_prefix,
                        path_postfix,
                    } => check_allowed_subfolders(
                        ut,
                        rule,
                        allowed_subfolders,
                        path_prefix.as_ref(),
                        path_postfix.as_ref(),
                    ),

                    // Unit Tests do not implement the following rules
                    ManifestSpecificRuleConfig::CodeMaxLines { .. }
                    | ManifestSpecificRuleConfig::CodeForbiddenPatterns { .. }
                    | ManifestSpecificRuleConfig::HasTags { .. }
                    | ManifestSpecificRuleConfig::HasRefs {}
                    | ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                    | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                    | ManifestSpecificRuleConfig::HasRequiredTests { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                    | ManifestSpecificRuleConfig::HasMetadataKeys { .. }
                    | ManifestSpecificRuleConfig::CodeContainsRefs {}
                    | ManifestSpecificRuleConfig::CodeMaxJoins { .. }
                    | ManifestSpecificRuleConfig::CodeNoHardcodedRefs {}
                    | ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                    | ManifestSpecificRuleConfig::ExposureParentsMaterialized { .. }
                    | ManifestSpecificRuleConfig::SourcesHaveLoader {}
                    | ManifestSpecificRuleConfig::SourcesHaveFreshness {}
                    | ManifestSpecificRuleConfig::PropertyFileColocation { .. } => return Ok(acc),
                };

                if let Some(mut rule_row) = rule_row_result {
                    rule_row.category = rule.get_category().to_string();
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}

/// Applies function (UDF) rules to the manifest.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern).
fn apply_function_rules<'a>(
    manifest: &'a Manifest,
    config: &'a Config,
    _verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let results = if let Some(manifest_tests) = &config.manifest_tests {
        manifest
            .functions
            .values()
            .flat_map(|func| manifest_tests.iter().map(move |rule| (func, rule)))
            .try_fold(Vec::new(), |mut acc, (func, rule)| -> anyhow::Result<_> {
                if !should_run_test(func, rule.includes.as_ref(), rule.excludes.as_ref()) {
                    return Ok(acc);
                }

                if let Some(applies) = &rule.applies_to {
                    if !applies.function_objects.contains(&func.ruletarget()) {
                        return Ok(acc);
                    }
                }

                let rule_row_result = match &rule.rule {
                    ManifestSpecificRuleConfig::HasDescription {
                        ref min_length,
                        ref forbidden_substrings,
                    } => has_description(func, rule, *min_length, forbidden_substrings.as_deref()),
                    ManifestSpecificRuleConfig::NameConvention { convention } => {
                        check_name_convention(func, rule, convention)
                    }
                    ManifestSpecificRuleConfig::HasTags {
                        required_tags,
                        criteria,
                    } => has_tags(func, rule, required_tags, criteria),
                    ManifestSpecificRuleConfig::HasMetadataKeys {
                        required_keys,
                        custom_message,
                    } => has_metadata_keys(func, rule, required_keys, custom_message.as_ref()),
                    ManifestSpecificRuleConfig::HasRefs {} => has_refs(func, rule),
                    ManifestSpecificRuleConfig::CodeMaxLines { max_lines } => {
                        code_max_lines(func, rule, *max_lines)
                    }
                    ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                        forbidden_patterns,
                        case_sensitive,
                    } => code_forbidden_patterns(func, rule, forbidden_patterns, *case_sensitive),
                    ManifestSpecificRuleConfig::CodeContainsRefs {} => {
                        code_contains_refs(func, rule)
                    }
                    ManifestSpecificRuleConfig::CodeMaxJoins {
                        max_joins: code_max_joins_allowed,
                    } => code_max_joins(func, rule, *code_max_joins_allowed),
                    ManifestSpecificRuleConfig::CodeNoHardcodedRefs {} => {
                        code_no_hardcoded_refs(func, rule)
                    }
                    ManifestSpecificRuleConfig::AllowedSubfolders {
                        allowed_subfolders,
                        path_prefix,
                        path_postfix,
                    } => check_allowed_subfolders(
                        func,
                        rule,
                        allowed_subfolders,
                        path_prefix.as_ref(),
                        path_postfix.as_ref(),
                    ),
                    // These can't be implemented for functions
                    ManifestSpecificRuleConfig::IsNotOrphaned { .. }
                    | ManifestSpecificRuleConfig::HasUniqueTest { .. }
                    | ManifestSpecificRuleConfig::HasRequiredTests { .. }
                    | ManifestSpecificRuleConfig::HasContractEnforced { .. }
                    | ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. }
                    | ManifestSpecificRuleConfig::MaxMaterializationLineage { .. }
                    | ManifestSpecificRuleConfig::ExposureParentsMaterialized { .. }
                    | ManifestSpecificRuleConfig::SourcesHaveLoader {}
                    | ManifestSpecificRuleConfig::SourcesHaveFreshness {}
                    | ManifestSpecificRuleConfig::PropertyFileColocation { .. } => return Ok(acc),
                };

                if let Some(mut rule_row) = rule_row_result {
                    rule_row.category = rule.get_category().to_string();
                    acc.push((rule_row, &rule.severity));
                }

                Ok(acc)
            })?
    } else {
        Vec::new()
    };

    Ok(results)
}
