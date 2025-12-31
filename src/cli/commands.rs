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

#[derive(Args, Debug, Clone)]
pub struct InitOptions {
    /// Directory where the config file should be created
    #[arg(long, short = 'l', default_value = ".")]
    pub location: String,

    /// Configuration format to generate (yml, toml, or pyproject)
    #[arg(long, short = 'f', value_parser = ["yml", "yaml", "toml", "pyproject"], default_value = "yml")]
    pub format: String,
}

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

    /// Hide warnings from the output (only show errors)
    #[arg(long, default_value_t = false)]
    pub hide_warnings: bool,
}

#[cfg(test)]
mod tests {
    use crate::cli::commands::{Cli, Commands, InitOptions, RunOptions};

    fn default_init_options() -> InitOptions {
        InitOptions {
            location: ".".to_string(),
            format: "yml".to_string(),
        }
    }

    #[test]
    fn test_init_options_debug() {
        let options = default_init_options();
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("InitOptions"));
        assert!(debug_str.contains("location"));
        assert!(debug_str.contains("format"));
    }

    #[test]
    fn test_init_options_with_custom_values() {
        let options = InitOptions {
            location: "/custom/path".to_string(),
            format: "toml".to_string(),
        };
        assert_eq!(options.location, "/custom/path");
        assert_eq!(options.format, "toml");
    }

    #[test]
    fn test_init_options_pyproject_format() {
        let options = InitOptions {
            location: ".".to_string(),
            format: "pyproject".to_string(),
        };
        assert_eq!(options.format, "pyproject");
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
            hide_warnings: false,
        };
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("RunOptions"));
        assert!(debug_str.contains("entry_point"));
        assert!(debug_str.contains("config_file"));
    }

    #[test]
    fn test_run_options_default_values() {
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "./".to_string(),
            config_file: Some("dbtective.toml".to_string()),
            catalog_file: "target/catalog.json".to_string(),
            only_manifest: false,
            disable_hyperlinks: false,
            hide_warnings: false,
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
            hide_warnings: false,
        };

        assert_eq!(options.entry_point, "/path/to/project");
        assert_eq!(options.config_file, Some("custom_config.toml".to_string()));
        assert!(options.only_manifest);
    }

    #[test]
    fn test_commands_enum_variants() {
        let init_cmd = Commands::Init {
            options: default_init_options(),
        };

        match init_cmd {
            Commands::Init { options: _ } => {}
            Commands::Run { .. } => panic!("Expected Init variant"),
        }

        let run_cmd = Commands::Run {
            options: RunOptions {
                manifest_file: "custom_manifest.json".to_string(),
                entry_point: "./".to_string(),
                config_file: None,
                catalog_file: "target/catalog.json".to_string(),
                only_manifest: false,
                disable_hyperlinks: false,
                hide_warnings: false,
            },
        };

        match run_cmd {
            Commands::Run { options: _ } => {}
            Commands::Init { .. } => panic!("Expected Run variant"),
        }
    }

    #[test]
    fn test_cli_structure() {
        let cli = Cli {
            verbose: true,
            command: Some(Commands::Init {
                options: default_init_options(),
            }),
        };

        assert!(cli.verbose);
        assert!(cli.command.is_some());

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
                    hide_warnings: false,
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
            options: default_init_options(),
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
                hide_warnings: false,
            },
        };
        let debug_str = format!("{run_cmd:?}");
        assert!(debug_str.contains("Run"));
    }

    #[test]
    fn test_comprehensive_combinations() {
        let test_cases = [
            (true, None),
            (false, None),
            (
                true,
                Some(Commands::Init {
                    options: default_init_options(),
                }),
            ),
            (
                false,
                Some(Commands::Init {
                    options: default_init_options(),
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
