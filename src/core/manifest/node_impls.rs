// Trait implementations for Node that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Columnable;
use crate::core::rules::rule_config::child_map::ChildMappable;
use crate::core::rules::rule_config::has_contract_enforced::ContractAble;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_refs::CanReference;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::has_unique_test::TestAble;
use crate::core::rules::rule_config::max_code_lines::HasCode;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::{Manifest, Node};

impl RuleTargetable for Node {
    // Match config rule target names to node types
    fn ruletarget(&self) -> RuleTarget {
        match self {
            Self::Model(_) => RuleTarget::Models,
            Self::Seed(_) => RuleTarget::Seeds,
            Self::Test(_) => RuleTarget::UnitTests,
            Self::Analysis(_) => RuleTarget::Analyses,
            Self::Snapshot(_) => RuleTarget::Snapshots,
            Self::HookNode(_) => RuleTarget::HookNodes,
            Self::SqlOperation(_) => RuleTarget::SqlOperations,
        }
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

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl Columnable for Node {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.get_base().columns.as_ref().map(|columns_map| {
            let column_names: Vec<&String> = columns_map.keys().collect();
            column_names
        })
    }

    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        self.get_base().columns.as_ref().map(|cols| {
            cols.iter()
                .filter_map(|(name, col)| col.description.as_ref().map(|desc| (name, desc)))
                .collect::<Vec<(&String, &String)>>()
        })
    }

    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
        self.get_base().columns.as_ref().map(|cols| {
            cols.iter()
                .filter_map(|(name, col)| {
                    col.data_type
                        .as_ref()
                        .or(col.datatype.as_ref())
                        .map(|dtype| (name, dtype))
                })
                .collect::<Vec<(&String, &String)>>()
        })
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl IncludeExcludable for Node {
    fn get_relative_path(&self) -> &String {
        &self.get_base().original_file_path
    }
}

impl IncludeExcludable for &Node {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
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

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
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

    fn get_relative_path(&self) -> Option<&String> {
        Some((*self).get_relative_path())
    }
}

impl Tagable for Node {
    fn get_tags(&self) -> Option<&Tags> {
        self.get_base().tags.as_ref()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_object_string(&self) -> &str {
        (*self).get_object_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some((*self).get_relative_path())
    }
}

impl HasCode for Node {
    fn get_code(&self) -> Option<&str> {
        match self {
            Self::Model(_) | Self::Snapshot(_) => self.get_base().raw_code.as_deref(),
            _ => unreachable!("MaxCodeLines can only be called on models and snapshots nodes"),
        }
    }

    fn get_name(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(&self.get_base().original_file_path)
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }
}

impl ChildMappable for Node {
    fn get_object_type(&self) -> &str {
        match self {
            Self::Model(_) | Self::Seed(_) => self.get_object_type(),
            _ => {
                unreachable!("IsNotOrphaned should only be called on models, seeds, and snapshots")
            }
        }
    }

    fn get_object_string(&self) -> &str {
        match self {
            Self::Model(_) | Self::Seed(_) => self.get_name(),
            _ => {
                unreachable!("IsNotOrphaned should only be called on models, seeds, and snapshots")
            }
        }
    }

    fn get_relative_path(&self) -> Option<&String> {
        match &self {
            Self::Model(_) | Self::Seed(_) => Some(&self.get_base().original_file_path),
            _ => None,
        }
    }

    fn get_childs<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str> {
        let unique_id = self.get_unique_id();
        manifest
            .child_map
            .get(unique_id)
            .map(|children| children.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

impl TestAble for Node {
    fn get_unique_id(&self) -> &String {
        self.get_unique_id()
    }

    fn get_object_string(&self) -> &String {
        self.get_name()
    }

    fn get_object_type(&self) -> String {
        self.get_object_type().to_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(&self.get_base().original_file_path)
    }
}

// Only for model this works though
impl ContractAble for Node {
    fn get_contract_enforced(&self) -> Option<bool> {
        match self {
            Self::Model(_) => self
                .get_base()
                .config
                .as_ref()
                .and_then(|cfg| cfg.contract.as_ref().map(|contract| contract.enforced)),
            // Nothing for other node types
            Self::Seed(_)
            | Self::Analysis(_)
            | Self::Test(_)
            | Self::Snapshot(_)
            | Self::HookNode(_)
            | Self::SqlOperation(_) => None,
        }
    }
    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_name(&self) -> &str {
        self.get_name()
    }
    fn get_relative_path(&self) -> Option<&String> {
        Some(&self.get_base().original_file_path)
    }
}

impl ContractAble for &Node {
    fn get_contract_enforced(&self) -> Option<bool> {
        (*self).get_contract_enforced()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_name(&self) -> &str {
        (*self).get_name()
    }
    fn get_relative_path(&self) -> Option<&String> {
        Some(&(*self).get_base().original_file_path)
    }
}

impl HasMetadata for Node {
    fn get_metadata(&self) -> Option<&Meta> {
        self.get_base().meta.as_ref()
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl CanReference for Node {
    fn get_depends_on_nodes(&self) -> &[String] {
        match self {
            Self::Model(_) | Self::Analysis(_) | Self::Snapshot(_) => {
                match &self.get_base().depends_on.nodes {
                    Some(nodes) => nodes,
                    None => &[],
                }
            }
            _ => unreachable!(
                "CanReference should only be called on models, analyses, and snapshots"
            ),
        }
    }
    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}
