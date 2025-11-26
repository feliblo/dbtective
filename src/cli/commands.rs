use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum LimitOptions {
    /// Analyze dbt models
    Models,
    /// Analyze dbt tests
    Tests,
    /// Analyze dbt sources
    Sources,
    /// Analyze dbt snapshots
    Snapshots,
    /// Analyze dbt seeds
    Seeds,
    /// Analyze dbt macros
    Macros,
    /// Analyze dbt exposures
    Exposures,
    /// Analyze dbt metrics
    Metrics,
}

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

    /// Path to pyproject.toml file
    #[arg(long, short, default_value = "pyproject.toml")]
    pub pyproject_file: String,

    /// Path to config toml file
    #[arg(long, short, default_value = "dbtective.yml")]
    pub config_file: String,

    #[arg(long, short, default_value = "target/manifest.json")]
    pub manifest_file: String,

    /// Optional output file with test results
    #[arg(long, short)]
    pub output_file: Option<String>,

    /// Only check for certain dbt object types
    #[arg(long, short)]
    pub limit: Option<LimitOptions>,
}

#[cfg(test)]
mod tests {
    use crate::cli::commands::{Cli, Commands, InitOptions, LimitOptions, RunOptions};

    #[test]
    fn test_limit_options_enum_variants() {
        // Test all enum variants exist and can be constructed
        let variants = [
            LimitOptions::Models,
            LimitOptions::Tests,
            LimitOptions::Sources,
            LimitOptions::Snapshots,
            LimitOptions::Seeds,
            LimitOptions::Macros,
            LimitOptions::Exposures,
            LimitOptions::Metrics,
        ];

        for variant in variants {
            // Test Debug trait implementation
            let debug_str = format!("{variant:?}");
            assert!(!debug_str.is_empty());

            // Test Clone trait implementation
            let cloned = variant.clone();
            assert_eq!(format!("{cloned:?}"), format!("{variant:?}"));
        }
    }

    #[test]
    fn test_limit_options_debug_output() {
        // Test specific debug output for each variant
        assert_eq!(format!("{:?}", LimitOptions::Models), "Models");
        assert_eq!(format!("{:?}", LimitOptions::Tests), "Tests");
        assert_eq!(format!("{:?}", LimitOptions::Sources), "Sources");
        assert_eq!(format!("{:?}", LimitOptions::Snapshots), "Snapshots");
        assert_eq!(format!("{:?}", LimitOptions::Seeds), "Seeds");
        assert_eq!(format!("{:?}", LimitOptions::Macros), "Macros");
        assert_eq!(format!("{:?}", LimitOptions::Exposures), "Exposures");
        assert_eq!(format!("{:?}", LimitOptions::Metrics), "Metrics");
    }

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
            pyproject_file: "pyproject.toml".to_string(),
            config_file: "dbtective.toml".to_string(),
            output_file: Some("output.json".to_string()),
            limit: Some(LimitOptions::Models),
        };
        let debug_str = format!("{options:?}");
        assert!(debug_str.contains("RunOptions"));
        assert!(debug_str.contains("entry_point"));
        assert!(debug_str.contains("pyproject"));
        assert!(debug_str.contains("config_file"));
        assert!(debug_str.contains("output_file"));
        assert!(debug_str.contains("limit"));
    }

    #[test]
    fn test_run_options_default_values() {
        // Test creating RunOptions with default-like values
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "./".to_string(),
            pyproject_file: "pyproject.toml".to_string(),
            config_file: "dbtective.toml".to_string(),
            output_file: None,
            limit: None,
        };

        assert_eq!(options.entry_point, "./");
        assert_eq!(options.pyproject_file, "pyproject.toml");
        assert!(options.output_file.is_none());
        assert!(options.limit.is_none());
    }

    #[test]
    fn test_run_options_with_all_fields() {
        let options = RunOptions {
            manifest_file: "custom_manifest.json".to_string(),
            entry_point: "/path/to/project".to_string(),
            pyproject_file: "custom_pyproject.toml".to_string(),
            config_file: "custom_config.toml".to_string(),
            output_file: Some("results.json".to_string()),
            limit: Some(LimitOptions::Tests),
        };

        assert_eq!(options.entry_point, "/path/to/project");
        assert_eq!(options.pyproject_file, "custom_pyproject.toml");
        assert_eq!(options.config_file, "custom_config.toml".to_string());
        assert_eq!(options.output_file, Some("results.json".to_string()));
        assert_eq!(format!("{:?}", options.limit), "Some(Tests)");
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
                pyproject_file: "pyproject.toml".to_string(),
                config_file: String::new(),
                output_file: None,
                limit: None,
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
        // Test CLI struct can be created
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
                    pyproject_file: "pyproject.toml".to_string(),
                    config_file: "config.toml".to_string(),
                    output_file: Some("output.json".to_string()),
                    limit: Some(LimitOptions::Models),
                },
            }),
        };

        assert!(cli.verbose);

        match &cli.command {
            Some(Commands::Run { options }) => {
                assert_eq!(options.entry_point, "./src");
                assert_eq!(options.pyproject_file, "pyproject.toml");
                assert_eq!(options.config_file, "config.toml".to_string());
                assert_eq!(options.output_file, Some("output.json".to_string()));
                assert!(options.limit.is_some());
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
                pyproject_file: "pyproject.toml".to_string(),
                config_file: "dbtective.toml".to_string(),
                output_file: None,
                limit: None,
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
