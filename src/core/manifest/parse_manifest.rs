use super::super::dbt_objects::{Node, Source};
use super::{Exposure, Group, Macro, Metric, SavedQuery, SemanticModel, UnitTest};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

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

#[allow(dead_code)]
pub fn load_manifest(manifest_path: &Path) -> Result<Manifest> {
    let file = File::open(manifest_path).context(format!(
        "Unable to open manifest, expected at {}",
        manifest_path.display()
    ))?;

    let reader = BufReader::new(file);
    let manifest: Manifest = serde_json::from_reader(reader).context(format!(
        "Unable to parse manifest JSON, delete it from {} and regenerate using 'dbt run'\nSee: \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\dbt manifest documentation\x1b]8;;\x1b\\",
        manifest_path.display()
    ))?;

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_manifest() {
        let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
        let manifest = load_manifest(&manifest_path).expect("Failed to parse manifest");
        println!("{:#?}", manifest.sources);
        assert_eq!(manifest.metadata.dbt_version, "1.10.2");
    }

    #[test]
    fn test_load_manifest_invalid_path() {
        let manifest_path = PathBuf::from("invalid/path/manifest.json");
        let result = load_manifest(&manifest_path);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("Unable to open manifest"));
    }

    #[test]
    fn test_deserialize_timing_comparison() {
        use std::time::Instant;

        let manifest_path = PathBuf::from("dbt_project/target/manifest.json");

        // Typed deserialization (to our Manifest struct)
        let start = Instant::now();
        let file = File::open(&manifest_path).unwrap();
        let reader = BufReader::new(file);
        let _manifest: Manifest = serde_json::from_reader(reader).unwrap();
        let typed_duration = start.elapsed();
        println!("Typed (Manifest struct): {typed_duration:?}");

        // Untyped deserialization (to serde_json::Value)
        let start = Instant::now();
        let file = File::open(&manifest_path).unwrap();
        let reader = BufReader::new(file);
        let _value: serde_json::Value = serde_json::from_reader(reader).unwrap();
        let untyped_duration = start.elapsed();
        println!("Untyped (serde_json::Value): {untyped_duration:?}");

        println!(
            "Difference: {:?}",
            typed_duration.saturating_sub(untyped_duration)
        );
    }
}
