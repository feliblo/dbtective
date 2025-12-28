use crate::{
    cli::table::RuleResult,
    core::{
        catalog::Catalog,
        config::{
            applies_to::RuleTargetable, catalog_rule::CatalogSpecificRuleConfig,
            severity::Severity, Config,
        },
        manifest::Manifest,
        rules::catalog::{
            column_name_convention, columns_are_documented, columns_have_description,
        },
    },
};
use owo_colors::OwoColorize;

///  Catalog rules take a more complex approach
/// (since they will iterate over the manifest objects aswell as the catalog objects)
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern)
/// This error is then bubbled up to the `run` function using anyhow.
///
/// Furthermore, filtering of e.g. `applies_to` is also done at the function level (1 level below)
/// This is because again the tests apply to both manifest and catalog objects,
///
/// Catalogs only contain 2 object types: nodes and sources. We handle source cases here.
pub fn apply_catalog_source_rules<'a>(
    config: &'a Config,
    catalog: &'a Catalog,
    manifest: &'a Manifest,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let Some(catalog_tests) = &config.catalog_tests else {
        return Ok(Vec::new());
    };

    catalog
        .sources
        .values()
        .flat_map(|catalog_source| catalog_tests.iter().map(move |rule| (catalog_source, rule)))
        .try_fold(Vec::new(), |mut acc, (catalog_source, rule)| -> anyhow::Result<_> {
            let Some(manifest_source) = manifest.get_source(catalog_source.get_unique_id()) else {
                // Mismatch between catalog and manifest sources
                println!(
                    "{}",
                    format!(
                        "Warning: No matching manifest source found for catalog source '{}'.\n\
                        This may be due to differences in execution runs, renamed or moved files.\n\
                        Consider removing 'catalog.json' and regenerating it using 'dbt docs generate'.",
                        catalog_source.get_name()
                    )
                    .yellow()
                );
                return Ok(acc);
            };

            // `applies_to` filtering has to be done from the manifest source side (only it contains the path)
            if let Some(applies) = &rule.applies_to {
                if !applies.source_objects.contains(&manifest_source.ruletarget()) {
                    return Ok(acc);
                }
            }

            // APPLY THE RULE HERE
            let rule_row_result = match &rule.rule {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                    columns_are_documented(
                        catalog_source,
                        manifest_source,
                        rule,
                        manifest,
                        verbose,
                    )
                }
                CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                    columns_have_description(
                        catalog_source,
                        manifest_source,
                        rule,
                        verbose,
                    )
                }
                CatalogSpecificRuleConfig::ColumnsNameConvention { convention, data_types } => {
                    column_name_convention(
                        catalog_source,
                        convention,
                        data_types.as_ref(),
                        rule,
                        verbose,
                    )
                }
            };

            if let Some(rule_row) = rule_row_result {
                acc.push((rule_row, &rule.severity));
            }

            Ok(acc)
        })
}
