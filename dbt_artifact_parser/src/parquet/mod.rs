pub mod catalog;
pub mod columns;
pub mod manifest;
pub mod nodes;
pub mod objects;
pub mod reader;
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_writer;

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

pub use reader::{IndexRow, Table};

pub struct IndexLayout {
    dir: PathBuf,
    tables: Vec<(String, PathBuf)>,
}

impl IndexLayout {
    /// # Errors
    /// If the directory exists but cannot be read.
    pub fn discover<P: AsRef<Path>>(dir: P) -> Result<Option<Self>> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return Ok(None);
        }

        let mut tables = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().is_none_or(|e| e != "parquet") {
                continue;
            }
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                tables.push((name.to_string(), path));
            }
        }

        if tables.is_empty() {
            return Ok(None);
        }
        tables.sort_unstable();

        Ok(Some(Self {
            dir: dir.to_path_buf(),
            tables,
        }))
    }

    fn path_of(&self, table: &str) -> Option<&PathBuf> {
        self.tables
            .iter()
            .find(|(name, _)| name == table)
            .map(|(_, path)| path)
    }

    pub fn table_names(&self) -> Vec<&str> {
        self.tables.iter().map(|(name, _)| name.as_str()).collect()
    }

    /// # Errors
    /// If the table is absent or unreadable.
    pub fn require(&self, table: &str) -> Result<Table> {
        let path = self.path_of(table).ok_or_else(|| {
            anyhow!(
                "The dbt index at {} has no '{table}' table (found: {}).\n\
                 Regenerate it with a dbt v2 command such as 'dbt build --write-index'.",
                self.dir.display(),
                self.table_names().join(", ")
            )
        })?;
        Table::open(path)
    }

    /// # Errors
    /// If the table is present but unreadable.
    pub fn optional(&self, table: &str) -> Result<Option<Table>> {
        self.path_of(table).map(Table::open).transpose()
    }
}
