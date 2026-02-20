use crate::{
    cli::table::RuleResult,
    core::{
        config::{
            applies_to::RuleTargetable, catalog_rule::CatalogSpecificRuleConfig,
            includes_excludes::should_run_test, severity::Severity, Config,
        },
        manifest::Manifest,
        rules::catalog::{
            column_name_convention, columns_canonical_name, columns_have_data_type,
            columns_have_description,
        },
    },
};

/// Applies catalog rules in manifest-fallback mode for node objects.
/// Instead of iterating catalog nodes, this iterates manifest nodes and
/// passes them to the existing generic rule functions.
///
/// Too make this happen we simply act like the manifest object is the catalog object aswell.
///
/// Only rules where `supports_manifest_fallback()` is true are applied.
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern)
pub fn apply_catalog_fallback_node_rules<'a>(
    config: &'a Config,
    manifest: &'a Manifest,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let Some(catalog_tests) = &config.catalog_tests else {
        return Ok(Vec::new());
    };

    manifest
        .nodes
        .values()
        .flat_map(|node| catalog_tests.iter().map(move |rule| (node, rule)))
        .try_fold(Vec::new(), |mut acc, (node, rule)| -> anyhow::Result<_> {
            // Skip rules that cannot fall back to manifest-only mode
            if !rule.rule.supports_manifest_fallback() {
                return Ok(acc);
            }

            // `includes`/`excludes` filtering
            if !should_run_test(node, rule.includes.as_ref(), rule.excludes.as_ref()) {
                return Ok(acc);
            }

            if let Some(applies) = &rule.applies_to {
                if !applies.node_objects.contains(&node.ruletarget()) {
                    return Ok(acc);
                }
            }

            if let Some(allowed_materializations) = &rule.model_materializations {
                if let Some(node_materialization) = node.get_materialization() {
                    if !allowed_materializations.contains(node_materialization) {
                        return Ok(acc);
                    }
                }
            }

            // APPLY THE RULE - reuse existing generic functions with manifest node
            let rule_row_result = match &rule.rule {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                    return Ok(acc);
                }
                CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                    columns_have_description(node, node, rule, verbose)
                }
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention,
                    data_types,
                    ..
                } => column_name_convention(
                    node,
                    node,
                    convention,
                    data_types.as_ref(),
                    false,
                    rule,
                    verbose,
                ),
                CatalogSpecificRuleConfig::ColumnsCanonicalName {
                    canonical,
                    invalid_names,
                    exceptions,
                } => columns_canonical_name(
                    node,
                    canonical,
                    invalid_names,
                    exceptions.as_ref(),
                    rule,
                    verbose,
                ),
                CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage } => {
                    columns_have_data_type(node, node, rule, *min_coverage, verbose)
                }
            };

            if let Some(mut rule_row) = rule_row_result {
                rule_row.category = rule.get_category().to_string();
                acc.push((rule_row, &rule.severity));
            }

            Ok(acc)
        })
}
