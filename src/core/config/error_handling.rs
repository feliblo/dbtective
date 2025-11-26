use crate::core::config::parse_config::SpecificRuleConfig;
use strsim::levenshtein;

pub fn handle_config_error(err: &serde_yaml::Error) -> anyhow::Error {
    let msg = &err.to_string();
    if msg.contains("manifest_tests[") {
        if let Some(start) = msg.find("unknown variant `") {
            let end = msg[start + 17..].find('`').unwrap_or(0) + start + 17;
            let invalid_rule = &msg[start + 17..end];

            let valid_rules = SpecificRuleConfig::get_all_as_str();
            let maybe_meant = valid_rules
                .iter()
                .min_by_key(|rule| levenshtein(invalid_rule, rule))
                .cloned()
                .unwrap_or_default();

            if !maybe_meant.is_empty() {
                return anyhow::anyhow!(
                    "Configuration error: Unknown rule type '{invalid_rule}'. Maybe you meant '{maybe_meant}'?"
                );
            }
        }
    }
    anyhow::anyhow!("Error in configuration: {msg}")
}
