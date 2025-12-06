use super::node::{CompiledNodeFields, NodeBase};
use serde::Deserialize;
use std::borrow::Cow;
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Test {
    #[serde(flatten)]
    pub base: NodeBase,
    #[serde(flatten)]
    pub compiled: CompiledNodeFields,

    // GenericTest-specific fields (will be None for SingularTests)
    pub column_name: Option<String>,
    pub file_key_name: Option<String>,
    pub attached_node: Option<String>,
    pub test_metadata: Option<TestMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct TestMetadata {
    pub name: String,
    #[allow(dead_code)]
    pub kwargs: Option<serde_json::Value>,
    pub namespace: Option<String>,
}

impl Test {
    // This actually contains the type of test the test originally is
    // e.g. unique, not the name of the test as given by the user
    // Not the name the user has given it
    pub fn get_metadata_name(&self) -> Option<Cow<'_, str>> {
        let metadata = self.test_metadata.as_ref()?;

        Some(metadata.namespace.as_ref().map_or_else(
            || Cow::Borrowed(metadata.name.as_str()), // Borrowed if no namespace
            |ns| Cow::Owned(format!("{}::{}", ns, metadata.name)), // Owned if namespaced
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::manifest::dbt_objects::nodes::node::FileHash;

    impl Default for Test {
        fn default() -> Self {
            Self {
                base: NodeBase {
                    database: None,
                    schema: String::new(),
                    name: String::new(),
                    description: None,
                    package_name: String::new(),
                    path: String::new(),
                    original_file_path: String::new(),
                    unique_id: String::new(),
                    fqn: vec![],
                    alias: String::new(),
                    checksum: FileHash {
                        name: String::new(),
                        checksum: String::new(),
                    },
                    tags: None,
                    meta: None,
                    columns: None,
                    config: None,
                },
                compiled: CompiledNodeFields { language: None },
                column_name: None,
                file_key_name: None,
                attached_node: None,
                test_metadata: None,
            }
        }
    }
}
