use crate::cli::commands::RunOptions;
use crate::cli::table::show_results;
use crate::core::catalog::parse_catalog::Catalog;
use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::checks::manifest::source_checks::apply_source_checks;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use log::debug;
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;

#[must_use]
pub fn run(options: &RunOptions, verbose: bool) -> i32 {
    let start = Instant::now();
    let config = match Config::from_file(format!("{}/{}", options.entry_point, options.config_file))
    {
        Ok(cfg) => {
            debug!("Loaded configuration: {cfg:#?}");
            cfg
        }
        Err(err) => {
            debug!("Failed to load configuration: {err}");
            eprintln!("{}", err.to_string().red());
            exit(1);
        }
    };

    let mut findings: Vec<(
        crate::cli::table::RuleResult,
        &crate::core::config::severity::Severity,
    )> = Vec::new();

    // Manifest-based checks
    let manifest = if options.only_catalog {
        None
    } else {
        let manifest_path =
            std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.manifest_file));

        match Manifest::from_file(&manifest_path) {
            Ok(manifest) => Some(manifest),
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit(1);
            }
        }
    };

    if let Some(ref manifest) = manifest {
        match apply_node_checks(manifest, &config, verbose) {
            Ok(f) => findings.extend(f),
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit(1);
            }
        }

        match apply_source_checks(manifest, &config, verbose) {
            Ok(source_findings) => findings.extend(source_findings),
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit(1);
            }
        }
    }

    // Catalog-based checks (needs both manifest and catalog)
    let catalog = if options.only_manifest {
        None
    } else {
        let catalog_path =
            std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.catalog_file));
        match Catalog::from_file(&catalog_path) {
            Ok(catalog) => Some(catalog),
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                exit(1);
            }
        }
    };

    if let Some(ref _catalog) = catalog {
        if let Some(ref _manifest) = manifest {
            dbg!("Applying catalog-based checks");
            todo!()
        } else {
            eprintln!(
                "{}",
                "Catalog-based checks require both a manifest and a catalog".red()
            );
            exit(1);
        }
    } else {
        println!("Skipping catalog-based checks");
    }
    show_results(
        &findings,
        verbose,
        options.entry_point.as_ref(),
        Some(start.elapsed()),
    )
}
