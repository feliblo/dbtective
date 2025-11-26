mod cli;
mod core;
use crate::cli::commands::{Cli, Commands};
pub use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::run::run;
use clap::{CommandFactory, Parser};
use log::{debug, info};
use owo_colors::OwoColorize;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Run { options }) => {
            if args.verbose {
                debug!("Running dbtective analysis...");
                debug!("{options:#?}");
            }
            run(options, args.verbose);
        }
        Some(Commands::Init { options }) => {
            if args.verbose {
                debug!("Initializing dbtective project...");
                debug!("{options:#?}");
                todo!();
            }
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
