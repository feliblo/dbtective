use std::path::PathBuf;

use dbtective::core::rules::common_traits::Identifiable;
use dbtective::core::{
    catalog::{Catalog, CatalogNode},
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

#[test]
fn test_patch_path_parsed_from_manifest() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    // stg_customers has a patch_path defined in the manifest
    let node = manifest
        .nodes
        .get("model.dbtective_test_project.stg_customers")
        .expect("Node not found");

    // Verify patch_path is parsed and stripped correctly
    let patch_path = node.get_patch_path();
    assert!(patch_path.is_some());
    assert_eq!(
        patch_path.unwrap(),
        "models/staging/crm/_stg_crm__models.yml"
    );

    // Verify original_file_path is still accessible
    assert_eq!(
        node.get_relative_path(),
        "models/staging/crm/stg_customers.sql"
    );
}

#[test]
fn test_identifiable_prefers_patch_path() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    // stg_customers has a patch_path - Identifiable should return that
    let node = manifest
        .nodes
        .get("model.dbtective_test_project.stg_customers")
        .expect("Node not found");

    let identifiable_path = Identifiable::get_relative_path(node);
    assert!(identifiable_path.is_some());
    // Should return the YAML path, not the SQL path
    assert_eq!(
        identifiable_path.unwrap(),
        "models/staging/crm/_stg_crm__models.yml"
    );
}

#[test]
fn test_identifiable_falls_back_to_original_path() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    // Seeds typically don't have patch_path
    let seed = manifest
        .nodes
        .get("seed.dbtective_test_project.raw_customers")
        .expect("Seed not found");

    // Verify patch_path is None
    assert!(seed.get_patch_path().is_none());

    // Identifiable should fall back to original_file_path
    let identifiable_path = Identifiable::get_relative_path(seed);
    assert!(identifiable_path.is_some());
    assert_eq!(identifiable_path.unwrap(), "seeds/raw_customers.csv");
}

#[test]
fn test_multiple_models_have_correct_patch_paths() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    // Test multiple models with different patch_paths
    let test_cases = vec![
        (
            "model.dbtective_test_project.stg_customers",
            Some("models/staging/crm/_stg_crm__models.yml"),
        ),
        (
            "model.dbtective_test_project.stg_payments",
            Some("models/staging/payments/_stg_payments__models.yml"),
        ),
        (
            "model.dbtective_test_project.orders",
            Some("models/marts/finance/_finance__models.yml"),
        ),
    ];

    for (node_id, expected_patch_path) in test_cases {
        let node = manifest.nodes.get(node_id).expect("Node not found");
        assert_eq!(
            node.get_patch_path(),
            expected_patch_path,
            "Mismatch for {node_id}",
        );
    }
}
