use crate::core::catalog::{
    columns::CatalogColumn, resource_metadata::CatalogResourceMetadata, stats::CatalogStat,
};

use crate::core::manifest::dbt_objects::Node as ManifestNode;
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

    #[allow(dead_code)]
    pub const fn match_manifest_node(&self, manifest_node: &ManifestNode) -> bool {
        matches!(
            (self, manifest_node),
            (Self::Model(_), ManifestNode::Model(_))
                | (Self::Seed(_), ManifestNode::Seed(_))
                | (Self::Snapshot(_), ManifestNode::Snapshot(_))
                | (Self::Test(_), ManifestNode::Test(_))
                | (Self::Analysis(_), ManifestNode::Analysis(_))
                | (Self::Operation(_), ManifestNode::HookNode(_))
                | (Self::SqlOperation(_), ManifestNode::SqlOperation(_))
        )
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
