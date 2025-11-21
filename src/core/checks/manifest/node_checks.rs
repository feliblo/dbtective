use crate::core::checks::common::has_description;
use crate::core::checks::RuleResult;
use crate::core::config::Config;
use crate::core::config::SpecificRuleConfig::HasDescription;
use crate::core::manifest::Manifest;
use crate::core::traits::Descriptable;

pub fn apply_node_checks(manifest: &Manifest, config: &Config) -> Vec<RuleResult> {
    let mut results = Vec::new();
    for rule in &config.manifest_tests {
        for node in manifest.nodes.values() {
            if rule
                .applies_to
                .as_ref()
                .unwrap()
                .contains(&node.ruletarget())
            {
                match rule.rule {
                    HasDescription {} => {
                        let result = has_description::check_node_description(node, rule);
                        results.push(result);
                    }
                }
            }
        }
    }
    results
}
