//! Builds synthetic index tables so tests do not depend on committed artifacts.

use ::parquet::basic::Compression;
use ::parquet::data_type::{BoolType, ByteArray, ByteArrayType, DoubleType, Int32Type, Int64Type};
use ::parquet::file::properties::WriterProperties;
use ::parquet::file::writer::SerializedFileWriter;
use ::parquet::schema::parser::parse_message_type;
use anyhow::Result;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColumnKind {
    Utf8,
    Bool,
    Int32,
    Int64,
    Double,
}

#[derive(Clone)]
pub enum Cell {
    Str(&'static str),
    Owned(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Null,
}

impl Cell {
    const fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            Self::Owned(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

pub struct TableBuilder {
    columns: Vec<(String, ColumnKind)>,
    rows: Vec<Vec<Cell>>,
}

impl TableBuilder {
    pub fn new(columns: &[(&str, ColumnKind)]) -> Self {
        Self {
            columns: columns
                .iter()
                .map(|(n, k)| ((*n).to_string(), *k))
                .collect(),
            rows: Vec::new(),
        }
    }

    /// Every column is optional, so a short row is padded with nulls.
    #[must_use]
    pub fn row(mut self, cells: &[Cell]) -> Self {
        let mut row = cells.to_vec();
        row.resize(self.columns.len(), Cell::Null);
        self.rows.push(row);
        self
    }

    fn message_type(&self) -> String {
        let fields: Vec<String> = self
            .columns
            .iter()
            .map(|(name, kind)| match kind {
                ColumnKind::Utf8 => format!("  OPTIONAL BYTE_ARRAY {name} (UTF8);"),
                ColumnKind::Bool => format!("  OPTIONAL BOOLEAN {name};"),
                ColumnKind::Int32 => format!("  OPTIONAL INT32 {name};"),
                ColumnKind::Int64 => format!("  OPTIONAL INT64 {name};"),
                ColumnKind::Double => format!("  OPTIONAL DOUBLE {name};"),
            })
            .collect();
        format!("message index {{\n{}\n}}", fields.join("\n"))
    }

    /// # Errors
    /// If the file cannot be written.
    pub fn write(self, path: &Path) -> Result<()> {
        let schema = Arc::new(parse_message_type(&self.message_type())?);
        let props = Arc::new(
            WriterProperties::builder()
                .set_compression(Compression::SNAPPY)
                .build(),
        );
        let mut writer = SerializedFileWriter::new(File::create(path)?, schema, props)?;
        let mut group = writer.next_row_group()?;

        let mut col = 0;
        while let Some(mut column) = group.next_column()? {
            let kind = self.columns[col].1;
            let cells: Vec<&Cell> = self.rows.iter().map(|r| &r[col]).collect();
            let def_levels: Vec<i16> = cells
                .iter()
                .map(|c| i16::from(!matches!(c, Cell::Null)))
                .collect();

            match kind {
                ColumnKind::Utf8 => {
                    let values: Vec<ByteArray> = cells
                        .iter()
                        .filter_map(|c| c.as_str())
                        .map(ByteArray::from)
                        .collect();
                    column.typed::<ByteArrayType>().write_batch(
                        &values,
                        Some(&def_levels),
                        None,
                    )?;
                }
                ColumnKind::Bool => {
                    let values: Vec<bool> = cells
                        .iter()
                        .filter_map(|c| match c {
                            Cell::Bool(b) => Some(*b),
                            _ => None,
                        })
                        .collect();
                    column
                        .typed::<BoolType>()
                        .write_batch(&values, Some(&def_levels), None)?;
                }
                ColumnKind::Int32 => {
                    let values: Vec<i32> = cells
                        .iter()
                        .filter_map(|c| match c {
                            Cell::Int(v) => i32::try_from(*v).ok(),
                            _ => None,
                        })
                        .collect();
                    column
                        .typed::<Int32Type>()
                        .write_batch(&values, Some(&def_levels), None)?;
                }
                ColumnKind::Int64 => {
                    let values: Vec<i64> = cells
                        .iter()
                        .filter_map(|c| match c {
                            Cell::Int(v) => Some(*v),
                            _ => None,
                        })
                        .collect();
                    column
                        .typed::<Int64Type>()
                        .write_batch(&values, Some(&def_levels), None)?;
                }
                ColumnKind::Double => {
                    let values: Vec<f64> = cells
                        .iter()
                        .filter_map(|c| match c {
                            Cell::Float(v) => Some(*v),
                            _ => None,
                        })
                        .collect();
                    column
                        .typed::<DoubleType>()
                        .write_batch(&values, Some(&def_levels), None)?;
                }
            }
            column.close()?;
            col += 1;
        }

        group.close()?;
        writer.close()?;
        Ok(())
    }
}

/// Writes a one-table index and hands each decoded row to `check`.
///
/// # Errors
/// If the table cannot be written or read back.
#[cfg(test)]
pub fn with_rows<F>(columns: &[(&str, ColumnKind)], rows: &[&[Cell]], mut check: F) -> Result<()>
where
    F: FnMut(&super::reader::IndexRow) -> Result<()>,
{
    let dir = tempfile::TempDir::new()?;
    let path = dir.path().join("dbt.probe.parquet");
    let mut builder = TableBuilder::new(columns);
    for row in rows {
        builder = builder.row(row);
    }
    builder.write(&path)?;
    super::reader::Table::open(&path)?.for_each_row(|row| check(row))
}

pub struct IndexBuilder {
    dir: std::path::PathBuf,
}

impl IndexBuilder {
    /// # Errors
    /// If the directory cannot be created.
    pub fn new(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir)?;
        Ok(Self {
            dir: dir.to_path_buf(),
        })
    }

    /// # Errors
    /// If the table cannot be written.
    pub fn table(self, name: &str, builder: TableBuilder) -> Result<Self> {
        builder.write(&self.dir.join(format!("{name}.parquet")))?;
        Ok(self)
    }
}
