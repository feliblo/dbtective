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
    #[arg(long, default_value = "./")]
    pub entry_point: String,

    /// Path to pyproject.toml file
    #[arg(long, short, default_value = "pyproject.toml")]
    pub pyproject_file: String,

    /// Path to config toml file
    #[arg(long, short, default_value = "dbtective.toml")]
    pub config_file: String,

    /// Optional output file with test results
    #[arg(long, short)]
    pub output_file: Option<String>,

    /// Only check for certain dbt object types
    #[arg(long, short)]
    pub limit: Option<LimitOptions>,
}
