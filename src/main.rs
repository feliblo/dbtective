mod cli;
mod core;
use crate::cli::commands::{Cli, Commands};
use crate::cli::table::print_result_table;
pub use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use clap::{CommandFactory, Parser};
use log::{debug, info};
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Run { options }) => {
            if args.verbose {
                debug!("Starting dbtective analysis...");
                debug!("{options:#?}");
            }

            let start = Instant::now();
            let manifest_path = std::path::PathBuf::from(format!(
                "{}/{}",
                options.entry_point, options.manifest_file
            ));

            let manifest = match Manifest::from_file(&manifest_path) {
                Ok(manifest) => manifest,
                Err(err) => {
                    eprintln!("{}", err.to_string().red());
                    exit(1);
                }
            };
            let config =
                match Config::from_file(format!("{}/{}", options.entry_point, options.config_file))
                {
                    Ok(cfg) => cfg,
                    Err(err) => {
                        eprintln!("{}", err.to_string().red());
                        exit(1);
                    }
                };

            let node_checks_results = apply_node_checks(&manifest, &config);

            print_result_table(node_checks_results);

            if args.verbose {
                let duration = start.elapsed();
                println!("Analysis completed in: {duration:?}");
            }
        }

        Some(Commands::Init { options }) => {
            if args.verbose {
                debug!("Initializing dbtective project...");
                debug!("{options:#?}");
            }
            // Initialization logic here
        }
        None => {
            info!(
                "\n {}",
                r"
                ██████╗ ██████╗ ████████╗███████╗ ██████╗████████╗██╗██╗   ██╗███████╗
                ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██║██║   ██║██╔════╝
                ██║  ██║██████╔╝   ██║   █████╗  ██║        ██║   ██║██║   ██║█████╗
                ██║  ██║██╔══██╗   ██║   ██╔══╝  ██║        ██║   ██║╚██╗ ██╔╝██╔══╝
                ██████╔╝██████╔╝   ██║   ███████╗╚██████╗   ██║   ██║ ╚████╔╝ ███████╗
                ╚═════╝ ╚═════╝    ╚═╝   ╚══════╝ ╚═════╝   ╚═╝   ╚═╝  ╚═══╝  ╚══════╝

                "
            );
            info!(
                "{}",
                "\t \t 🕵️ \t dbtective - On the case for your dbt best practices! \t 🕵️ \n".red()
            );
            Cli::command().print_help().unwrap();
        }
    }
}
