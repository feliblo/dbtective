// Re-export types from dbt_artifact_parser
#[allow(unused_imports)]
pub use dbt_artifact_parser::catalog::{Catalog, CatalogNode, CatalogSource};

// Trait implementations for catalog types
mod node_impls;
mod source_impls;
