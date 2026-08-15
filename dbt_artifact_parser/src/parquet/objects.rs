use super::reader::IndexRow;
use crate::manifest::dbt_objects::Meta;
use crate::manifest::exposure::{Exposure, ExposureDependsOn};
use crate::manifest::group::{Group, GroupOwner};
use crate::manifest::macro_obj::Macro;
use crate::manifest::metric::Metric;
use crate::manifest::saved_query::SavedQuery;
use crate::manifest::semantic_model::{SemanticModel, SemanticModelDependsOn};
use crate::manifest::unit_test::UnitTest;

pub fn read_macro(row: &IndexRow) -> Option<(String, Macro)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        Macro {
            name: row.str("name").unwrap_or_default(),
            package_name: row.str("package_name").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            patch_path: row.non_empty_str("patch_path"),
            macro_sql: row.str("macro_sql").unwrap_or_default(),
            description: row.non_empty_str("description"),
            meta: row.json("meta").map(Meta),
        },
    ))
}

pub fn read_exposure(row: &IndexRow) -> Option<(String, Exposure)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        Exposure {
            name: row.str("name").unwrap_or_default(),
            package_name: row.str("package_name").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            patch_path: row.non_empty_str("patch_path"),
            description: row.non_empty_str("description"),
            meta: row.json("meta").map(Meta),
            tags: row.list("tags"),
            depends_on: ExposureDependsOn {
                macros: row.list("depends_on_macros"),
                nodes: row.list("depends_on_nodes"),
            },
        },
    ))
}

pub fn read_group(row: &IndexRow) -> Option<(String, Group)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id.clone(),
        Group {
            name: row.str("name").unwrap_or_default(),
            resource_type: "group".to_string(),
            package_name: row.str("package_name").unwrap_or_default(),
            path: row.str("file_path").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            unique_id,
            owner: GroupOwner {
                email: row.non_empty_str("owner_email"),
                name: row.non_empty_str("owner_name"),
            },
            description: row.non_empty_str("description"),
            config: row.json("config"),
        },
    ))
}

pub fn read_unit_test(row: &IndexRow) -> Option<(String, UnitTest)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        UnitTest {
            name: row.str("name").unwrap_or_default(),
            model: row.str("model").unwrap_or_default(),
            package_name: row.str("package_name").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            patch_path: row.non_empty_str("patch_path"),
            description: row.non_empty_str("description"),
        },
    ))
}

pub fn read_semantic_model(row: &IndexRow) -> Option<(String, SemanticModel)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        SemanticModel {
            name: row.str("name").unwrap_or_default(),
            package_name: row.str("package_name").unwrap_or_default(),
            original_file_path: row.str("original_file_path").unwrap_or_default(),
            patch_path: row.non_empty_str("patch_path"),
            description: row.non_empty_str("description"),
            metadata: row.json("meta").map(Meta),
            depends_on: SemanticModelDependsOn {
                macros: row.list("depends_on_macros"),
                nodes: row.list("depends_on_nodes"),
            },
        },
    ))
}

pub fn read_metric(row: &IndexRow) -> Option<(String, Metric)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        Metric {
            name: row.str("name").unwrap_or_default(),
        },
    ))
}

pub fn read_saved_query(row: &IndexRow) -> Option<(String, SavedQuery)> {
    let unique_id = row.non_empty_str("unique_id")?;
    Some((
        unique_id,
        SavedQuery {
            name: row.str("name").unwrap_or_default(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        read_exposure, read_group, read_macro, read_metric, read_saved_query, read_semantic_model,
        read_unit_test,
    };
    use crate::parquet::test_writer::{with_rows, Cell, ColumnKind};
    use ColumnKind::Utf8;

    #[test]
    fn maps_a_project_macro() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("package_name", Utf8),
            ("original_file_path", Utf8),
            ("macro_sql", Utf8),
            ("description", Utf8),
            ("patch_path", Utf8),
        ];
        let row = &[
            Cell::Str("macro.my_project.cents_to_dollars"),
            Cell::Str("cents_to_dollars"),
            Cell::Str("my_project"),
            Cell::Str("macros/cents_to_dollars.sql"),
            Cell::Str("{% macro cents_to_dollars(column_name) %} ... {% endmacro %}"),
            Cell::Str("Converts cents to dollars"),
            Cell::Str("my_project://macros/_macros.yml"),
        ];

        with_rows(columns, &[row], |r| {
            let (id, m) = read_macro(r).expect("macro row should map");
            assert_eq!(id, "macro.my_project.cents_to_dollars");
            assert_eq!(m.get_name(), "cents_to_dollars");
            assert_eq!(m.get_package_name(), "my_project");
            assert_eq!(m.description.as_deref(), Some("Converts cents to dollars"));
            // The project:// prefix is stripped so rules can match on a real path.
            assert_eq!(m.get_patch_path(), Some("macros/_macros.yml"));
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_an_exposure_with_its_dependencies() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("package_name", Utf8),
            ("original_file_path", Utf8),
            ("description", Utf8),
            ("tags", Utf8),
            ("depends_on_nodes", Utf8),
            ("meta", Utf8),
        ];
        let row = &[
            Cell::Str("exposure.my_project.customers"),
            Cell::Str("customers"),
            Cell::Str("my_project"),
            Cell::Str("models/marts/finance/_exposures.yml"),
            Cell::Null,
            Cell::Str(r#"["daily"]"#),
            Cell::Str(r#"["model.my_project.customers"]"#),
            Cell::Str(r#"{"owner":"finance"}"#),
        ];

        with_rows(columns, &[row], |r| {
            let (id, e) = read_exposure(r).expect("exposure row should map");
            assert_eq!(id, "exposure.my_project.customers");
            assert_eq!(e.get_name(), "customers");
            assert_eq!(
                e.description, None,
                "undocumented exposure has no description"
            );
            assert_eq!(e.tags.as_deref(), Some(["daily".to_string()].as_slice()));
            assert_eq!(
                e.depends_on.nodes.as_deref(),
                Some(["model.my_project.customers".to_string()].as_slice())
            );
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_a_group_owner() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("package_name", Utf8),
            ("file_path", Utf8),
            ("original_file_path", Utf8),
            ("owner_name", Utf8),
            ("owner_email", Utf8),
        ];
        let row = &[
            Cell::Str("group.my_project.finance"),
            Cell::Str("finance"),
            Cell::Str("my_project"),
            Cell::Str("models/marts/_groups.yml"),
            Cell::Str("models/marts/_groups.yml"),
            Cell::Str("Finance Team"),
            Cell::Str("finance@example.com"),
        ];

        with_rows(columns, &[row], |r| {
            let (id, g) = read_group(r).expect("group row should map");
            assert_eq!(id, "group.my_project.finance");
            assert_eq!(g.unique_id, "group.my_project.finance");
            assert_eq!(g.resource_type, "group");
            assert_eq!(g.owner.name.as_deref(), Some("Finance Team"));
            assert_eq!(g.owner.email.as_deref(), Some("finance@example.com"));
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_a_unit_test_to_its_model() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("model", Utf8),
            ("package_name", Utf8),
            ("original_file_path", Utf8),
        ];
        let row = &[
            Cell::Str("unit_test.my_project.customers.test_is_valid"),
            Cell::Str("test_is_valid"),
            Cell::Str("customers"),
            Cell::Str("my_project"),
            Cell::Str("models/marts/finance/_finance_unit_tests.yml"),
        ];

        with_rows(columns, &[row], |r| {
            let (_, ut) = read_unit_test(r).expect("unit test row should map");
            assert_eq!(ut.get_name(), "test_is_valid");
            assert_eq!(ut.model, "customers");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_a_semantic_model() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("package_name", Utf8),
            ("original_file_path", Utf8),
            ("depends_on_nodes", Utf8),
        ];
        let row = &[
            Cell::Str("semantic_model.my_project.orders"),
            Cell::Str("orders"),
            Cell::Str("my_project"),
            Cell::Str("models/marts/finance/_semantic_models.yml"),
            Cell::Str(r#"["model.my_project.orders"]"#),
        ];

        with_rows(columns, &[row], |r| {
            let (_, sm) = read_semantic_model(r).expect("semantic model row should map");
            assert_eq!(sm.get_name(), "orders");
            assert_eq!(
                sm.depends_on.nodes.as_deref(),
                Some(["model.my_project.orders".to_string()].as_slice())
            );
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_a_metric() {
        let columns = &[("unique_id", Utf8), ("name", Utf8), ("metric_type", Utf8)];
        let row = &[
            Cell::Str("metric.my_project.total_revenue"),
            Cell::Str("total_revenue"),
            Cell::Str("simple"),
        ];

        with_rows(columns, &[row], |r| {
            let (id, metric) = read_metric(r).expect("metric row should map");
            assert_eq!(id, "metric.my_project.total_revenue");
            assert_eq!(metric.name, "total_revenue");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn maps_a_saved_query() {
        let columns = &[("unique_id", Utf8), ("name", Utf8)];
        let row = &[
            Cell::Str("saved_query.my_project.revenue_by_month"),
            Cell::Str("revenue_by_month"),
        ];

        with_rows(columns, &[row], |r| {
            let (id, query) = read_saved_query(r).expect("saved query row should map");
            assert_eq!(id, "saved_query.my_project.revenue_by_month");
            assert_eq!(query.name, "revenue_by_month");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn skips_a_row_without_a_unique_id() {
        with_rows(
            &[("unique_id", Utf8), ("name", Utf8)],
            &[&[Cell::Null, Cell::Str("x")]],
            |r| {
                assert!(read_macro(r).is_none());
                assert!(read_exposure(r).is_none());
                Ok(())
            },
        )
        .unwrap();
    }
}
