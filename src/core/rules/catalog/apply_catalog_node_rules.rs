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
/// Catalogs only contain 2 object types: nodes and sources. So we handle node cases here.
pub fn apply_catalog_node_rules<'a>(
    config: &'a Config,
    catalog: &'a Catalog,
    manifest: &'a Manifest,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    let Some(catalog_tests) = &config.catalog_tests else {
        return Ok(Vec::new());
    };

    catalog
        .nodes
        .values()
        .flat_map(|catalog_node| catalog_tests.iter().map(move |rule| (catalog_node, rule)))
        .try_fold(Vec::new(), |mut acc, (catalog_node, rule)| -> anyhow::Result<_> {
            let Some(manifest_node) = manifest.get_node(catalog_node.get_unique_id()) else {
                println!(
                    "{}",
                    format!(
                        "Warning: No matching manifest node found for catalog node '{}'.\n\
                        This may be due to differences in execution runs, renamed or moved files.\n\
                        Consider removing 'catalog.json' and regenerating it using 'dbt docs generate'.",
                        catalog_node.get_name()
                    )
                    .yellow()
                );
                return Ok(acc);
            };

            // `applies_to` filtering
            if let Some(applies) = &rule.applies_to {
                if !applies.node_objects.contains(&manifest_node.ruletarget()) {
                    return Ok(acc);
                }
            }

            // `model_materializations` filtering
            if let Some(allowed_materializations) = &rule.model_materializations {
                if let Some(node_materialization) = manifest_node.get_materialization() {
                    if !allowed_materializations.contains(node_materialization) {
                        return Ok(acc);
                    }
                }
            }

            // APPLY THE RULE HERE
            let rule_row_result = match &rule.rule {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                    columns_are_documented(
                        catalog_node,
                        manifest_node,
                        rule,
                        manifest,
                        verbose,
                    )
                }
                CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                    columns_have_description(
                        catalog_node,
                        manifest_node,
                        rule,
                        verbose,
                    )
                }
                CatalogSpecificRuleConfig::ColumnsNameConvention { convention, data_types } => {
                    column_name_convention(
                        catalog_node,
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
