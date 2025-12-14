mod cli;
mod core;
use crate::cli::commands::{Cli, Commands};
use crate::core::init::init;
use crate::core::run::run;
use clap::{CommandFactory, Parser};
use log::debug;
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
            std::process::exit(run(options, args.verbose));
        }
        Some(Commands::Init { options }) => {
            if args.verbose {
                debug!("Initializing dbtective project...");
                debug!("{options:#?}");
            }
            std::process::exit(init(options, args.verbose));
        }
        None => {
            println!(
                r"
                ██████╗ ██████╗ ████████╗███████╗ ██████╗████████╗██╗██╗   ██╗███████╗
                ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██║██║   ██║██╔════╝
                ██║  ██║██████╔╝   ██║   █████╗  ██║        ██║   ██║██║   ██║█████╗
                ██║  ██║██╔══██╗   ██║   ██╔══╝  ██║        ██║   ██║╚██╗ ██╔╝██╔══╝
                ██████╔╝██████╔╝   ██║   ███████╗╚██████╗   ██║   ██║ ╚████╔╝ ███████╗
                ╚═════╝ ╚═════╝    ╚═╝   ╚══════╝ ╚═════╝   ╚═╝   ╚═╝  ╚═══╝  ╚══════╝

                "
            );
            println!(
                "{}",
                "\t \t 🕵️ \t dbtective - On the case for your dbt best practices! \t 🕵️ \n".red()
            );
            Cli::command().print_help().unwrap();
        }
    }
}
