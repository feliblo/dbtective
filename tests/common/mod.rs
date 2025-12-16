#![allow(dead_code)]

use dbtective::cli::table::{show_results_and_exit, RuleResult};
use dbtective::core::catalog::parse_catalog::Catalog;
use dbtective::core::checks::catalog::catalog_node_checks::apply_catalog_node_checks;
use dbtective::core::checks::catalog::catalog_source_checks::apply_catalog_source_checks;
use dbtective::core::checks::manifest::node_checks::apply_node_checks;
use dbtective::core::checks::manifest::other_manifest_object_checks::apply_manifest_object_checks;
use dbtective::core::config::severity::Severity;
use dbtective::core::config::Config;
use dbtective::core::manifest::Manifest;
use std::io::Write;
use tempfile::TempDir;

pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub manifest_path: std::path::PathBuf,
    pub config_path: std::path::PathBuf,
    pub catalog_path: Option<std::path::PathBuf>,
}

impl TestEnvironment {
    pub fn new(manifest_json: &str, config_yaml: &str) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        // Write manifest.json
        let manifest_path = temp_path.join("manifest.json");
        let mut manifest_file =
            std::fs::File::create(&manifest_path).expect("Failed to create manifest file");
        manifest_file
            .write_all(manifest_json.as_bytes())
            .expect("Failed to write manifest");

        // Write config.yml
        let config_path = temp_path.join("config.yml");
        let mut config_file =
            std::fs::File::create(&config_path).expect("Failed to create config file");
        config_file
            .write_all(config_yaml.as_bytes())
            .expect("Failed to write config");

        Self {
            temp_dir,
            manifest_path,
            config_path,
            catalog_path: None,
        }
    }

    pub fn new_with_catalog(manifest_json: &str, catalog_json: &str, config_yaml: &str) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        // Write manifest.json
        let manifest_path = temp_path.join("manifest.json");
        let mut manifest_file =
            std::fs::File::create(&manifest_path).expect("Failed to create manifest file");
        manifest_file
            .write_all(manifest_json.as_bytes())
            .expect("Failed to write manifest");

        // Write catalog.json
        let catalog_path = temp_path.join("catalog.json");
        let mut catalog_file =
            std::fs::File::create(&catalog_path).expect("Failed to create catalog file");
        catalog_file
            .write_all(catalog_json.as_bytes())
            .expect("Failed to write catalog");

        // Write config.yml
        let config_path = temp_path.join("config.yml");
        let mut config_file =
            std::fs::File::create(&config_path).expect("Failed to create config file");
        config_file
            .write_all(config_yaml.as_bytes())
            .expect("Failed to write config");

        Self {
            temp_dir,
            manifest_path,
            config_path,
            catalog_path: Some(catalog_path),
        }
    }

    pub fn run_checks(&self, verbose: bool) -> Vec<(RuleResult, Severity)> {
        let manifest = Manifest::from_file(&self.manifest_path).expect("Failed to load manifest");
        let config = Config::from_file(&self.config_path).expect("Failed to load config");

        let mut findings =
            apply_node_checks(&manifest, &config, verbose).expect("Failed to apply node checks");
        findings.extend(
            apply_manifest_object_checks(&manifest, &config, verbose)
                .expect("Failed to apply source checks"),
        );

        // Convert from Vec<(RuleResult, &Severity)> to Vec<(RuleResult, Severity)>
        findings
            .into_iter()
            .map(|(result, severity)| (result, severity.clone()))
            .collect()
    }

    pub fn run_catalog_checks(&self, verbose: bool) -> anyhow::Result<Vec<(RuleResult, Severity)>> {
        let manifest = Manifest::from_file(&self.manifest_path)?;
        let config = Config::from_file(&self.config_path)?;
        let catalog = self
            .catalog_path
            .as_ref()
            .map(Catalog::from_file)
            .transpose()?;

        let mut findings = Vec::new();

        if let Some(ref catalog) = catalog {
            findings.extend(apply_catalog_node_checks(
                &config, catalog, &manifest, verbose,
            )?);
            findings.extend(apply_catalog_source_checks(
                &config, catalog, &manifest, verbose,
            )?);
        }

        Ok(findings
            .into_iter()
            .map(|(result, severity)| (result, severity.clone()))
            .collect())
    }

    pub fn run_and_show_results(&self, verbose: bool) -> i32 {
        let manifest = Manifest::from_file(&self.manifest_path).expect("Failed to load manifest");
        let config = Config::from_file(&self.config_path).expect("Failed to load config");

        let mut findings =
            apply_node_checks(&manifest, &config, verbose).expect("Failed to apply node checks");
        findings.extend(
            apply_manifest_object_checks(&manifest, &config, verbose)
                .expect("Failed to apply source checks"),
        );

        show_results_and_exit(
            &findings,
            verbose,
            self.temp_dir.path().to_str().unwrap(),
            false,
            None,
        )
    }
}
