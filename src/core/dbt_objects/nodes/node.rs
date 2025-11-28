use super::super::column::Column;
use super::super::{Meta, Tags};
use super::{Analysis, HookNode, Model, Seed, Snapshot, SqlOperation, Test};
use crate::core::checks::common::name_convention::NameAble;
use crate::core::config::applies_to::RuleTarget;
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::traits::Descriptable;
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

    // Match config rule target names to node types
    pub const fn ruletarget(&self) -> RuleTarget {
        match self {
            Self::Model(_) => RuleTarget::Models,
            Self::Seed(_) => RuleTarget::Seeds,
            Self::Test(_) => RuleTarget::Tests,
            Self::Analysis(_) => RuleTarget::Analyses,
            Self::Snapshot(_) => RuleTarget::Snapshots,
            Self::HookNode(_) => RuleTarget::HookNodes,
            Self::SqlOperation(_) => RuleTarget::SqlOperations,
        }
    }
}

impl Node {
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

    pub const fn original_file_path(&self) -> &String {
        &self.get_base().original_file_path
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
}

impl NameAble for Node {
    fn name(&self) -> &str {
        self.get_name()
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }
}

impl IncludeExcludable for Node {
    fn get_path(&self) -> &String {
        &self.get_base().original_file_path
    }
}

impl IncludeExcludable for &Node {
    fn get_path(&self) -> &String {
        (*self).get_path()
    }
}

impl Descriptable for Node {
    fn description(&self) -> Option<&String> {
        self.get_base().description.as_ref()
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }
}

impl Descriptable for &Node {
    fn description(&self) -> Option<&String> {
        (*self).description()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_object_string(&self) -> &str {
        (*self).get_object_string()
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FileHash {
    pub name: String,
    pub checksum: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MacroDependsOn {
    pub macros: Vec<String>,
}

// Base Layer: Core fields ALL nodes have
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NodeBase {
    pub database: Option<String>,
    pub schema: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: FileHash,

    // Common optional fields
    pub tags: Option<Tags>,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    pub columns: Option<HashMap<String, Column>>,
    pub config: Option<serde_json::Value>,
    // Currently unused fields that do exist in the data
    // pub group: Option<String>,
    // pub docs: Option<NodeDocs>,
    // pub patch_path: Option<String>,
    // pub build_path: Option<String>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub created_at: Option<f64>,
    // pub config_call_dict: Option<serde_json::Value>,
    // pub unrendered_config_call_dict: Option<serde_json::Value>,
    // pub relation_name: Option<String>,
    // pub raw_code: Option<String>,
    // pub doc_blocks: Option<Vec<String>>,
    // pub root_path: Option<String>,
    // pub depends_on: Option<MacroDependsOn>,
}

// Layer 2: Compiled node specific fields
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CompiledNodeFields {
    pub language: Option<String>,
    // Currently unused fields that do exist in the data
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub sources: Option<Vec<serde_json::Value>>,
    // pub metrics: Option<Vec<serde_json::Value>>,
    // pub compiled_path: Option<String>,
    // pub compiled: Option<bool>,
    // pub compiled_code: Option<String>,
    // pub extra_ctes_injected: Option<bool>,
    // pub extra_ctes: Option<Vec<serde_json::Value>>,
    // #[serde(rename = "_pre_injected_sql")]
    // pub pre_injected_sql: Option<String>,
    // pub contract: Option<serde_json::Value>,
}
