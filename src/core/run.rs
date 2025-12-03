use crate::cli::commands::RunOptions;
use crate::cli::table::{show_results_and_exit, RuleResult};
use crate::core::catalog::parse_catalog::Catalog;
use crate::core::checks::catalog::catalog_checks::apply_catalog_node_checks;
use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::checks::manifest::other_manifest_object_checks::apply_manifest_object_checks;
use crate::core::config::severity::Severity;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use log::debug;
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;

fn unwrap_or_exit<T>(result: anyhow::Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            exit(1);
        }
    }
}

#[must_use]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let start = Instant::now();
    let config_path = format!("{}/{}", options.entry_point, options.config_file);
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
    }

    show_results_and_exit(
        &findings,
        verbose,
        options.entry_point.as_ref(),
        Some(start.elapsed()),
    )
}
