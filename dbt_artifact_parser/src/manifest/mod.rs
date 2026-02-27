pub mod dbt_objects;
pub mod exposure;
pub mod group;
pub mod macro_obj;
pub mod materialization;
pub mod metric;
pub mod nodes;
pub mod parse_manifest;
pub mod saved_query;
pub mod semantic_model;
pub mod source;
pub mod udf;
pub mod unit_test;

pub use dbt_objects::{Column, Meta, NodeDocs, Tags};
pub use exposure::{Exposure, ExposureDependsOn};
pub use group::{Group, GroupOwner};
pub use macro_obj::Macro;
pub use materialization::Materialization;
pub use metric::Metric;
pub use nodes::Node;
pub use parse_manifest::{Manifest, ManifestMetadata, Quoting};
pub use saved_query::{SavedQuery, SavedQueryDependsOn};
pub use semantic_model::{SemanticModel, SemanticModelDependsOn};
pub use source::{Source, SourceFreshness};
pub use udf::UDF;
pub use unit_test::UnitTest;

/// Strips the project prefix from a `patch_path` (e.g., `project://path` -> "path")
pub fn strip_patch_path_prefix(patch_path: &str) -> Option<&str> {
    patch_path.find("://").map(|idx| &patch_path[idx + 3..])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_patch_path_prefix_valid() {
        let patch_path = "dbtective_test_project://models/staging/crm/_stg_crm__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some("models/staging/crm/_stg_crm__models.yml"));
    }

    #[test]
    fn test_strip_patch_path_prefix_different_project() {
        let patch_path = "my_project://models/marts/finance/_finance__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some("models/marts/finance/_finance__models.yml"));
    }

    #[test]
    fn test_strip_patch_path_prefix_no_prefix() {
        let patch_path = "models/staging/crm/_stg_crm__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, None);
    }

    #[test]
    fn test_strip_patch_path_prefix_empty_after_prefix() {
        let patch_path = "project://";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some(""));
    }

    #[test]
    fn test_strip_patch_path_prefix_windows_backslashes() {
        // Windows paths may use backslashes
        let patch_path = r"dbtective_test_project://models\staging\crm\_stg_crm__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some(r"models\staging\crm\_stg_crm__models.yml"));
    }

    #[test]
    fn test_strip_patch_path_prefix_windows_mixed_slashes() {
        // Some tools produce mixed slashes
        let patch_path = r"my_project://models/staging\crm\_stg_crm__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some(r"models/staging\crm\_stg_crm__models.yml"));
    }

    #[test]
    fn test_strip_patch_path_prefix_windows_no_prefix_backslashes() {
        // Path without prefix but with Windows backslashes
        let patch_path = r"models\staging\crm\_stg_crm__models.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, None);
    }

    #[test]
    fn test_strip_patch_path_prefix_windows_dbt_real_example() {
        // Real Windows dbt output: forward slash after project, then backslashes
        let patch_path = r"stichtse_vecht_dbt://dbt/models\marts\mart_answered_calls.yml";
        let result = strip_patch_path_prefix(patch_path);
        assert_eq!(result, Some(r"dbt/models\marts\mart_answered_calls.yml"));
    }
}
