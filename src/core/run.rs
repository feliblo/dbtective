use crate::cli::commands::RunOptions;
use crate::cli::table::{show_results_and_exit, RuleResult};
use crate::core::catalog::Catalog;
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
use crate::core::utils::{print_catalog_warning, unwrap_or_exit};
use log::debug;
use std::time::Instant;

#[must_use]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let start = Instant::now();

    let config_path = resolve_config_path(&options.entry_point, options.config_file.as_ref());
    let config = unwrap_or_exit(Config::from_file(config_path));

    debug!("Loaded configuration: {config:#?}");

    let mut findings: Vec<(RuleResult, &Severity)> = Vec::new();

    // Manifest-based rules
    let manifest_path =
        std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.manifest_file));
    let manifest = unwrap_or_exit(Manifest::from_file(&manifest_path));

    findings.extend(unwrap_or_exit(apply_manifest_node_rules(
        &manifest, &config, verbose,
    )));
    findings.extend(unwrap_or_exit(apply_manifest_object_rules(
        &manifest, &config, verbose,
    )));

    // Catalog-based rules
    let catalog_path =
        std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.catalog_file));

    let catalog = if options.only_manifest {
        None
    } else {
        Some(unwrap_or_exit(Catalog::from_file(&catalog_path)))
    };

    // Track if any catalog tests failed
    let mut has_catalog_failures = false;

    if let Some(ref cat) = catalog {
        let mut catalog_findings = Vec::new();

        catalog_findings.extend(unwrap_or_exit(apply_catalog_node_rules(
            &config, cat, &manifest, verbose,
        )));
        catalog_findings.extend(unwrap_or_exit(apply_catalog_source_rules(
            &config, cat, &manifest, verbose,
        )));

        has_catalog_failures = !catalog_findings.is_empty();
        findings.extend(catalog_findings);
    }

    // Show results table first
    let exit_code = show_results_and_exit(
        &findings,
        verbose,
        options.entry_point.as_ref(),
        options.disable_hyperlinks,
        options.hide_warnings,
        Some(start.elapsed()),
    );

    // Print catalog warning *after* table
    if has_catalog_failures && !options.hide_catalog_tip {
        print_catalog_warning();
    }

    exit_code
}
