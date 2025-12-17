use crate::cli::commands::RunOptions;
use crate::cli::table::{show_results_and_exit, RuleResult};
use crate::core::catalog::parse_catalog::Catalog;
use crate::core::config::parse_config::resolve_config_path;
use crate::core::config::severity::Severity;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use crate::core::rules::catalog::{
    apply_catalog_node_rules::apply_catalog_node_rules,
    apply_catalog_source_rules::apply_catalog_source_rules,
};
use crate::core::rules::manifest::{
    apply_manifest_node_rules::apply_manifest_node_rules,
    apply_other_manifest_object_rules::apply_manifest_object_rules,
};
use crate::core::utils::unwrap_or_exit;
use log::debug;
use std::time::Instant;

#[must_use]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let start = Instant::now();

    let config_path = resolve_config_path(&options.entry_point, options.config_file.as_ref());
    let config = unwrap_or_exit(Config::from_file(config_path));

    debug!("Loaded configuration: {config:#?}");

    // Store all findings in a result vector
    let mut findings: Vec<(RuleResult, &Severity)> = Vec::new();

    // Manifest-based rules
    let manifest_path =
        std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.manifest_file));
    let manifest = unwrap_or_exit(Manifest::from_file(&manifest_path));

    // Manifest-node object rules
    findings.extend(unwrap_or_exit(apply_manifest_node_rules(
        &manifest, &config, verbose,
    )));
    // Manifest-non-node object rules (source macro exposures semantic_models unit_tests)
    findings.extend(unwrap_or_exit(apply_manifest_object_rules(
        &manifest, &config, verbose,
    )));

    // Catalog-based rules (need both manifest and catalog)
    // This can error in the following case:
    // The manifest has been rebuild using a `dbt` command,
    // yet the `catalog.json` has not been updated with `dbt docs generate`
    let catalog = if options.only_manifest {
        None
    } else {
        let catalog_path =
            std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.catalog_file));
        Some(unwrap_or_exit(Catalog::from_file(&catalog_path)))
    };

    if let Some(ref catalog) = catalog {
        findings.extend(unwrap_or_exit(apply_catalog_node_rules(
            &config, catalog, &manifest, verbose,
        )));
        findings.extend(unwrap_or_exit(apply_catalog_source_rules(
            &config, catalog, &manifest, verbose,
        )));
    }

    show_results_and_exit(
        &findings,
        verbose,
        options.entry_point.as_ref(),
        options.disable_hyperlinks,
        Some(start.elapsed()),
    )
}
