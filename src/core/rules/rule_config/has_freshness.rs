use crate::cli::table::RuleResult;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

pub trait SourcesHaveFreshness: Identifiable {
    fn has_freshness_configured(&self) -> bool;
}

pub fn has_freshness<T: SourcesHaveFreshness>(
    source: &T,
    rule: &ManifestRule,
) -> Option<RuleResult> {
    if source.has_freshness_configured() {
        None
    } else {
        Some(RuleResult::new(
            &rule.severity,
            source.get_object_type(),
            rule.get_name(),
            format!(
                "{} is missing freshness configuration.",
                source.get_object_string()
            ),
            source.get_problematic_path(false).map(str::to_owned),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    use super::*;

    struct TestSource {
        name: String,
        has_freshness: bool,
    }
    impl Identifiable for TestSource {
        #[allow(clippy::unnecessary_literal_bound)]
        fn get_object_type(&self) -> &str {
            "Source"
        }
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
            None
        }
        fn get_object_string(&self) -> &str {
            &self.name
        }
    }
    impl SourcesHaveFreshness for TestSource {
        fn has_freshness_configured(&self) -> bool {
            self.has_freshness
        }
    }

    #[test]
    fn test_source_with_freshness_passes() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveFreshness {},
            Severity::Error,
        );
        let source = TestSource {
            name: "raw_customers".to_string(),
            has_freshness: true,
        };
        assert_eq!(has_freshness(&source, &rule), None);
    }

    #[test]
    fn test_source_without_freshness_fails() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveFreshness {},
            Severity::Error,
        );
        let source = TestSource {
            name: "raw_customers".to_string(),
            has_freshness: false,
        };
        assert_eq!(
            has_freshness(&source, &rule),
            Some(RuleResult::new(
                &Severity::Error,
                "Source",
                "sources_have_freshness",
                "raw_customers is missing freshness configuration.",
                None,
            ))
        );
    }

    #[test]
    fn test_source_without_freshness_warning_severity() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveFreshness {},
            Severity::Warning,
        );
        let source = TestSource {
            name: "raw_orders".to_string(),
            has_freshness: false,
        };
        assert_eq!(
            has_freshness(&source, &rule),
            Some(RuleResult::new(
                &Severity::Warning,
                "Source",
                "sources_have_freshness",
                "raw_orders is missing freshness configuration.",
                None,
            ))
        );
    }
}
