//! Parser for dbt manifest and catalog JSON artifacts
//!
//! This crate provides data structures and parsing logic for dbt's
//! `manifest.json` and `catalog.json` files.

pub mod catalog;
pub mod manifest;

// Re-export commonly used types at crate root
pub use catalog::{Catalog, CatalogMetadata, CatalogNode, CatalogSource};
pub use manifest::{Manifest, ManifestMetadata, Node, Source};
