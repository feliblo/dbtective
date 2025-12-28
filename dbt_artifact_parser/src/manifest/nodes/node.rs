use super::super::dbt_objects::column::Column;
use super::super::dbt_objects::{Meta, Tags};
use super::super::materialization::Materialization;
use super::{Analysis, HookNode, Model, Seed, Snapshot, SqlOperation, Test};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "resource_type")]
#[allow(dead_code)]
pub enum Node {
    #[serde(rename = "analysis")]
    Analysis(Analysis),
    #[serde(rename = "seed")]
    Seed(Seed),
    #[serde(rename = "model")]
    Model(Model),
    #[serde(rename = "test")]
    Test(Test),
    #[serde(rename = "snapshot")]
    Snapshot(Snapshot),
    #[serde(rename = "operation")]
    HookNode(HookNode),
    #[serde(rename = "sql_operation")]
    SqlOperation(SqlOperation),
}

impl Node {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Analysis(_) => "Analysis",
            Self::Seed(_) => "Seed",
            Self::Model(_) => "Model",
            Self::Test(_) => "Test",
            Self::Snapshot(_) => "Snapshot",
            Self::HookNode(_) => "Operation",
            Self::SqlOperation(_) => "SqlOperation",
        }
    }

    pub const fn get_name(&self) -> &String {
        &self.get_base().name
    }

    pub const fn get_base(&self) -> &NodeBase {
        match self {
            Self::Analysis(a) => &a.base,
            Self::Seed(s) => &s.base,
            Self::Model(m) => &m.base,
            Self::Test(t) => &t.base,
            Self::Snapshot(s) => &s.base,
            Self::HookNode(h) => &h.base,
            Self::SqlOperation(s) => &s.base,
        }
    }

    pub const fn get_unique_id(&self) -> &String {
        &self.get_base().unique_id
    }

    pub const fn get_package_name(&self) -> &String {
        &self.get_base().package_name
    }

    pub fn get_object_string(&self) -> &str {
        self.get_name()
    }

    pub const fn get_object_type(&self) -> &str {
        self.as_str()
    }

    pub const fn get_relative_path(&self) -> &String {
        &self.get_base().original_file_path
    }

    pub fn get_materialization(&self) -> Option<&Materialization> {
        match self {
            Self::Model(_) => self
                .get_base()
                .config
                .as_ref()
                .and_then(|c| c.materialized.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct FileHash {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub checksum: String,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct DependsOn {
    pub nodes: Option<Vec<String>>,
    pub macros: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub contract: Option<Contract>,
    pub materialized: Option<Materialization>,
}

#[derive(Debug, Deserialize)]
pub struct Contract {
    pub enforced: bool,
    #[allow(dead_code)]
    pub alias_types: bool,
}

// Base Layer: Core fields ALL nodes have
#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct NodeBase {
    pub database: Option<String>,
    #[serde(default)]
    pub schema: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub original_file_path: String,
    #[serde(default)]
    pub unique_id: String,
    #[serde(default)]
    pub fqn: Vec<String>,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub checksum: FileHash,

    // Common optional fields
    pub tags: Option<Tags>,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    pub columns: Option<HashMap<String, Column>>,
    pub config: Option<NodeConfig>,
    #[serde(default)]
    pub depends_on: DependsOn,
    pub raw_code: Option<String>,
}

// Layer 2: Compiled node specific fields
#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct CompiledNodeFields {
    pub language: Option<String>,
}
