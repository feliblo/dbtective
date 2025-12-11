use crate::{
    cli::table::RuleResult,
    core::{
        catalog::parse_catalog::Catalog,
        checks::catalog::{check_columns_are_documented, check_columns_have_description},
        config::{catalog_rule::CatalogSpecificRuleConfig, severity::Severity, Config},
        manifest::Manifest,
    },
};
use owo_colors::OwoColorize;

///  Catalog checks take a more complext approach
/// (since they will iterate over the manifest objects aswell as the catalog objects)
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern)
/// This error is then bubbled up to the `run` function using anyhow.
///
/// Furthermore, filtering of e.g. `applies_to` is also done at the function level (1 level below)
/// This is because again the tests apply to both manifest and catalog objects,
///
/// Catalogs only contain 2 object types: nodes and sources. So we handle node cases here.
pub fn apply_catalog_node_checks<'a>(
    config: &'a Config,
    catalog: &'a Catalog,
    manifest: &'a Manifest,
    verbose: bool,
) -> Vec<(RuleResult, &'a Severity)> {
    let Some(catalog_tests) = &config.catalog_tests else {
        return Vec::new();
    };

    catalog
        .nodes
        .values()
        .flat_map(|catalog_node| catalog_tests.iter().map(move |rule| (catalog_node, rule)))
        .fold(Vec::new(), |mut acc, (catalog_node, rule)| {
            if verbose {
                println!(
                    "{}",
                    format!("Applying catalog rule: {}", rule.get_name()).blue()
                );
            }

            let Some(manifest_node) = manifest.get_node(catalog_node.get_unique_id()) else {
                // Mismatch between catalog and manifest nodes
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
                return acc;
            };

            // `applies_to` filtering has to be done from the manifest node side (only it contains the path)
            if let Some(applies) = &rule.applies_to {
                if !applies.node_objects.contains(&manifest_node.ruletarget()) {
                    return acc;
                }
            }

            // APPLY THE RULE HERE
            let check_row_result = match &rule.rule {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                    check_columns_are_documented(
                        catalog_node,
                        manifest_node,
                        rule,
                        manifest,
                        verbose,
                    )
                }
                CatalogSpecificRuleConfig::ColumnsHaveDescription {  }=> {
                    check_columns_have_description(
                        catalog_node,
                        manifest_node,
                        rule,
                        verbose,
                    )
                }
            };

            if let Some(check_row) = check_row_result {
                acc.push((check_row, &rule.severity));
            }

            acc
        })
}
