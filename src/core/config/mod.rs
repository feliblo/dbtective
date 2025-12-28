pub mod applies_to;
pub mod includes_excludes;
pub mod naming_convention;
pub mod parse_config;
pub mod severity;
pub use parse_config::Config;
pub mod catalog_rule;
pub mod check_config_options;
pub mod manifest_rule;

// Re-export Materialization from dbt_artifact_parser
pub use dbt_artifact_parser::manifest::Materialization;
