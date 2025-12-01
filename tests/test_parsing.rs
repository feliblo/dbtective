use std::path::PathBuf;

use dbtective::core::{
    catalog::{nodes::CatalogNode, parse_catalog::Catalog},
    manifest::Manifest,
};

#[test]
fn test_load_example_manifest() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");
    assert_eq!(manifest.metadata.dbt_version, "1.10.2");
}

#[test]
fn test_parse_example_catalog() {
    let catalog_path = PathBuf::from("dbt_project/target/catalog.json");
    let catalog = Catalog::from_file(&catalog_path).expect("Failed to parse catalog");
    println!("{:#?}", catalog.nodes.values().next());
    assert_eq!(catalog.metadata.dbt_version, "1.10.2");
    assert_eq!(catalog.sources.len(), 0);
    assert!(matches!(
        catalog
            .nodes
            .get("model.dbtective_test_project.metricflow_time_spine")
            .unwrap(),
        CatalogNode::Model { .. }
    ));
    assert!(matches!(
        catalog
            .nodes
            .get("snapshot.dbtective_test_project.snapshot_orders_multiple_unique_keys")
            .unwrap(),
        CatalogNode::Snapshot { .. }
    ));
}
