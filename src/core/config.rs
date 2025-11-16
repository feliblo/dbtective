use anyhow::{Context, Result};
use config::Config;
use std::path::Path;

pub fn parse_config(pyproject_path: &str, dbtective_config_path: &str) -> Result<Config> {
    if !Path::new(pyproject_path).exists() && !Path::new(dbtective_config_path).exists() {
        return Err(anyhow::anyhow!(
            "pyproject.toml [{}] or dbtective configuration [{}] not found. \n Please ensure at least one is available.",
            pyproject_path,
            dbtective_config_path
        ));
    }

    let config = Config::builder()
        .add_source(config::File::with_name(pyproject_path))
        .add_source(config::File::with_name(dbtective_config_path))
        .build()
        .with_context(|| {
            format!("Failed to build config from {pyproject_path} and {dbtective_config_path}")
        })?;

    Ok(config)
}
