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

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::manifest::nodes::node::{CompiledNodeFields, DependsOn, NodeBase};

    fn make_model(raw_code: Option<&str>, nodes: &str, contract: Option<bool>) -> Model {
        let raw_code_json = raw_code
            .map(|c| format!(r#""raw_code": "{c}","#))
            .unwrap_or_default();
        let contract_json = contract
            .map(|enforced| {
                format!(
                    r#""config": {{"contract": {{"enforced": {enforced}, "alias_types": false}}}},"#
                )
            })
            .unwrap_or_default();
        let json = format!(
            r#"{{
                "resource_type": "model",
                "name": "test_model",
                "package_name": "proj",
                "path": "test.sql",
                "original_file_path": "models/test.sql",
                "unique_id": "model.proj.test_model",
                "fqn": ["proj", "test_model"],
                "alias": "test_model",
                "checksum": {{"name": "sha256", "checksum": "abc"}},
                "schema": "public",
                {raw_code_json}
                {contract_json}
                "depends_on": {{"nodes": {nodes}}}
            }}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn test_model_depends_on_some() {
        let m = make_model(None, r#"["model.proj.a"]"#, None);
        assert_eq!(m.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_model_depends_on_none() {
        let m = make_model(None, "[]", None);
        assert!(m.get_depends_on_nodes().is_empty());
    }

    #[test]
    fn test_model_raw_code_some() {
        let m = make_model(Some("SELECT 1"), "[]", None);
        assert_eq!(m.get_raw_code(), Some("SELECT 1"));
    }

    #[test]
    fn test_model_raw_code_none() {
        let m = make_model(None, "[]", None);
        assert!(m.get_raw_code().is_none());
    }

    #[test]
    fn test_model_contract_enforced() {
        let m = make_model(None, "[]", Some(true));
        assert_eq!(m.get_contract_enforced(), Some(true));
    }

    #[test]
    fn test_model_contract_not_enforced() {
        let m = make_model(None, "[]", Some(false));
        assert_eq!(m.get_contract_enforced(), Some(false));
    }

    #[test]
    fn test_model_contract_none() {
        let m = make_model(None, "[]", None);
        assert!(m.get_contract_enforced().is_none());
    }

    #[test]
    fn test_snapshot_depends_on_some() {
        let s = Snapshot {
            base: NodeBase {
                depends_on: DependsOn {
                    nodes: Some(vec!["model.proj.a".to_string()]),
                    macros: None,
                },
                ..Default::default()
            },
            compiled: CompiledNodeFields::default(),
        };
        assert_eq!(s.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_snapshot_depends_on_none() {
        let s = Snapshot {
            base: NodeBase::default(),
            compiled: CompiledNodeFields::default(),
        };
        assert!(s.get_depends_on_nodes().is_empty());
    }

    #[test]
    fn test_snapshot_raw_code() {
        let s = Snapshot {
            base: NodeBase {
                raw_code: Some("SELECT *".to_string()),
                ..Default::default()
            },
            compiled: CompiledNodeFields::default(),
        };
        assert_eq!(s.get_raw_code(), Some("SELECT *"));
    }

    #[test]
    fn test_snapshot_raw_code_none() {
        let s = Snapshot {
            base: NodeBase::default(),
            compiled: CompiledNodeFields::default(),
        };
        assert!(s.get_raw_code().is_none());
    }

    #[test]
    fn test_analysis_depends_on_some() {
        let a = Analysis {
            base: NodeBase {
                depends_on: DependsOn {
                    nodes: Some(vec!["model.proj.b".to_string()]),
                    macros: None,
                },
                ..Default::default()
            },
            compiled: CompiledNodeFields::default(),
        };
        assert_eq!(a.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_analysis_depends_on_none() {
        let a = Analysis {
            base: NodeBase::default(),
            compiled: CompiledNodeFields::default(),
        };
        assert!(a.get_depends_on_nodes().is_empty());
    }
}
