use crate::core::config::manifest_rule::ManifestRule;
use crate::core::config::{
    catalog_rule::default_applies_to_for_catalog_rule, catalog_rule::CatalogRule,
    manifest_rule::default_applies_to_for_manifest_rule,
};
use crate::core::utils::unwrap_or_exit;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub manifest_tests: Option<Vec<ManifestRule>>,
    pub catalog_tests: Option<Vec<CatalogRule>>,
}

#[derive(Deserialize)]
struct PyProject {
    tool: Tool,
}
// tool.dbtective in pyproject.toml
#[derive(Deserialize)]
struct Tool {
    dbtective: Config,
}

impl Config {
    /// Finds and selects the appropriate config file from a directory.
    /// Searches for config files in the following preference order:
    /// 1. dbtective.yml or dbtective.yaml (highest priority)
    /// 2. dbtective.toml
    /// 3. pyproject.toml (lowest priority)
    ///
    /// # Returns
    /// Returns a tuple of `(chosen_config_name, all_found_configs)`
    ///
    /// # Errors
    /// Returns an error if no config files are found
    pub fn find_config_in_dir<P: AsRef<Path>>(dir: P) -> Result<(String, Vec<String>)> {
        let dir = dir.as_ref();

        let possible_configs = [
            ("dbtective.yml", 1),
            ("dbtective.yaml", 1), // same priority as .yml
            ("dbtective.toml", 2),
            ("pyproject.toml", 3),
        ];

        let mut found_configs = Vec::new();

        for (config_name, priority) in &possible_configs {
            let config_path = dir.join(config_name);
            if config_path.exists() {
                found_configs.push(((*config_name).to_string(), *priority));
            }
        }

        if found_configs.is_empty() {
            return Err(anyhow::anyhow!(
                "No dbtective config file found in {}. Looked for: dbtective.yml, dbtective.yaml, dbtective.toml, or pyproject.toml",
                dir.display()
            ));
        }

        // Sort by priority (lower number = higher priority)
        found_configs.sort_by_key(|(_, priority)| *priority);

        let chosen_config = found_configs[0].0.clone();
        let all_found: Vec<String> = found_configs.into_iter().map(|(name, _)| name).collect();

        Ok((chosen_config, all_found))
    }

    /// Load and parse the configuration from a file, auto-detecting the format
    /// Supports YAML (.yml, .yaml), TOML (.toml), and pyproject.toml
    /// # Errors
    /// Returns an error if the file cannot be opened, the format is unsupported, or parsing fails
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Get the file name and extension
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file name")?;

        let extension = path.extension().and_then(|e| e.to_str());

        // Auto-detect format based on file name or extension
        if file_name == "pyproject.toml" {
            Self::from_pyproject(path)
        } else {
            match extension {
                Some("yml" | "yaml") => Self::from_yaml(path),
                Some("toml") => Self::from_toml(path),
                _ => Err(anyhow::anyhow!(
                    "Unsupported config file format. Supported formats: .yml, .yaml, .toml, pyproject.toml"
                )),
            }
        }
    }

    /// Load and parse the configuration from a YAML file
    /// # Errors
    /// Returns an error if the file cannot be opened or if the YAML is invalid
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)
            .context(format!("Unable to open config file at {}", path.display()))?;

        let config: std::result::Result<Self, serde_yaml::Error> = serde_yaml::from_reader(file);

        match config {
            Ok(mut cfg) => {
                cfg.clean_config();
                cfg.validate()?;
                Ok(cfg)
            }
            Err(err) => Err(anyhow::anyhow!("Error parsing config file: {err}")),
        }
    }

    /// Load and parse the configuration from a TOML file
    /// Uses standard TOML array-of-tables syntax: `[[manifest_tests]]`
    /// # Errors
    /// Returns an error if the file cannot be opened or if the TOML is invalid
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let contents = std::fs::read_to_string(path)
            .context(format!("Unable to read config file at {}", path.display()))?;

        let config: std::result::Result<Self, toml::de::Error> = toml::from_str(&contents);

        match config {
            Ok(mut cfg) => {
                cfg.clean_config();
                cfg.validate()?;
                Ok(cfg)
            }
            Err(err) => Err(anyhow::anyhow!("Error parsing TOML config file: {err}")),
        }
    }

    /// Load and parse the configuration from a pyproject.toml file
    /// This expects the configuration to be under the `[tool.dbtective]` section
    /// # Errors
    /// Returns an error if the file cannot be opened, if the TOML is invalid,
    /// or if the `[tool.dbtective]` section is missing
    pub fn from_pyproject<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let contents = std::fs::read_to_string(path).context(format!(
            "Unable to read pyproject.toml file at {}",
            path.display()
        ))?;

        let pyproject: std::result::Result<PyProject, toml::de::Error> = toml::from_str(&contents);

        match pyproject {
            Ok(mut pyproj) => {
                pyproj.tool.dbtective.clean_config();
                pyproj.tool.dbtective.validate()?;
                Ok(pyproj.tool.dbtective)
            }
            Err(err) => Err(anyhow::anyhow!(
                "Error parsing pyproject.toml config file. Make sure the configuration is under [tool.dbtective]: {err}"
            )),
        }
    }

    // 1. Apply default applies_to if not specified
    // 2. Normalize the includes/excludes paths
    pub fn clean_config(&mut self) {
        if let Some(rules) = &mut self.manifest_tests {
            for rule in rules {
                if rule.applies_to.is_none() {
                    rule.applies_to = Some(default_applies_to_for_manifest_rule(&rule.rule));
                }
                rule.normalize_includes_excludes();
            }
        }
        if let Some(rules) = &mut self.catalog_tests {
            for rule in rules {
                if rule.applies_to.is_none() {
                    rule.applies_to = Some(default_applies_to_for_catalog_rule(&rule.rule));
                }
                rule.normalize_includes_excludes();
            }
        }
    }
    // Validate each manifest rule's applies_to targets
    //  # Errors
    // Returns an error if any rule has invalid `applies_to` targets for that specific rule
    ///
    /// # Errors
    /// Returns an error if any rule has invalid `applies_to` targets for that specific rule
    pub fn validate(&self) -> Result<()> {
        if let Some(rules) = &self.manifest_tests {
            for rule in rules {
                rule.validate_applies_to()?;
            }
        }
        if let Some(rules) = &self.catalog_tests {
            for rule in rules {
                rule.validate_applies_to()?;
            }
        }
        Ok(())
    }
}

pub fn resolve_config_path(entry_point: &str, config_file: Option<&String>) -> String {
    // Early return on specified config file in --config-file flag
    if let Some(config_file) = config_file {
        let explicit_config_path = format!("{entry_point}/{config_file}");
        if std::path::Path::new(&explicit_config_path).exists() {
            return explicit_config_path;
        }
    }

    // Auto-detect config file
    let (config_file, all_found) = unwrap_or_exit(Config::find_config_in_dir(entry_point));

    if all_found.len() > 1 {
        eprintln!(
            "{} {}",
            "Warning:".yellow().bold(),
            format!(
                "Found multiple dbtective config files: '{}'. Using '{}' according to priority (yml > toml > pyproject.toml).\n\
To use a specific config file, provide the '--config-file' flag.",
                all_found.join(", "),
                config_file
            )
            .yellow()
        );
    }

    format!("{entry_point}/{config_file}")
}
