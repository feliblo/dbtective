use crate::cli::commands::{OutputFormat, RunOptions};
use crate::cli::structured_output::{write_output, StructuredOutput};
use crate::cli::table::{show_results_and_exitcode, RuleResult};
use crate::core::catalog::Catalog;
use crate::core::config::parse_config::resolve_config_path;
use crate::core::config::severity::Severity;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use crate::core::rules::catalog::{
    apply_catalog_fallback_node_rules::apply_catalog_fallback_node_rules,
    apply_catalog_fallback_source_rules::apply_catalog_fallback_source_rules,
    apply_catalog_node_rules::apply_catalog_node_rules,
    apply_catalog_source_rules::apply_catalog_source_rules,
};
use crate::core::rules::manifest::{
    apply_manifest_node_rules::apply_manifest_node_rules,
    apply_other_manifest_object_rules::apply_manifest_object_rules,
};
use crate::core::utils::{print_catalog_warning, unwrap_or_exit};
use chrono::Utc;
use log::debug;
use std::collections::HashSet;
use std::time::Instant;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let started_at = Utc::now().to_rfc3339();
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

    // Track if any catalog tests failed and whether we used fallback mode
    let mut has_catalog_failures = false;
    let mut fallback_rules: Vec<String> = Vec::new();
    let mut skipped_rules: Vec<String> = Vec::new();

    if let Some(ref cat) = catalog {
        // Normal catalog mode
        let mut catalog_findings = Vec::new();

        catalog_findings.extend(unwrap_or_exit(apply_catalog_node_rules(
            &config, cat, &manifest, verbose,
        )));
        catalog_findings.extend(unwrap_or_exit(apply_catalog_source_rules(
            &config, cat, &manifest, verbose,
        )));

        has_catalog_failures = !catalog_findings.is_empty();
        findings.extend(catalog_findings);
    } else if config.catalog_tests.is_some() {
        // Manifest-fallback mode: --only-manifest is set but catalog_tests exist
        // Collect which rules ran in fallback and which were skipped
        if let Some(catalog_tests) = &config.catalog_tests {
            let mut seen_fallback = HashSet::new();
            let mut seen_skipped = HashSet::new();
            for rule in catalog_tests {
                let name = rule.get_name();
                if rule.rule.supports_manifest_fallback() {
                    if seen_fallback.insert(name.clone()) {
                        fallback_rules.push(name);
                    }
                } else if seen_skipped.insert(name.clone()) {
                    skipped_rules.push(name);
                }
            }
        }

        let mut fallback_findings = Vec::new();
        fallback_findings.extend(unwrap_or_exit(apply_catalog_fallback_node_rules(
            &config, &manifest, verbose,
        )));
        fallback_findings.extend(unwrap_or_exit(apply_catalog_fallback_source_rules(
            &config, &manifest, verbose,
        )));

        has_catalog_failures = !fallback_findings.is_empty();
        findings.extend(fallback_findings);
    }

    let elapsed = start.elapsed();

    let exit_code = match options.output_format {
        OutputFormat::Table => {
            let code = show_results_and_exitcode(
                &findings,
                verbose,
                options.entry_point.as_ref(),
                options.disable_hyperlinks,
                options.hide_warnings,
                Some(elapsed),
            );

            if has_catalog_failures && !options.hide_catalog_tip {
                print_catalog_warning(&fallback_rules, &skipped_rules);
            }

            code
        }
        OutputFormat::Json | OutputFormat::Csv | OutputFormat::Ndjson => {
            let project_name = manifest
                .metadata
                .project_name
                .as_deref()
                .unwrap_or("unknown");
            let output = StructuredOutput::from_results(
                &findings,
                &started_at,
                elapsed,
                options.hide_warnings,
                project_name,
                &options.manifest_file,
                &options.catalog_file,
            );

            let rendered = match options.output_format {
                OutputFormat::Json => output.to_json(),
                OutputFormat::Csv => output.to_csv(),
                OutputFormat::Ndjson => output.to_ndjson(),
                OutputFormat::Table => unreachable!(),
            };

            unwrap_or_exit(write_output(&rendered, options.output_file.as_deref()));

            i32::from(findings.iter().any(|(_, sev)| **sev == Severity::Error))
        }
    };

    exit_code
}
