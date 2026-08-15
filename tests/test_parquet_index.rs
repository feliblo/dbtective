use dbt_artifact_parser::parquet::test_writer::{Cell, ColumnKind, IndexBuilder, TableBuilder};
use dbtective::core::manifest::{Manifest, Node};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use ColumnKind::{Bool, Int64, Utf8};

const PROJECT: &str = "my_project";

fn project_table() -> TableBuilder {
    TableBuilder::new(&[
        ("project_name", Utf8),
        ("adapter_type", Utf8),
        ("dbt_version", Utf8),
        ("quoting", Utf8),
    ])
    .row(&[
        Cell::Str(PROJECT),
        Cell::Str("duckdb"),
        Cell::Str("2.0.0-beta.1"),
        Cell::Str(r#"{"database":true,"schema":false}"#),
    ])
}

fn nodes_table() -> TableBuilder {
    TableBuilder::new(&[
        ("unique_id", Utf8),
        ("name", Utf8),
        ("resource_type", Utf8),
        ("package_name", Utf8),
        ("original_file_path", Utf8),
        ("schema_name", Utf8),
        ("materialized", Utf8),
        ("contract_enforced", Bool),
        ("description", Utf8),
        ("tags", Utf8),
        ("access_level", Utf8),
    ])
    .row(&[
        Cell::Str("model.my_project.orders"),
        Cell::Str("orders"),
        Cell::Str("model"),
        Cell::Str(PROJECT),
        Cell::Str("models/marts/orders.sql"),
        Cell::Str("marts"),
        Cell::Str("table"),
        Cell::Bool(true),
        Cell::Str("All orders"),
        Cell::Str(r#"["finance"]"#),
        Cell::Str("public"),
    ])
    .row(&[
        Cell::Str("model.my_project.stg_orders"),
        Cell::Str("stg_orders"),
        Cell::Str("model"),
        Cell::Str(PROJECT),
        Cell::Str("models/staging/stg_orders.sql"),
        Cell::Str("staging"),
        Cell::Str("view"),
    ])
    .row(&[
        Cell::Str("source.my_project.raw.orders"),
        Cell::Str("orders"),
        Cell::Str("source"),
        Cell::Str(PROJECT),
        Cell::Str("models/staging/_sources.yml"),
        Cell::Str("raw"),
    ])
    .row(&[
        Cell::Str("test.my_project.unique_orders_id"),
        Cell::Str("unique_orders_id"),
        Cell::Str("test"),
        Cell::Str(PROJECT),
        Cell::Str("models/marts/_models.yml"),
    ])
    .row(&[
        Cell::Str("model.other_package.vendored"),
        Cell::Str("vendored"),
        Cell::Str("model"),
        Cell::Str("other_package"),
        Cell::Str("models/vendored.sql"),
    ])
}

fn edges_table() -> TableBuilder {
    TableBuilder::new(&[
        ("parent_unique_id", Utf8),
        ("child_unique_id", Utf8),
        ("edge_type", Utf8),
    ])
    .row(&[
        Cell::Str("model.my_project.stg_orders"),
        Cell::Str("model.my_project.orders"),
        Cell::Str("ref"),
    ])
    .row(&[
        Cell::Str("source.my_project.raw.orders"),
        Cell::Str("model.my_project.stg_orders"),
        Cell::Str("source"),
    ])
    .row(&[
        Cell::Str("macro.dbt.some_macro"),
        Cell::Str("model.my_project.orders"),
        Cell::Str("macro"),
    ])
}

fn columns_table() -> TableBuilder {
    TableBuilder::new(&[
        ("unique_id", Utf8),
        ("column_name", Utf8),
        ("column_index", Int64),
        ("declared_type", Utf8),
        ("data_type", Utf8),
        ("description", Utf8),
    ])
    .row(&[
        Cell::Str("model.my_project.orders"),
        Cell::Str("order_id"),
        Cell::Int(0),
        Cell::Str("integer"),
        Cell::Str("INTEGER"),
        Cell::Str("Primary key"),
    ])
    .row(&[
        Cell::Str("model.my_project.orders"),
        Cell::Str("status"),
        Cell::Int(1),
        Cell::Null,
        Cell::Null,
        Cell::Str("Order status"),
    ])
}

fn test_metadata_table() -> TableBuilder {
    TableBuilder::new(&[
        ("unique_id", Utf8),
        ("test_name", Utf8),
        ("test_namespace", Utf8),
        ("column_name", Utf8),
        ("attached_node", Utf8),
    ])
    .row(&[
        Cell::Str("test.my_project.unique_orders_id"),
        Cell::Str("unique"),
        Cell::Null,
        Cell::Str("order_id"),
        Cell::Str("model.my_project.orders"),
    ])
}

fn write_full_index(dir: &Path) {
    IndexBuilder::new(dir)
        .unwrap()
        .table("dbt.project", project_table())
        .unwrap()
        .table("dbt.nodes", nodes_table())
        .unwrap()
        .table("dbt.edges", edges_table())
        .unwrap()
        .table("dbt.node_columns", columns_table())
        .unwrap()
        .table("dbt.test_metadata", test_metadata_table())
        .unwrap();
}

fn full_index() -> (TempDir, Manifest) {
    let dir = TempDir::new().unwrap();
    let index = dir.path().join("index");
    write_full_index(&index);
    let manifest = Manifest::from_index(&index).expect("failed to parse index");
    (dir, manifest)
}

#[test]
fn reads_project_metadata() {
    let (_dir, manifest) = full_index();
    assert_eq!(manifest.metadata.project_name.as_deref(), Some(PROJECT));
    assert_eq!(manifest.metadata.adapter_type.as_deref(), Some("duckdb"));
    assert_eq!(manifest.metadata.dbt_version, "2.0.0-beta.1");
    assert_eq!(manifest.metadata.quoting.database, Some(true));
    assert_eq!(manifest.metadata.quoting.schema, Some(false));
}

#[test]
fn splits_nodes_from_sources() {
    let (_dir, manifest) = full_index();
    assert!(manifest.nodes.contains_key("model.my_project.orders"));
    assert!(manifest
        .sources
        .contains_key("source.my_project.raw.orders"));
    assert!(manifest.nodes.keys().all(|k| !k.starts_with("source.")));
}

#[test]
fn drops_objects_from_other_packages() {
    let (_dir, manifest) = full_index();
    assert!(!manifest.nodes.contains_key("model.other_package.vendored"));
    assert!(manifest
        .nodes
        .values()
        .all(|n| n.get_package_name() == PROJECT));
}

#[test]
fn unflattens_config_onto_the_node() {
    let (_dir, manifest) = full_index();
    let orders = &manifest.nodes["model.my_project.orders"];
    assert_eq!(
        orders.get_materialization().map(ToString::to_string),
        Some("table".to_string())
    );
    let config = orders.get_base().config.as_ref().expect("config missing");
    assert!(config.contract.as_ref().expect("contract missing").enforced);
}

#[test]
fn reads_tags_from_a_json_encoded_column() {
    let (_dir, manifest) = full_index();
    let orders = &manifest.nodes["model.my_project.orders"];
    assert_eq!(
        orders.get_base().tags.as_deref(),
        Some(["finance".to_string()].as_slice())
    );
}

#[test]
fn builds_parent_and_child_maps() {
    let (_dir, manifest) = full_index();
    let parents = &manifest.parent_map["model.my_project.orders"];
    assert!(parents.contains(&"model.my_project.stg_orders".to_string()));
    let children = &manifest.child_map["model.my_project.stg_orders"];
    assert!(children.contains(&"model.my_project.orders".to_string()));
}

#[test]
fn separates_macro_dependencies_from_node_dependencies() {
    let (_dir, manifest) = full_index();
    let depends_on = &manifest.nodes["model.my_project.orders"]
        .get_base()
        .depends_on;
    let nodes = depends_on.nodes.as_ref().unwrap();
    let macros = depends_on.macros.as_ref().unwrap();
    assert!(nodes.contains(&"model.my_project.stg_orders".to_string()));
    assert!(!nodes.iter().any(|n| n.starts_with("macro.")));
    assert!(macros.contains(&"macro.dbt.some_macro".to_string()));
}

#[test]
fn attaches_columns_to_their_node() {
    let (_dir, manifest) = full_index();
    let columns = manifest.nodes["model.my_project.orders"]
        .get_base()
        .columns
        .as_ref()
        .expect("columns missing");
    assert_eq!(columns.len(), 2);
    assert_eq!(
        columns["order_id"].data_type.as_deref(),
        Some("integer"),
        "declared_type should win over the warehouse type"
    );
    assert_eq!(columns["status"].data_type, None);
}

#[test]
fn resolves_test_metadata() {
    let (_dir, manifest) = full_index();
    let Node::Test(test) = &manifest.nodes["test.my_project.unique_orders_id"] else {
        panic!("expected a test node");
    };
    assert_eq!(test.get_metadata_name().as_deref(), Some("unique"));
    assert_eq!(test.column_name.as_deref(), Some("order_id"));
    assert_eq!(
        test.attached_node.as_deref(),
        Some("model.my_project.orders")
    );
}

#[test]
fn tolerates_an_index_with_only_the_nodes_table() {
    let dir = TempDir::new().unwrap();
    let index = dir.path().join("index");
    IndexBuilder::new(&index)
        .unwrap()
        .table("dbt.nodes", nodes_table())
        .unwrap();

    let manifest = Manifest::from_index(&index).expect("nodes-only index should parse");
    // Without dbt.project there is no project name, so nothing is filtered out.
    assert_eq!(manifest.nodes.len(), 4);
    assert!(manifest.parent_map.is_empty());
}

#[test]
fn tolerates_unknown_and_missing_columns() {
    let dir = TempDir::new().unwrap();
    let index = dir.path().join("index");
    let drifted = TableBuilder::new(&[
        ("unique_id", Utf8),
        ("name", Utf8),
        ("resource_type", Utf8),
        ("package_name", Utf8),
        ("a_column_from_a_future_release", Utf8),
    ])
    .row(&[
        Cell::Str("model.my_project.orders"),
        Cell::Str("orders"),
        Cell::Str("model"),
        Cell::Str(PROJECT),
        Cell::Str("ignore me"),
    ]);
    IndexBuilder::new(&index)
        .unwrap()
        .table("dbt.project", project_table())
        .unwrap()
        .table("dbt.nodes", drifted)
        .unwrap();

    let manifest = Manifest::from_index(&index).expect("drifted schema should still parse");
    assert_eq!(manifest.nodes.len(), 1);
    assert!(manifest.nodes["model.my_project.orders"]
        .get_base()
        .config
        .is_none());
}

#[test]
fn errors_when_the_nodes_table_is_absent() {
    let dir = TempDir::new().unwrap();
    let index = dir.path().join("index");
    IndexBuilder::new(&index)
        .unwrap()
        .table("dbt.project", project_table())
        .unwrap();

    let err = Manifest::from_index(&index).unwrap_err().to_string();
    assert!(err.contains("dbt.nodes"), "unexpected error: {err}");
}

#[test]
fn errors_when_there_is_no_index() {
    let dir = TempDir::new().unwrap();
    let err = Manifest::from_index(dir.path().join("nope"))
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("No dbt Parquet index"),
        "unexpected error: {err}"
    );
}

#[test]
fn parses_the_committed_dbt_v2_fixture() {
    let index = PathBuf::from("dbt_project/target/index");
    if !index.is_dir() {
        return;
    }
    let manifest = Manifest::from_index(&index).expect("committed fixture should parse");
    assert_eq!(
        manifest.metadata.project_name.as_deref(),
        Some("dbtective_test_project")
    );
    assert!(!manifest.nodes.is_empty());
    assert!(!manifest.sources.is_empty());
}
