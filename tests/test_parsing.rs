use std::path::PathBuf;

use dbt_artifact_parser::manifest::udf::{UDFType, Volatility};
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
fn test_patch_path_parsed_from_manifest_sql() {
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
        node.get_problematic_path(true),
        Some("models/staging/crm/stg_customers.sql")
    );
}

#[test]
fn test_patch_path_parsed_from_manifest_yml() {
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
        node.get_problematic_path(false),
        Some("models/staging/crm/_stg_crm__models.yml")
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

    let identifiable_path = node.get_problematic_path(false);
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
    let identifiable_path = seed.get_problematic_path(false);
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

#[test]
fn test_parse_functions_from_manifest() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");
    assert_eq!(manifest.functions.len(), 1);
    assert!(manifest
        .functions
        .contains_key("function.dbtective.is_positive_int"));
}

#[test]
fn test_function_basic_fields() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert_eq!(udf.name, "is_positive_int");
    assert_eq!(udf.package_name, "dbtective_test_project");
    assert_eq!(udf.unique_id, "function.dbtective.is_positive_int");
    assert_eq!(udf.schema, "public_udf_schema");
    assert_eq!(udf.database.as_deref(), Some("dbt"));
    assert_eq!(udf.path, "is_positive_int.sql");
    assert_eq!(udf.original_file_path, "functions/is_positive_int.sql");
    assert_eq!(udf.alias, "is_positive_int");
    assert_eq!(udf.language, "sql");
    assert_eq!(udf.fqn, vec!["dbtective", "is_positive_int"]);
}

#[test]
fn test_function_returns() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert_eq!(udf.returns.data_type, "integer");
    assert!(udf.returns.description.is_none());
}

#[test]
fn test_function_arguments() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert_eq!(udf.arguments.len(), 1);
    let arg = &udf.arguments[0];
    assert_eq!(arg.name, "a_string");
    assert_eq!(arg.data_type, "text");
    assert!(arg.description.is_some());
    assert!(arg.default_value.is_some());
}

#[test]
fn test_function_config() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    let config = &udf.config;
    assert!(config.enabled);
    assert_eq!(config.schema.as_deref(), Some("udf_schema"));
    assert_eq!(config.database.as_deref(), Some("dbt"));
    assert_eq!(config.materialized, "function");
    assert!(config.full_refresh.is_none());
    assert!(matches!(config.udf_type, Some(UDFType::Scalar)));
    assert!(matches!(config.volatility, Some(Volatility::Deterministic)));
    assert!(config.runtime_version.is_none());
    assert!(config.entry_point.is_none());
}

#[test]
fn test_function_patch_path() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert_eq!(
        udf.patch_path.as_deref(),
        Some("dbtective://functions/schema.yml")
    );
}

#[test]
fn test_function_description_and_raw_code() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert!(!udf.description.is_empty());
    assert_eq!(udf.raw_code, "SELECT REGEXP_INSTR(a_string, '^[0-9]+$')");
}

#[test]
fn test_function_checksum() {
    let manifest_path = PathBuf::from("dbt_project/target/manifest.json");
    let manifest = Manifest::from_file(&manifest_path).expect("Failed to parse manifest");

    let udf = manifest
        .functions
        .get("function.dbtective.is_positive_int")
        .expect("Function not found");

    assert_eq!(udf.checksum.name, "sha256");
    assert!(!udf.checksum.checksum.is_empty());
}
