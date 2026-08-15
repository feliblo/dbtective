use ::parquet::file::reader::{FileReader, SerializedFileReader};
use ::parquet::record::{Field, Row};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct Table {
    path: PathBuf,
    reader: SerializedFileReader<File>,
    columns: HashMap<String, usize>,
}

impl Table {
    /// # Errors
    /// If the file cannot be opened or is not valid parquet.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)
            .with_context(|| format!("Unable to open index table at {}", path.display()))?;
        let reader = SerializedFileReader::new(file)
            .with_context(|| format!("Unable to read parquet metadata from {}", path.display()))?;

        let columns = reader
            .metadata()
            .file_metadata()
            .schema()
            .get_fields()
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name().to_string(), i))
            .collect();

        Ok(Self {
            path,
            reader,
            columns,
        })
    }

    /// # Errors
    /// If a row cannot be decoded, or if `f` returns one.
    pub fn for_each_row<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&IndexRow) -> Result<()>,
    {
        let iter = self
            .reader
            .get_row_iter(None)
            .with_context(|| format!("Unable to iterate rows of {}", self.path.display()))?;

        for (n, row) in iter.enumerate() {
            let row = row
                .with_context(|| format!("Unable to decode row {n} of {}", self.path.display()))?;
            let row = IndexRow::new(row, &self.columns);
            f(&row).with_context(|| format!("Failed to map row {n} of {}", self.path.display()))?;
        }
        Ok(())
    }
}

pub struct IndexRow<'t> {
    fields: Vec<Field>,
    columns: &'t HashMap<String, usize>,
}

impl<'t> IndexRow<'t> {
    fn new(row: Row, columns: &'t HashMap<String, usize>) -> Self {
        let fields = row.into_columns().into_iter().map(|(_, f)| f).collect();
        Self { fields, columns }
    }

    fn field(&self, name: &str) -> Option<&Field> {
        let idx = *self.columns.get(name)?;
        match self.fields.get(idx)? {
            Field::Null => None,
            other => Some(other),
        }
    }

    /// Renders a non-text field so a column that changes type between alpha
    /// releases still reads.
    pub fn str(&self, name: &str) -> Option<String> {
        match self.field(name)? {
            Field::Str(s) => Some(s.clone()),
            Field::Bool(v) => Some(v.to_string()),
            Field::Int(v) => Some(v.to_string()),
            Field::Long(v) => Some(v.to_string()),
            Field::Float(v) => Some(v.to_string()),
            Field::Double(v) => Some(v.to_string()),
            _ => None,
        }
    }

    pub fn non_empty_str(&self, name: &str) -> Option<String> {
        self.str(name).filter(|s| !s.is_empty())
    }

    pub fn bool(&self, name: &str) -> Option<bool> {
        match self.field(name)? {
            Field::Bool(b) => Some(*b),
            Field::Str(s) => match s.to_ascii_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn i64(&self, name: &str) -> Option<i64> {
        match self.field(name)? {
            Field::Int(v) => Some(i64::from(*v)),
            Field::Long(v) => Some(*v),
            _ => None,
        }
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.field(name)? {
            Field::ListInternal(list) => Some(
                list.elements()
                    .iter()
                    .filter_map(|f| match f {
                        Field::Str(s) => Some(s.clone()),
                        Field::Null => None,
                        other => Some(other.to_string()),
                    })
                    .collect(),
            ),
            Field::Str(s) => serde_json::from_str::<Vec<String>>(s).ok(),
            _ => None,
        }
    }

    /// dbt double-encodes some of these, so a string result is parsed again.
    pub fn json(&self, name: &str) -> Option<serde_json::Value> {
        let raw = self.non_empty_str(name)?;
        match serde_json::from_str::<serde_json::Value>(&raw) {
            Ok(serde_json::Value::Null) | Err(_) => None,
            Ok(serde_json::Value::String(inner)) => {
                match serde_json::from_str::<serde_json::Value>(&inner) {
                    Ok(serde_json::Value::Null) | Err(_) => None,
                    Ok(v) => Some(v),
                }
            }
            Ok(v) => Some(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parquet::test_writer::{with_rows, Cell, ColumnKind};
    use ColumnKind::{Bool, Double, Int32, Int64, Utf8};

    /// A `dbt.nodes` row as dbt v2 writes it: config is a JSON string, and for
    /// models that string is itself JSON-encoded a second time.
    #[test]
    fn reads_a_model_row() {
        let columns = &[
            ("unique_id", Utf8),
            ("name", Utf8),
            ("materialized", Utf8),
            ("contract_enforced", Bool),
            ("config", Utf8),
            ("patch_path", Utf8),
        ];
        let row = &[
            Cell::Str("model.my_project.orders"),
            Cell::Str("orders"),
            Cell::Str("table"),
            Cell::Bool(true),
            Cell::Str(
                r#""{\"enabled\":true,\"meta\":{\"owner\":\"finance\"},\"tags\":[\"daily\"]}""#,
            ),
            Cell::Null,
        ];

        with_rows(columns, &[row], |r| {
            assert_eq!(
                r.str("unique_id").as_deref(),
                Some("model.my_project.orders")
            );
            assert_eq!(r.str("materialized").as_deref(), Some("table"));
            assert_eq!(r.bool("contract_enforced"), Some(true));
            assert_eq!(
                r.non_empty_str("patch_path"),
                None,
                "a SQL-only model has no patch_path"
            );

            let config = r
                .json("config")
                .expect("double-encoded config should parse");
            assert_eq!(config["meta"]["owner"], "finance");
            assert_eq!(config["tags"][0], "daily");
            Ok(())
        })
        .unwrap();
    }

    /// `dbt.node_columns` carries a warehouse type only after
    /// `--static-analysis strict`; undocumented columns come back NULL.
    #[test]
    fn reads_node_column_rows() {
        let columns = &[
            ("unique_id", Utf8),
            ("column_name", Utf8),
            ("column_index", Int64),
            ("data_type", Utf8),
            ("description", Utf8),
        ];
        let typed = &[
            Cell::Str("model.my_project.orders"),
            Cell::Str("order_id"),
            Cell::Int(0),
            Cell::Str("INTEGER"),
            Cell::Str("Primary key"),
        ];
        let untyped = &[
            Cell::Str("model.my_project.orders"),
            Cell::Str("status"),
            Cell::Int(1),
            Cell::Null,
            Cell::Null,
        ];

        let mut seen = Vec::new();
        with_rows(columns, &[typed, untyped], |r| {
            seen.push((
                r.str("column_name").unwrap(),
                r.i64("column_index").unwrap(),
                r.non_empty_str("data_type"),
                r.non_empty_str("description"),
            ));
            Ok(())
        })
        .unwrap();

        assert_eq!(
            seen,
            vec![
                (
                    "order_id".to_string(),
                    0,
                    Some("INTEGER".to_string()),
                    Some("Primary key".to_string())
                ),
                ("status".to_string(), 1, None, None),
            ]
        );
    }

    /// A column dbtective asks for that this dbt release does not write.
    #[test]
    fn unknown_column_reads_as_absent() {
        with_rows(&[("unique_id", Utf8)], &[&[Cell::Str("model.a")]], |r| {
            assert_eq!(r.str("a_column_from_a_future_release"), None);
            assert_eq!(r.bool("a_column_from_a_future_release"), None);
            assert_eq!(r.list("a_column_from_a_future_release"), None);
            assert_eq!(r.json("a_column_from_a_future_release"), None);
            Ok(())
        })
        .unwrap();
    }

    /// If a column dbtective reads as text is emitted as a number or boolean by a
    /// later alpha, it should still read rather than silently vanish.
    #[test]
    fn a_column_that_changed_type_still_reads_as_text() {
        with_rows(
            &[
                ("enabled", Bool),
                ("column_index", Int32),
                ("created_at", Double),
                ("version", Int64),
            ],
            &[&[
                Cell::Bool(true),
                Cell::Int(3),
                Cell::Float(1.5),
                Cell::Int(2),
            ]],
            |r| {
                assert_eq!(r.str("enabled").as_deref(), Some("true"));
                assert_eq!(r.str("column_index").as_deref(), Some("3"));
                assert_eq!(r.str("created_at").as_deref(), Some("1.5"));
                assert_eq!(r.str("version").as_deref(), Some("2"));
                Ok(())
            },
        )
        .unwrap();
    }

    #[test]
    fn integer_columns_read_at_either_width() {
        with_rows(
            &[("column_index", Int32), ("test_limit", Int64)],
            &[&[Cell::Int(2), Cell::Int(500)]],
            |r| {
                assert_eq!(r.i64("column_index"), Some(2));
                assert_eq!(r.i64("test_limit"), Some(500));
                assert_eq!(r.i64("missing"), None);
                Ok(())
            },
        )
        .unwrap();
    }

    #[test]
    fn a_boolean_written_as_text_still_reads() {
        with_rows(
            &[("contract_enforced", Utf8), ("enabled", Utf8)],
            &[&[Cell::Str("TRUE"), Cell::Str("maybe")]],
            |r| {
                assert_eq!(r.bool("contract_enforced"), Some(true));
                assert_eq!(r.bool("enabled"), None);
                Ok(())
            },
        )
        .unwrap();
    }

    /// `fqn` and `tags` are native lists, but an alpha release could encode them
    /// as a JSON array in a text column.
    #[test]
    fn reads_lists_encoded_as_json_text() {
        with_rows(
            &[("fqn", Utf8), ("tags", Utf8)],
            &[&[
                Cell::Str(r#"["my_project","marts","orders"]"#),
                Cell::Str("[]"),
            ]],
            |r| {
                assert_eq!(
                    r.list("fqn"),
                    Some(vec![
                        "my_project".to_string(),
                        "marts".to_string(),
                        "orders".to_string()
                    ])
                );
                assert_eq!(r.list("tags"), Some(vec![]), "empty list is not absent");
                Ok(())
            },
        )
        .unwrap();
    }
}
