use crate::core::catalog::{
    columns::CatalogColumn, resource_metadata::CatalogResourceMetadata, stats::CatalogStat,
};

use crate::core::checks::common_traits::Columnable;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Base structure shared by all catalog nodes
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogNodeBase {
    pub unique_id: String,
    pub metadata: CatalogResourceMetadata,
    pub columns: HashMap<String, CatalogColumn>,
    pub stats: HashMap<String, CatalogStat>,
}

// Catalog node types based on unique_id prefix (e.g., "model.project.name")
#[derive(Debug)]
#[allow(dead_code)]
pub enum CatalogNode {
    Model(CatalogModel),
    Seed(CatalogSeed),
    Snapshot(CatalogSnapshot),
    Test(CatalogTest),
    Analysis(CatalogAnalysis),
    Operation(CatalogOperation),
    SqlOperation(CatalogSqlOperation),
}

impl CatalogNode {
    pub fn get_name(&self) -> &str {
        &self.get_base().metadata.name
    }

    fn from_base(base: CatalogNodeBase) -> Result<Self, String> {
        let resource_type = base
            .unique_id
            .split('.')
            .next()
            .ok_or_else(|| format!("Invalid unique_id format: {}", base.unique_id))?;

        match resource_type {
            "model" => Ok(Self::Model(CatalogModel { base })),
            "seed" => Ok(Self::Seed(CatalogSeed { base })),
            "snapshot" => Ok(Self::Snapshot(CatalogSnapshot { base })),
            "test" => Ok(Self::Test(CatalogTest { base })),
            "analysis" => Ok(Self::Analysis(CatalogAnalysis { base })),
            "operation" => Ok(Self::Operation(CatalogOperation { base })),
            "sql_operation" => Ok(Self::SqlOperation(CatalogSqlOperation { base })),
            _ => Err(format!("Unknown resource type: {resource_type}")),
        }
    }

    pub fn get_unique_id(&self) -> &str {
        &self.get_base().unique_id
    }
}

impl<'de> Deserialize<'de> for CatalogNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let base = CatalogNodeBase::deserialize(deserializer)?;
        Self::from_base(base).map_err(serde::de::Error::custom)
    }
}

impl CatalogNode {
    #[allow(dead_code)]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Model(_) => "Model",
            Self::Seed(_) => "Seed",
            Self::Snapshot(_) => "Snapshot",
            Self::Test(_) => "Test",
            Self::Analysis(_) => "Analysis",
            Self::Operation(_) => "Operation",
            Self::SqlOperation(_) => "SqlOperation",
        }
    }

    const fn get_object_type(&self) -> &str {
        self.as_str()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    #[allow(dead_code)]
    pub const fn get_base(&self) -> &CatalogNodeBase {
        match self {
            Self::Model(m) => &m.base,
            Self::Seed(s) => &s.base,
            Self::Snapshot(s) => &s.base,
            Self::Test(t) => &t.base,
            Self::Analysis(a) => &a.base,
            Self::Operation(o) => &o.base,
            Self::SqlOperation(s) => &s.base,
        }
    }
}

impl Columnable for CatalogNode {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.get_base()
            .columns
            .keys()
            .collect::<Vec<&String>>()
            .into()
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }

    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

impl Columnable for &CatalogNode {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        (*self).get_column_names()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_object_string(&self) -> &str {
        (*self).get_object_string()
    }
    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}
// Specific node types
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogModel {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogSeed {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogSnapshot {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogTest {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogAnalysis {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogOperation {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogSqlOperation {
    #[serde(flatten)]
    pub base: CatalogNodeBase,
}
