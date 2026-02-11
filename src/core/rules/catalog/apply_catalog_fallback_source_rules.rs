use crate::{
    cli::table::RuleResult,
    core::{
        config::{
            applies_to::RuleTargetable, catalog_rule::CatalogSpecificRuleConfig,
            severity::Severity, Config,
        },
        manifest::Manifest,
        rules::catalog::{
            column_name_convention, columns_canonical_name, columns_have_description,
        },
    },
};

/// Applies catalog rules in manifest-fallback mode for source objects.
/// Instead of iterating catalog sources, this iterates manifest sources and
/// passes them to the existing generic rule functions.
///
/// In fallback: we fake that the manifest source is both "catalog" and "manifest" object
///
/// Only rules where `supports_manifest_fallback()` is true are applied.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern)
pub fn apply_catalog_fallback_source_rules<'a>(
    config: &'a Config,
    manifest: &'a Manifest,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let Some(catalog_tests) = &config.catalog_tests else {
        return Ok(Vec::new());
    };

    manifest
        .sources
        .values()
        .flat_map(|source| catalog_tests.iter().map(move |rule| (source, rule)))
        .try_fold(Vec::new(), |mut acc, (source, rule)| -> anyhow::Result<_> {
            if !rule.rule.supports_manifest_fallback() {
                return Ok(acc);
            }

            if let Some(applies) = &rule.applies_to {
                if !applies.source_objects.contains(&source.ruletarget()) {
                    return Ok(acc);
                }
            }

            // APPLY THE RULE - reuse existing generic functions with manifest source
            let rule_row_result = match &rule.rule {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                    return Ok(acc);
                }
                CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                    columns_have_description(source, source, rule, verbose)
                }
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention,
                    data_types,
                } => column_name_convention(source, convention, data_types.as_ref(), rule, verbose),
                CatalogSpecificRuleConfig::ColumnsCanonicalName {
                    canonical,
                    invalid_names,
                    exceptions,
                } => columns_canonical_name(
                    source,
                    canonical,
                    invalid_names,
                    exceptions.as_ref(),
                    rule,
                    verbose,
                ),
            };

            if let Some(rule_row) = rule_row_result {
                acc.push((rule_row, &rule.severity));
            }

            Ok(acc)
        })
}
