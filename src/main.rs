mod cli;
mod core;
#[cfg(test)]
mod test;

use crate::cli::commands::{Cli, Commands};
use crate::core::config::parse_config;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use log::{debug, error, info};

fn main() {
    dotenv::dotenv().ok();
    colored::control::set_override(true);

    let _debug_mode = cfg!(debug_assertions);
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Run { options }) => {
            if args.verbose {
                let config = parse_config(&options.pyproject_file, &options.config_file)
                    .expect("Failed to parse configuration");

                error!("{:?}", config);

                // Apply color to the text, not the emoji
                debug!("{} {}", "🕵️", "\t Starting dbt analysis...".red());
                debug!("\n {options:#?}");
            }
        }
        Some(Commands::Init { options }) => {
            if args.verbose {
                // Apply color to the text, not the emoji
                debug!("{} {}", "🚀", "Initializing dbtective project...".blue());
                debug!("\n {options:#?}");
            }
        }
        None => {
            debug!(" HELLO ");
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
