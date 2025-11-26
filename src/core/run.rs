use crate::cli::commands::RunOptions;
use crate::cli::table::print_node_checks_table;
use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use log::debug;
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;

pub fn run(options: &RunOptions, verbose: bool) {
    if verbose {
        debug!("Starting dbtective analysis...");
        debug!("{options:#?}");
    }

    let start = Instant::now();
    let manifest_path =
        std::path::PathBuf::from(format!("{}/{}", options.entry_point, options.manifest_file));

    let manifest = match Manifest::from_file(&manifest_path) {
        Ok(manifest) => manifest,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            exit(1);
        }
    };
    let config = match Config::from_file(format!("{}/{}", options.entry_point, options.config_file))
    {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            exit(1);
        }
    };

    let node_checks_results = apply_node_checks(&manifest, &config);

    let results = &node_checks_results;
    if results.iter().any(|&(_, severity)| severity.as_code() != 0) {
        println!("{}", "🕵️  dbtective detected some issues:".red());
    } else {
        println!(
            "{} 🕵️",
            "All checks passed successfully! - dbtective off the case.".green(),
        );
    }

    print_node_checks_table(&node_checks_results);

    if verbose {
        let duration = start.elapsed();
        println!("Analysis completed in: {duration:?}");
    }
    exit(0);
}
