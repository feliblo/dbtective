// Re-export types from dbt_artifact_parser
#[allow(unused_imports)]
pub use dbt_artifact_parser::manifest::{
    Exposure, Group, Macro, Manifest, Node, SavedQuery, SemanticModel, Source, UnitTest,
};

// Re-export dbt_objects for backward compatibility
#[allow(unused_imports)]
pub mod dbt_objects {
    pub use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
}

// Re-export parse_manifest for backward compatibility
#[allow(unused_imports)]
pub mod parse_manifest {
    pub use dbt_artifact_parser::manifest::parse_manifest::check_manifest_version;
    pub use dbt_artifact_parser::manifest::{Manifest, ManifestMetadata, Quoting};
}

// Trait implementation modules (these stay in dbtective)
mod exposure_impls;
mod macro_impls;
mod node_impls;
mod semantic_model_impls;
mod source_impls;
mod unit_test_impls;
