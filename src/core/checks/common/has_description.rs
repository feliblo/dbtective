use crate::core::checks::RuleResult;
use crate::core::config::ManifestRule;
use crate::core::traits::Descriptable;

pub fn check_node_description<T: Descriptable>(descriptable: T, rule: &ManifestRule) -> RuleResult {
    let (passed, message) = match descriptable.description() {
        Some(desc) if !desc.trim().is_empty() => (true, String::new()),
        _ => (
            false,
            format!(
                "'{}' is missing a description.",
                descriptable.original_file_path()
            ),
        ),
    };

    RuleResult {
        severity: rule.severity.clone(),
        code: i32::from(!passed),
        node_type: descriptable.ruletarget().to_string(),
        message,
    }
}
