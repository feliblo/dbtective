// Extension traits for individual node types (Model, Snapshot, Analysis, etc.)
// These allow the Node enum to delegate to specific implementations
// We use extension traits because Rust's orphan rules prevent adding inherent impls
// to types defined in another crate (dbt_artifact_parser)

use dbt_artifact_parser::manifest::nodes::{Analysis, Model, Snapshot};

// Extension trait for Model
pub trait ModelExt {
    fn get_depends_on_nodes(&self) -> &[String];
    fn get_raw_code(&self) -> Option<&str>;
    fn get_contract_enforced(&self) -> Option<bool>;
}

impl ModelExt for Model {
    fn get_depends_on_nodes(&self) -> &[String] {
        self.base.depends_on.nodes.as_deref().unwrap_or(&[])
    }

    fn get_raw_code(&self) -> Option<&str> {
        self.base.raw_code.as_deref()
    }

    fn get_contract_enforced(&self) -> Option<bool> {
        self.base
            .config
            .as_ref()
            .and_then(|cfg| cfg.contract.as_ref().map(|contract| contract.enforced))
    }
}

// Extension trait for Snapshot
pub trait SnapshotExt {
    fn get_depends_on_nodes(&self) -> &[String];
    fn get_raw_code(&self) -> Option<&str>;
}

impl SnapshotExt for Snapshot {
    fn get_depends_on_nodes(&self) -> &[String] {
        self.base.depends_on.nodes.as_deref().unwrap_or(&[])
    }

    fn get_raw_code(&self) -> Option<&str> {
        self.base.raw_code.as_deref()
    }
}

// Extension trait for Analysis
pub trait AnalysisExt {
    fn get_depends_on_nodes(&self) -> &[String];
}

impl AnalysisExt for Analysis {
    fn get_depends_on_nodes(&self) -> &[String] {
        self.base.depends_on.nodes.as_deref().unwrap_or(&[])
    }
}
