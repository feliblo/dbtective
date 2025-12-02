// Documentation: https://docs.getdbt.com/reference/artifacts/catalog-json
// Catalog is produced by running dbt docs generate
// Currently only version 1 is supported

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::core::catalog::nodes::CatalogNode;
use crate::core::catalog::source::CatalogSource;

enum AllowedCatalogVersions {
    V1,
}

impl AllowedCatalogVersions {
    fn from_str(version: &str) -> Option<Self> {
        match version {
            "https://schemas.getdbt.com/dbt/catalog/v1.json" => Some(Self::V1),
            _ => None,
        }
    }
}

/// Check if the catalog version is supported
/// Returns Ok(true) if supported, Err otherwise
/// # Errors
/// Returns an error if the catalog version is not supported
pub fn check_catalog_version(dbt_schema_version: &str) -> Result<bool> {
    match AllowedCatalogVersions::from_str(dbt_schema_version) {
        Some(_) => Ok(true),
        None => anyhow::bail!(
            "Unsupported catalog schema version: {dbt_schema_version}, expected version 1. Please regenerate the catalog using 'dbt docs generate' see: \x1b]8;;https://docs.getdbt.com/reference/artifacts/catalog-json\x1b\\dbt catalog documentation\x1b]8;;\x1b\\."
        ),
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Catalog {
    pub metadata: CatalogMetadata,
    pub nodes: HashMap<String, CatalogNode>,
    pub sources: HashMap<String, CatalogSource>,
    pub errors: Option<serde_json::Value>,
}

impl Catalog {
    #[allow(dead_code)]
    /// Load and parse a dbt catalog JSON file from the given path
    /// # Errors
    /// Returns an error if the file cannot be opened or parsed
    pub fn from_file<P: AsRef<Path>>(catalog_path: P) -> Result<Self> {
        let catalog_path = catalog_path.as_ref();

        let file = File::open(catalog_path).context(format!(
            "Unable to open catalog file at {}",
            catalog_path.display()
        ))?;

        let reader = BufReader::new(file);

        let catalog: Self = serde_json::from_reader(reader).context(format!(
            "Unable to parse catalog JSON, delete it from {} and regenerate using 'dbt docs generate'\nSee: \x1b]8;;https://docs.getdbt.com/reference/artifacts/catalog-json\x1b\\dbt catalog documentation\x1b]8;;\x1b\\",
            catalog_path.display()
        ))?;

        check_catalog_version(&catalog.metadata.dbt_schema_version)?;

        Ok(catalog)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogMetadata {
    pub dbt_schema_version: String,
    pub dbt_version: String,
    pub generated_at: String,
    pub invocation_id: Option<String>,
    pub invocation_started_at: Option<String>,
    pub env: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_catalog_version_supported() {
        let version = "https://schemas.getdbt.com/dbt/catalog/v1.json";
        let result = check_catalog_version(version);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_catalog_version_unsupported() {
        let version = "https://schemas.getdbt.com/dbt/catalog/v2.json";
        let result = check_catalog_version(version);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_empty_catalog() {
        let json_str = r#"
        {
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
                "dbt_version": "1.0.0",
                "generated_at": "2023-01-01T00:00:00Z",
                "invocation_id": null,
                "invocation_started_at": null,
                "env": {}
            },
            "nodes": {},
            "sources": {},
            "errors": null
        }
        "#;

        let catalog: Catalog = serde_json::from_str(json_str).unwrap();
        assert_eq!(
            catalog.metadata.dbt_schema_version,
            "https://schemas.getdbt.com/dbt/catalog/v1.json"
        );
        assert_eq!(catalog.metadata.dbt_version, "1.0.0");
        assert_eq!(catalog.nodes.len(), 0);
        assert_eq!(catalog.sources.len(), 0);
    }

    #[test]
    fn test_deserialize_catalog_with_nodes() {
        let json_str = r#"
        {
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
                "dbt_version": "1.0.0",
                "generated_at": "2023-01-01T00:00:00Z",
                "invocation_id": null,
                "invocation_started_at": null,
                "env": {}
            },
            "nodes": {
                "model.dbtective_test_project.model_orders": {
                "metadata": {
                    "type": "BASE TABLE",
                    "schema": "dbt_dbtective",
                    "name": "model_orders",
                    "database": "dbt",
                    "comment": null,
                    "owner": null
                },
                "columns": {
                    "id": { "type": "INTEGER", "index": 1, "name": "id", "comment": null },
                    "user_id": {
                    "type": "INTEGER",
                    "index": 2,
                    "name": "user_id",
                    "comment": null
                    },
                    "order_date": {
                    "type": "DATE",
                    "index": 3,
                    "name": "order_date",
                    "comment": null
                    },
                    "status": {
                    "type": "VARCHAR",
                    "index": 4,
                    "name": "status",
                    "comment": null
                    },
                    "dbt_scd_id": {
                    "type": "VARCHAR",
                    "index": 5,
                    "name": "dbt_scd_id",
                    "comment": null
                    },
                    "dbt_updated_at": {
                    "type": "DATE",
                    "index": 6,
                    "name": "dbt_updated_at",
                    "comment": null
                    },
                    "dbt_valid_from": {
                    "type": "DATE",
                    "index": 7,
                    "name": "dbt_valid_from",
                    "comment": null
                    },
                    "dbt_valid_to": {
                    "type": "DATE",
                    "index": 8,
                    "name": "dbt_valid_to",
                    "comment": null
                    }
                },
                "stats": {
                    "has_stats": {
                    "id": "has_stats",
                    "label": "Has Stats?",
                    "value": false,
                    "include": false,
                    "description": "Indicates whether there are statistics for this table"
                    }
                },
                "unique_id": "model.dbtective_test_project.model_orders"
                }
            },
            "sources": {},
            "errors": null
        }
        "#;

        let catalog: Catalog = serde_json::from_str(json_str).unwrap();
        assert_eq!(
            catalog.metadata.dbt_schema_version,
            "https://schemas.getdbt.com/dbt/catalog/v1.json"
        );
        assert_eq!(catalog.metadata.dbt_version, "1.0.0");
        assert_eq!(catalog.nodes.len(), 1);
        assert_eq!(catalog.sources.len(), 0);
        assert!(matches!(
            catalog
                .nodes
                .get("model.dbtective_test_project.model_orders")
                .unwrap(),
            CatalogNode::Model { .. }
        ));
    }

    #[test]
    fn test_deserialize_catalog_with_sources() {
        let json_str = r#"
        {
            "metadata": {
                "dbt_schema_version": "https://schemas.getdbt.com/dbt/catalog/v1.json",
                "dbt_version": "1.0.0",
                "generated_at": "2023-01-01T00:00:00Z",
                "invocation_id": null,
                "invocation_started_at": null,
                "env": {}
            },
            "nodes": {},
            "sources": {
                "source.dbtective_test_project.raw_customers": {
                    "metadata": {
                        "type": "BASE TABLE",
                        "schema": "dbt_dbtective",
                        "name": "raw_customers",
                        "database": "dbt",
                        "comment": null,
                        "owner": null
                    },
                    "columns": {},
                    "stats": {},
                    "unique_id": "source.dbtective_test_project.raw_customers"
                }
            },
            "errors": null
        }
        "#;

        let catalog: Catalog = serde_json::from_str(json_str).unwrap();
        assert_eq!(
            catalog.metadata.dbt_schema_version,
            "https://schemas.getdbt.com/dbt/catalog/v1.json"
        );
        assert_eq!(catalog.metadata.dbt_version, "1.0.0");
        assert_eq!(catalog.nodes.len(), 0);
        assert_eq!(catalog.sources.len(), 1);
        assert!(matches!(
            catalog
                .sources
                .get("source.dbtective_test_project.raw_customers")
                .unwrap(),
            CatalogSource { .. }
        ));
    }
}
