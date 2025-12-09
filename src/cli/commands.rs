use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Verbosity level
    #[arg(long, short, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new dbtective project
    Init {
        #[command(flatten)]
        options: InitOptions,
    },
    /// Run dbtective analysis
    Run {
        #[command(flatten)]
        options: RunOptions,
    },
}

#[derive(Args, Debug)]
pub struct InitOptions {}

#[derive(Args, Debug)]
pub struct RunOptions {
    /// Path to dbt project root directory
    #[arg(long, default_value = ".")]
    pub entry_point: String,

    #[arg(long, short = 'c')]
    pub config_file: Option<String>,

    #[arg(long, short = 'm', default_value = "target/manifest.json")]
    pub manifest_file: String,

    #[arg(long, short = 'g', default_value = "target/catalog.json")]
    pub catalog_file: String,

    #[arg(long, default_value_t = false)]
    pub only_manifest: bool,

    #[arg(long, default_value_t = false)]
    pub disable_hyperlinks: bool,
}

#[cfg(test)]
mod tests {
    use crate::cli::commands::{Cli, Commands, InitOptions, RunOptions};
    #[test]
    fn test_init_options_debug() {
        let options = InitOptions {};
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("InitOptions"));
    }

    #[test]
    fn test_run_options_debug() {
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "./".to_string(),
            config_file: Some("dbtective.toml".to_string()),
            catalog_file: "target/catalog.json".to_string(),
            only_manifest: false,
            disable_hyperlinks: false,
        };
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("RunOptions"));
        assert!(debug_str.contains("entry_point"));
        assert!(debug_str.contains("config_file"));
    }

    #[test]
    fn test_run_options_default_values() {
        // Test creating RunOptions with default-like values
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "./".to_string(),
            config_file: Some("dbtective.toml".to_string()),
            catalog_file: "target/catalog.json".to_string(),
            only_manifest: false,
            disable_hyperlinks: false,
        };

        assert_eq!(options.entry_point, "./");
        assert_eq!(options.config_file, Some("dbtective.toml".to_string()));
        assert!(!options.only_manifest);
    }

    #[test]
    fn test_run_options_with_all_fields() {
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "/path/to/project".to_string(),
            config_file: Some("custom_config.toml".to_string()),
            catalog_file: "custom_catalog.json".to_string(),
            only_manifest: true,
            disable_hyperlinks: false,
        };

        assert_eq!(options.entry_point, "/path/to/project");
        assert_eq!(options.config_file, Some("custom_config.toml".to_string()));
        assert!(options.only_manifest);
    }

    #[test]
    fn test_commands_enum_variants() {
        // Test Init command variant
        let init_cmd = Commands::Init {
            options: InitOptions {},
        };

        match init_cmd {
            Commands::Init { options: _ } => {
                // Test passes if we match the Init variant
            }
            Commands::Run { .. } => panic!("Expected Init variant"),
        }

        // Test Run command variant
        let run_cmd = Commands::Run {
            options: RunOptions {
                manifest_file: "custom_manifest.json".to_string(),
                entry_point: "./".to_string(),
                config_file: None,
                catalog_file: "target/catalog.json".to_string(),
                only_manifest: false,
                disable_hyperlinks: false,
            },
        };

        match run_cmd {
            Commands::Run { options: _ } => {
                // Test passes if we match the Run variant
            }
            Commands::Init { .. } => panic!("Expected Run variant"),
        }
    }

    #[test]
    fn test_cli_structure() {
        let cli = Cli {
            verbose: true,
            command: Some(Commands::Init {
                options: InitOptions {},
            }),
        };

        assert!(cli.verbose);
        assert!(cli.command.is_some());

        // Test with None command
        let cli_no_cmd = Cli {
            verbose: false,
            command: None,
        };

        assert!(!cli_no_cmd.verbose);
        assert!(cli_no_cmd.command.is_none());
    }

    #[test]
    fn test_cli_with_run_command() {
        let cli = Cli {
            verbose: true,
            command: Some(Commands::Run {
                options: RunOptions {
                    manifest_file: "custom_manifest.json".to_string(),
                    entry_point: "./src".to_string(),
                    catalog_file: "target/catalog.json".to_string(),
                    config_file: Some("config.toml".to_string()),
                    only_manifest: false,
                    disable_hyperlinks: false,
                },
            }),
        };

        assert!(cli.verbose);

        match &cli.command {
            Some(Commands::Run { options }) => {
                assert_eq!(options.entry_point, "./src");
                assert_eq!(options.config_file, Some("config.toml".to_string()));
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_commands_debug_output() {
        let init_cmd = Commands::Init {
            options: InitOptions {},
        };
        let debug_str = format!("{init_cmd:?}");
        assert!(debug_str.contains("Init"));

        let run_cmd = Commands::Run {
            options: RunOptions {
                manifest_file: "custom_manifest.json".to_string(),
                entry_point: "./".to_string(),
                config_file: Some("dbtective.toml".to_string()),
                catalog_file: "target/catalog.json".to_string(),
                only_manifest: false,
                disable_hyperlinks: false,
            },
        };
        let debug_str = format!("{run_cmd:?}");
        assert!(debug_str.contains("Run"));
    }

    // Test that we can construct all combinations for better coverage
    #[test]
    fn test_comprehensive_combinations() {
        let test_cases = [
            (true, None),
            (false, None),
            (
                true,
                Some(Commands::Init {
                    options: InitOptions {},
                }),
            ),
            (
                false,
                Some(Commands::Init {
                    options: InitOptions {},
                }),
            ),
        ];

        for (verbose, command) in test_cases {
            let cli = Cli { verbose, command };
            assert_eq!(cli.verbose, verbose);

            match cli.command {
                None => assert!(cli.command.is_none()),
                Some(_) => assert!(cli.command.is_some()),
            }
        }
    }
}
