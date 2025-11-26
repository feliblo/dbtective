use crate::cli::commands::RunOptions;
use crate::cli::table::show_results;
use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use log::debug;
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;

pub fn run(options: &RunOptions, verbose: bool) {
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

    let node_checks_results = apply_node_checks(&manifest, &config, verbose);
    show_results(&node_checks_results, verbose, Some(start.elapsed()));
}
