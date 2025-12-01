use super::dbt_objects::{Node, Source};
use super::{Exposure, Group, Macro, Metric, SavedQuery, SemanticModel, UnitTest};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

enum AllowedManifestVersions {
    V12,
}
impl AllowedManifestVersions {
    fn from_str(version: &str) -> Option<Self> {
        match version {
            "https://schemas.getdbt.com/dbt/manifest/v12.json" => Some(Self::V12),
            _ => None,
        }
    }
}

// Check if the manifest version is supported
/// Returns Ok(true) if supported, Err otherwise
/// # Errors
/// Returns an error if the manifest version is not supported
pub fn check_manifest_version(dbt_schema_version: &str) -> Result<bool> {
    match AllowedManifestVersions::from_str(dbt_schema_version) {
        Some(_) => Ok(true),
        None => anyhow::bail!(
            "Unsupported manifest schema version: {dbt_schema_version}, expected version 12. Please regenerate the manifest using 'dbt run' with dbt version 1.10.0 or higher see: \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\dbt manifest documentation\x1b]8;;\x1b\\."
        ),
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
// Valid manifest according to manifest version v12
pub struct Manifest {
    pub metadata: ManifestMetadata,
    pub nodes: HashMap<String, Node>,
    pub sources: HashMap<String, Source>,
    pub macros: HashMap<String, Macro>,
    // pub docs: HashMap<String, Documentation>,
    pub exposures: HashMap<String, Exposure>,
    pub metrics: HashMap<String, Metric>,
    pub groups: HashMap<String, Group>,
    // pub selectors: HashMap<String, Selector>,
    // pub disabled: HashMap<String, Vec<DisabledResource>>,
    pub parent_map: HashMap<String, Vec<String>>,
    pub child_map: HashMap<String, Vec<String>>,
    pub group_map: HashMap<String, Vec<String>>,
    pub saved_queries: HashMap<String, SavedQuery>,
    pub semantic_models: HashMap<String, SemanticModel>,
    pub unit_tests: HashMap<String, UnitTest>,
}

impl Manifest {
    /// Reads a manifest from a file and parses it into a `Manifest`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist or cannot be opened.
    /// - The file contents cannot be read as UTF-8.
    /// - The manifest format is invalid.
    pub fn from_file<P: AsRef<Path>>(manifest_path: P) -> Result<Self> {
        let manifest_path = manifest_path.as_ref();

        let file = File::open(manifest_path).context(format!(
            "Unable to open manifest file at {}",
            manifest_path.display()
        ))?;

        let reader = BufReader::new(file);

        let mut manifest: Self = serde_json::from_reader(reader).context(format!(
            "Unable to parse manifest JSON, delete it from {} and regenerate using 'dbt run'\nSee: \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\dbt manifest documentation\x1b]8;;\x1b\\",
            manifest_path.display()
        ))?;

        check_manifest_version(&manifest.metadata.dbt_schema_version)?;

        // Filter nodes to only those with package_name == project_name
        if let Some(project_name) = manifest.metadata.project_name.as_ref() {
            manifest
                .nodes
                .retain(|_, node| node.get_package_name() == project_name.as_str());
            manifest
                .sources
                .retain(|_, source| source.get_package_name() == project_name.as_str());
        }

        Ok(manifest)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ManifestMetadata {
    pub dbt_schema_version: String,
    pub dbt_version: String,
    pub generated_at: String,
    pub invocation_id: Option<String>,
    pub invocation_started_at: Option<String>,
    pub env: HashMap<String, String>,
    pub project_name: Option<String>,
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub send_anonymous_usage_stats: Option<bool>,
    pub adapter_type: Option<String>,
    pub quoting: Quoting,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Quoting {
    pub database: Option<bool>,
    pub schema: Option<bool>,
    pub identifier: Option<bool>,
    pub column: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_manifest_invalid_path() {
        let manifest_path = PathBuf::from("invalid/path/manifest.json");
        let result = Manifest::from_file(&manifest_path);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Unable to open manifest"));
    }

    #[test]
    fn test_invalid_manifest_version() {
        let invalid_version = "https://schemas.getdbt.com/dbt/manifest/v10.json";
        let result = check_manifest_version(invalid_version);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Unsupported manifest schema version"));
    }
}
