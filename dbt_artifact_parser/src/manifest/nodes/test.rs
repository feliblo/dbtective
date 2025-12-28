use super::node::{CompiledNodeFields, NodeBase};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize, Default)]
pub struct TestMetadata {
    #[serde(default)]
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
