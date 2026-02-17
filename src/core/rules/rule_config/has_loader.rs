use crate::cli::table::RuleResult;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

pub trait SourcesHaveLoader: Identifiable {
    fn loader(&self) -> Option<&String>;
}

pub fn has_loader<T: SourcesHaveLoader>(loadable: &T, rule: &ManifestRule) -> Option<RuleResult> {
    match loadable.loader() {
        Some(loader) if !loader.trim().is_empty() => None,
        _ => Some(RuleResult::new(
            &rule.severity,
            loadable.get_object_type(),
            rule.get_name(),
            format!("{} is missing a loader.", loadable.get_object_string()),
            loadable.get_problematic_path(false).map(str::to_owned),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    use super::*;

    struct TestSource {
        name: String,
        loader: Option<String>,
    }
    impl Identifiable for TestSource {
        #[allow(clippy::unnecessary_literal_bound)]
        fn get_object_type(&self) -> &str {
            "Source"
        }
        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            None
        }
        fn get_object_string(&self) -> &str {
            &self.name
        }
    }
    impl SourcesHaveLoader for TestSource {
        fn loader(&self) -> Option<&String> {
            self.loader.as_ref()
        }
    }

    #[test]
    fn test_source_with_loader_passes() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveLoader {},
            Severity::Error,
        );
        let source = TestSource {
            name: "raw_customers".to_string(),
            loader: Some("fivetran".to_string()),
        };
        assert_eq!(has_loader(&source, &rule), None);
    }

    #[test]
    fn test_source_without_loader_fails() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveLoader {},
            Severity::Error,
        );
        let source = TestSource {
            name: "raw_customers".to_string(),
            loader: None,
        };
        assert_eq!(
            has_loader(&source, &rule),
            Some(RuleResult::new(
                &Severity::Error,
                "Source",
                "sources_have_loader",
                "raw_customers is missing a loader.",
                None,
            ))
        );
    }

    #[test]
    fn test_source_with_empty_loader_fails() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveLoader {},
            Severity::Warning,
        );
        let source = TestSource {
            name: "raw_orders".to_string(),
            loader: Some(String::new()),
        };
        assert_eq!(
            has_loader(&source, &rule),
            Some(RuleResult::new(
                &Severity::Warning,
                "Source",
                "sources_have_loader",
                "raw_orders is missing a loader.",
                None,
            ))
        );
    }

    #[test]
    fn test_source_with_whitespace_loader_fails() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::SourcesHaveLoader {},
            Severity::Error,
        );
        let source = TestSource {
            name: "raw_payments".to_string(),
            loader: Some("   ".to_string()),
        };
        assert!(has_loader(&source, &rule).is_some());
    }
}
