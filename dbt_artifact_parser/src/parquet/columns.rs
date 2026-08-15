use super::reader::IndexRow;
use crate::catalog::columns::CatalogColumn;
use crate::manifest::dbt_objects::column::Column;
use crate::manifest::dbt_objects::Meta;

pub struct ColumnRow {
    pub unique_id: String,
    pub name: String,
    pub index: i64,
    pub declared_type: Option<String>,
    pub catalog_type: Option<String>,
    pub data_type: Option<String>,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub tags: Option<Vec<String>>,
    pub meta: Option<serde_json::Value>,
    pub quote: Option<bool>,
    pub granularity: Option<String>,
}

impl ColumnRow {
    pub fn read(row: &IndexRow) -> Option<Self> {
        Some(Self {
            unique_id: row.non_empty_str("unique_id")?,
            name: row.non_empty_str("column_name")?,
            index: row.i64("column_index").unwrap_or_default(),
            declared_type: row.non_empty_str("declared_type"),
            catalog_type: row.non_empty_str("catalog_type"),
            data_type: row.non_empty_str("data_type"),
            description: row.non_empty_str("description"),
            comment: row.non_empty_str("catalog_comment"),
            tags: row.list("tags"),
            meta: row.json("meta"),
            quote: row.bool("quote"),
            granularity: row.non_empty_str("granularity"),
        })
    }

    fn best_type(&self) -> Option<String> {
        self.declared_type
            .clone()
            .or_else(|| self.data_type.clone())
            .or_else(|| self.catalog_type.clone())
    }

    pub fn to_manifest_column(&self) -> Column {
        Column {
            name: self.name.clone(),
            description: self.description.clone(),
            data_type: self.best_type(),
            tests: None,
            meta: self.meta.clone().map(Meta),
            datatype: None,
            constraints: None,
            quoted: self.quote,
            config: None,
            tags: self.tags.clone().unwrap_or_default(),
            _extra: None,
            granularity: self.granularity.clone(),
            doc_blocks: None,
            additional_properties: None,
        }
    }

    /// True when the warehouse reported this column, which is what
    /// `--static-analysis strict` populates.
    pub const fn has_warehouse_type(&self) -> bool {
        self.data_type.is_some() || self.catalog_type.is_some()
    }

    pub fn to_catalog_column(&self) -> CatalogColumn {
        CatalogColumn {
            name: self.name.clone(),
            type_: self
                .data_type
                .clone()
                .or_else(|| self.catalog_type.clone())
                .or_else(|| self.declared_type.clone())
                .unwrap_or_default(),
            index: i32::try_from(self.index).unwrap_or_default(),
            comment: self.comment.clone().or_else(|| self.description.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColumnRow;
    use crate::parquet::test_writer::{with_rows, Cell, ColumnKind};
    use ColumnKind::{Int64, Utf8};

    fn columns() -> &'static [(&'static str, ColumnKind)] {
        &[
            ("unique_id", Utf8),
            ("column_name", Utf8),
            ("column_index", Int64),
            ("declared_type", Utf8),
            ("catalog_type", Utf8),
            ("data_type", Utf8),
            ("description", Utf8),
        ]
    }

    /// A column declared in YAML and also present in the warehouse. The manifest
    /// side should show what the user declared, not what the warehouse reported.
    #[test]
    fn declared_type_wins_for_the_manifest() {
        let row = &[
            Cell::Str("model.my_project.customers"),
            Cell::Str("customer_id"),
            Cell::Int(0),
            Cell::Str("bigint"),
            Cell::Str("BIGINT"),
            Cell::Str("BIGINT"),
            Cell::Str("This is a unique identifier for a customer"),
        ];

        with_rows(columns(), &[row], |r| {
            let col = ColumnRow::read(r).expect("column row should map");
            let manifest_column = col.to_manifest_column();
            assert_eq!(manifest_column.name, "customer_id");
            assert_eq!(manifest_column.data_type.as_deref(), Some("bigint"));
            assert_eq!(
                manifest_column.description.as_deref(),
                Some("This is a unique identifier for a customer")
            );

            assert!(col.has_warehouse_type());
            assert_eq!(col.to_catalog_column().type_, "BIGINT");
            Ok(())
        })
        .unwrap();
    }

    /// A column the warehouse never reported, which is the norm without
    /// `--static-analysis strict`.
    #[test]
    fn untyped_column_is_not_a_warehouse_column() {
        let row = &[
            Cell::Str("model.my_project.stg_orders"),
            Cell::Str("status"),
            Cell::Int(1),
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Null,
        ];

        with_rows(columns(), &[row], |r| {
            let col = ColumnRow::read(r).expect("column row should map");
            assert!(!col.has_warehouse_type());
            assert_eq!(col.to_manifest_column().data_type, None);
            // Still representable in the catalog, just without a type.
            assert_eq!(col.to_catalog_column().type_, "");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn skips_a_row_without_a_column_name() {
        let row = &[
            Cell::Str("model.my_project.orders"),
            Cell::Null,
            Cell::Int(0),
            Cell::Null,
            Cell::Null,
            Cell::Null,
            Cell::Null,
        ];
        with_rows(columns(), &[row], |r| {
            assert!(ColumnRow::read(r).is_none());
            Ok(())
        })
        .unwrap();
    }
}
