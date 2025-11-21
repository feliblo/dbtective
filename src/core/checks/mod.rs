pub mod common;
pub mod manifest;
use crate::core::config::Severity;

pub struct RuleResult {
    pub severity: Severity,
    pub code: i32,
    pub node_type: String,
    pub message: String,
}
