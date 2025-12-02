use crate::{
    cli::table::RuleResult,
    core::{
        catalog::parse_catalog::Catalog,
        config::{severity::Severity, Config},
        manifest::Manifest,
    },
};
use owo_colors::OwoColorize;

///  Catalog checks take a more complext approach
/// (since they will iterate over the manifest objects aswell as the catalog objects)
/// # Errors
/// Returns an error if a rule has invalid configuration (e.g., invalid regex pattern)
/// This error is then bubbled up to the `run` function using anyhow.
#[allow(dead_code)]
pub fn apply_catalog_checks<'a>(
    config: &'a Config,
    _catalog: &'a Catalog,
    _manifest: &'a Manifest,
    verbose: bool,
) -> anyhow::Result<Vec<(RuleResult, &'a Severity)>> {
    #[allow(unused_mut)]
    let mut _results: Vec<(RuleResult, &'a Severity)> = Vec::new();

    if let Some(catalog_tests) = &config.catalog_tests {
        println!("{catalog_tests:?}");
        for rule in catalog_tests {
            if verbose {
                println!(
                    "{}",
                    format!("Applying catalog rule: {}", rule.get_name()).blue()
                );
            }
        }
    }

    // println!(
    //     "printing the following to hide warning {:?} {} {} {}",
    //     catalog,
    //     manifest.sources.len(),
    //     config.catalog_tests.as_ref().map_or(0, |v| v.len()),
    //     results.len()
    // );

    todo!()
}
