use super::exposure::Exposure;
use super::group::Group;
use super::macro_obj::Macro;
use super::metric::Metric;
use super::nodes::node::Node;
use super::nodes::test::Test;
use super::saved_query::SavedQuery;
use super::semantic_model::SemanticModel;
use super::source::Source;
use super::udf::UDF;
use super::unit_test::UnitTest;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

enum AllowedManifestVersions {
    V12, // v12 and v20 are identical
}

// https://docs.getdbt.com/reference/artifacts/manifest-json
impl AllowedManifestVersions {
    fn from_str(version: &str) -> Option<Self> {
        match version {
            "https://schemas.getdbt.com/dbt/manifest/v12.json"
            | "https://schemas.getdbt.com/dbt/manifest/v20.json" => Some(Self::V12),
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

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
// Valid manifest according to manifest version v12
pub struct Manifest {
    #[serde(default)]
    pub metadata: ManifestMetadata,
    #[serde(default)]
    pub nodes: HashMap<String, Node>,
    #[serde(default)]
    pub sources: HashMap<String, Source>,
    #[serde(default)]
    pub macros: HashMap<String, Macro>,
    #[serde(default)]
    pub exposures: HashMap<String, Exposure>,
    #[serde(default)]
    pub metrics: HashMap<String, Metric>,
    #[serde(default)]
    pub groups: HashMap<String, Group>,
    #[serde(default)]
    pub parent_map: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub child_map: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub group_map: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub saved_queries: HashMap<String, SavedQuery>,
    #[serde(default)]
    pub semantic_models: HashMap<String, SemanticModel>,
    #[serde(default)]
    pub unit_tests: HashMap<String, UnitTest>,
    #[serde(default)]
    pub functions: HashMap<String, UDF>,
}

impl Manifest {
    /// Get a node by its `unique_id` (Required by Catalog tests)
    pub fn get_node(&self, unique_id: &str) -> Option<&Node> {
        self.nodes.get(unique_id)
    }

    /// Get a source by its `unique_id` (Required by Catalog tests)
    #[allow(dead_code)]
    pub fn get_source(&self, unique_id: &str) -> Option<&Source> {
        self.sources.get(unique_id)
    }

    // Get tests attached to a specific parent node or source
    // For nodes (models, seeds, etc.), the parent is in `attached_node`.
    // For sources, `attached_node` is null and the parent is in `depends_on.nodes`.
    pub fn get_tests_by_parent(&self, parent_unique_id: &str) -> Vec<&Test> {
        self.nodes
            .iter()
            .filter_map(|(_, node)| {
                if let Node::Test(test) = node {
                    // Check attached_node first (set for tests on nodes like models)
                    if let Some(attached_node) = &test.attached_node {
                        if attached_node == parent_unique_id {
                            return Some(test);
                        }
                    } else if let Some(dep_nodes) = &test.base.depends_on.nodes {
                        // Fallback to depends_on.nodes (used for tests on sources)
                        if dep_nodes.iter().any(|n| n == parent_unique_id) {
                            return Some(test);
                        }
                    }
                }
                None
            })
            .collect()
    }

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

        let mut de = serde_json::Deserializer::from_reader(reader);

        let mut manifest: Self = serde_path_to_error::deserialize(&mut de)
            .inspect_err(|e| {
                println!("{}", e.path());
            })
            .context(format!(
                "Unable to parse manifest JSON, delete it from {} and regenerate using eligible dbt commands.\n\
                See: \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\dbt manifest documentation\x1b]8;;\x1b\\",
                manifest_path.display()
            ))?;

        check_manifest_version(&manifest.metadata.dbt_schema_version)?;

        // Filter all objects to only include those from the current project
        if let Some(project_name) = manifest.metadata.project_name.as_ref() {
            manifest
                .nodes
                .retain(|_, node| node.get_package_name() == project_name.as_str());
            manifest
                .sources
                .retain(|_, source| source.get_package_name() == project_name.as_str());
            manifest
                .macros
                .retain(|_, macro_obj| macro_obj.get_package_name() == project_name.as_str());
            manifest
                .exposures
                .retain(|_, exposure| exposure.get_package_name() == project_name.as_str());

            manifest
                .semantic_models
                .retain(|_, sm| sm.get_package_name() == project_name.as_str());
            manifest
                .unit_tests
                .retain(|_, ut| ut.get_package_name() == project_name.as_str());
            manifest
                .functions
                .retain(|_, udf| udf.package_name == project_name.as_str());
        }

        Ok(manifest)
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct ManifestMetadata {
    #[serde(default)]
    pub dbt_schema_version: String,
    #[serde(default)]
    pub dbt_version: String,
    #[serde(default)]
    pub generated_at: String,
    pub invocation_id: Option<String>,
    pub invocation_started_at: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    pub project_name: Option<String>,
    pub project_id: Option<String>,
    pub user_id: Option<String>,
    pub send_anonymous_usage_stats: Option<bool>,
    pub adapter_type: Option<String>,
    #[serde(default)]
    pub quoting: Quoting,
}

#[derive(Debug, Deserialize, Default)]
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
    use crate::manifest::nodes::node::DependsOn;
    use crate::manifest::nodes::test::TestMetadata;
    use std::path::PathBuf;

    fn create_test_with_attached_node(unique_id: &str, name: &str, attached_node: &str) -> Test {
        let mut test = Test::default();
        test.base.unique_id = unique_id.to_string();
        test.base.name = name.to_string();
        test.attached_node = Some(attached_node.to_string());
        test.test_metadata = Some(TestMetadata {
            name: name.to_string(),
            kwargs: None,
            namespace: None,
        });
        test
    }

    fn create_test_with_depends_on(
        unique_id: &str,
        name: &str,
        depends_on_nodes: Vec<String>,
    ) -> Test {
        let mut test = Test::default();
        test.base.unique_id = unique_id.to_string();
        test.base.name = name.to_string();
        test.attached_node = None;
        test.base.depends_on = DependsOn {
            nodes: Some(depends_on_nodes),
            macros: None,
        };
        test.test_metadata = Some(TestMetadata {
            name: name.to_string(),
            kwargs: None,
            namespace: None,
        });
        test
    }

    fn manifest_with_tests(tests: Vec<Test>) -> Manifest {
        let mut manifest = Manifest::default();
        for test in tests {
            manifest
                .nodes
                .insert(test.base.unique_id.clone(), Node::Test(test));
        }
        manifest
    }

    #[test]
    fn test_get_tests_by_parent_via_attached_node() {
        let manifest = manifest_with_tests(vec![create_test_with_attached_node(
            "test.unique_id",
            "unique",
            "model.my_model",
        )]);
        let tests = manifest.get_tests_by_parent("model.my_model");
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].base.name, "unique");
    }

    #[test]
    fn test_get_tests_by_parent_via_depends_on_for_sources() {
        let manifest = manifest_with_tests(vec![create_test_with_depends_on(
            "test.source_unique",
            "unique",
            vec!["source.my_project.raw.my_table".to_string()],
        )]);
        let tests = manifest.get_tests_by_parent("source.my_project.raw.my_table");
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].base.name, "unique");
    }

    #[test]
    fn test_get_tests_by_parent_no_match() {
        let manifest = manifest_with_tests(vec![create_test_with_attached_node(
            "test.unique_id",
            "unique",
            "model.other_model",
        )]);
        let tests = manifest.get_tests_by_parent("model.my_model");
        assert!(tests.is_empty());
    }

    #[test]
    fn test_get_tests_by_parent_no_match_depends_on() {
        let manifest = manifest_with_tests(vec![create_test_with_depends_on(
            "test.source_unique",
            "unique",
            vec!["source.my_project.raw.other_table".to_string()],
        )]);
        let tests = manifest.get_tests_by_parent("source.my_project.raw.my_table");
        assert!(tests.is_empty());
    }

    #[test]
    fn test_get_tests_by_parent_mixed_attached_and_depends_on() {
        let manifest = manifest_with_tests(vec![
            create_test_with_attached_node("test.node_unique", "unique", "model.my_model"),
            create_test_with_depends_on(
                "test.source_unique",
                "unique",
                vec!["source.my_project.raw.my_table".to_string()],
            ),
        ]);
        // Node test should match
        let node_tests = manifest.get_tests_by_parent("model.my_model");
        assert_eq!(node_tests.len(), 1);
        assert_eq!(node_tests[0].base.unique_id, "test.node_unique");

        // Source test should match
        let source_tests = manifest.get_tests_by_parent("source.my_project.raw.my_table");
        assert_eq!(source_tests.len(), 1);
        assert_eq!(source_tests[0].base.unique_id, "test.source_unique");
    }

    #[test]
    fn test_get_tests_by_parent_prefers_attached_node_over_depends_on() {
        // When attached_node is set, it should match on that and not fall through to depends_on
        let mut test = create_test_with_attached_node("test.t1", "unique", "model.my_model");
        // Also set depends_on to a different parent
        test.base.depends_on = DependsOn {
            nodes: Some(vec!["source.some_source".to_string()]),
            macros: None,
        };
        let manifest = manifest_with_tests(vec![test]);

        // Should match via attached_node
        let tests = manifest.get_tests_by_parent("model.my_model");
        assert_eq!(tests.len(), 1);

        // Should NOT match via depends_on since attached_node is set
        let tests = manifest.get_tests_by_parent("source.some_source");
        assert!(tests.is_empty());
    }

    #[test]
    fn test_get_tests_by_parent_depends_on_empty_nodes() {
        let manifest = manifest_with_tests(vec![create_test_with_depends_on(
            "test.empty",
            "unique",
            vec![],
        )]);
        let tests = manifest.get_tests_by_parent("source.my_project.raw.my_table");
        assert!(tests.is_empty());
    }

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
