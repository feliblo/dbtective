use crate::cli::commands::RunOptions;
use crate::cli::table::{show_results_and_exit, RuleResult};
use crate::core::catalog::parse_catalog::Catalog;
use crate::core::checks::catalog::{
    catalog_node_checks::apply_catalog_node_checks,
    catalog_source_checks::apply_catalog_source_checks,
};
use crate::core::checks::manifest::{
    node_checks::apply_node_checks, other_manifest_object_checks::apply_manifest_object_checks,
};
use crate::core::config::parse_config::resolve_config_path;
use crate::core::config::severity::Severity;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use crate::core::utils::unwrap_or_exit;
use log::debug;
use owo_colors::OwoColorize;
use std::time::Instant;

#[must_use]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let start = Instant::now();

    let config_path = resolve_config_path(&options.entry_point, options.config_file.as_ref());
    let config = unwrap_or_exit(Config::from_file(config_path));

    debug!("Loaded configuration: {config:#?}");

    // Store all findings in a result vector
    let mut findings: Vec<(RuleResult, &Severity)> = Vec::new();

    // Manifest-based checks
    let manifest_path =
        std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.manifest_file));
    let manifest = unwrap_or_exit(Manifest::from_file(&manifest_path));

    // Manifest-node object checks
    findings.extend(unwrap_or_exit(apply_node_checks(
        &manifest, &config, verbose,
    )));
    // Manifest-non-node object checks (source macro exposures semantic_models unit_tests)
    findings.extend(unwrap_or_exit(apply_manifest_object_checks(
        &manifest, &config, verbose,
    )));

    // Catalog-based checks (need both manifest and catalog)
    // This can error in the following case:
    // The manifest has been rebuild using a `dbt` command,
    // yet the `catalog.json` has not been updated with `dbt docs generate`
    let catalog = if options.only_manifest {
        println!(
            "{}",
            "Skipping catalog-based checks, due to --only-manifest flag".blue()
        );
        None
    } else {
        let catalog_path =
            std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.catalog_file));
        Some(unwrap_or_exit(Catalog::from_file(&catalog_path)))
    };

    if let Some(ref catalog) = catalog {
        findings.extend(apply_catalog_node_checks(
            &config, catalog, &manifest, verbose,
        ));
        findings.extend(apply_catalog_source_checks(
            &config, catalog, &manifest, verbose,
        ));
    }

    show_results_and_exit(
        &findings,
        verbose,
        options.entry_point.as_ref(),
        options.disable_hyperlinks,
        Some(start.elapsed()),
    )
}
