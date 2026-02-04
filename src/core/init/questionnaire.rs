use crate::core::config::catalog_rule::CatalogSpecificRuleConfig;
use crate::core::config::check_config_options::HasTagsCriteria;
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;
use crate::core::config::naming_convention::NamingConvention;
use inquire::{MultiSelect, Select};
use owo_colors::OwoColorize;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Yaml,
    Toml,
    Pyproject,
}

impl ConfigFormat {
    #[allow(dead_code)]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Yaml => "yml",
            Self::Toml => "toml",
            Self::Pyproject => "pyproject",
        }
    }
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml => write!(f, "YAML (recommended)"),
            Self::Toml => write!(f, "TOML"),
            Self::Pyproject => write!(f, "pyproject.toml"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataModel {
    Medallion,
    Common,
    None,
}

impl std::fmt::Display for DataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Medallion => write!(f, "Medallion (bronze, silver, gold)"),
            Self::Common => write!(f, "Common (staging, marts, intermediate)"),
            Self::None => write!(f, "None (no folder structure enforcement)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strictness {
    Basic,
    Standard,
    Strict,
}

impl std::fmt::Display for Strictness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic - Essential rules only"),
            Self::Standard => write!(f, "Standard (recommended) - Balanced set of rules"),
            Self::Strict => write!(f, "Strict - Comprehensive rule coverage"),
        }
    }
}

/// Trait for providing init-time descriptions of rule configs.
/// Adding a new variant to a rule config enum without updating this impl
/// will cause a compile error.
pub trait Init {
    fn init_description(&self) -> &'static str;
}

impl Init for ManifestSpecificRuleConfig {
    fn init_description(&self) -> &'static str {
        match self {
            Self::HasDescription { .. } => "has_description - Require descriptions for resources",
            Self::NameConvention { .. } => "name_convention - Enforce naming conventions",
            Self::HasTags { .. } => "has_tags - Require specific tags",
            Self::IsNotOrphaned { .. } => "is_not_orphaned - Check for orphaned sources",
            Self::HasUniqueTest { .. } => "has_unique_test - Require uniqueness tests",
            Self::HasContractEnforced { .. } => {
                "has_contract_enforced - Require enforced contracts"
            }
            Self::HasMetadataKeys { .. } => {
                "has_metadata_keys - Require metadata keys (e.g., owner)"
            }
            Self::HasRefs { .. } => "has_refs - Require use of ref() function",
            Self::MaxCodeLines { .. } => "max_code_lines - Limit code line count",
            Self::AllowedSubfolders { .. } => "allowed_subfolders - Restrict subfolder usage",
        }
    }
}

impl std::fmt::Display for ManifestSpecificRuleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.init_description())
    }
}

impl Init for CatalogSpecificRuleConfig {
    fn init_description(&self) -> &'static str {
        match self {
            Self::ColumnsAllDocumented { .. } => {
                "columns_all_documented - All columns must exist in docs"
            }
            Self::ColumnsHaveDescription { .. } => {
                "columns_have_description - All columns need descriptions"
            }
            Self::ColumnsNameConvention { .. } => {
                "columns_name_convention - Enforce column naming conventions"
            }
            Self::ColumnsCanonicalName { .. } => {
                "columns_canonical_name - Enforce canonical column names"
            }
        }
    }
}

impl std::fmt::Display for CatalogSpecificRuleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.init_description())
    }
}

#[derive(Debug, Clone)]
pub struct QuestionnaireResult {
    #[allow(dead_code)]
    pub format: ConfigFormat,
    pub naming_convention: NamingConvention,
    pub data_model: DataModel,
    pub manifest_rules: Vec<ManifestSpecificRuleConfig>,
    pub catalog_rules: Vec<CatalogSpecificRuleConfig>,
}

#[allow(clippy::too_many_lines)]
/// # Errors
/// - Returns: `QuestionnaireResult` on success, String error message on failure
pub fn run_questionnaire() -> Result<QuestionnaireResult, String> {
    println!("\n🕵️ {}", "Welcome to dbtective!".green().bold());
    println!(
        "{}\n",
        "Let's set up your configuration. (Don't worry, you can always change it later)".white()
    );

    // 1. Ask for config format
    let format_options = vec![
        ConfigFormat::Yaml,
        ConfigFormat::Toml,
        ConfigFormat::Pyproject,
    ];
    let format = Select::new("What config format do you want?", format_options)
        .with_starting_cursor(0) // Default to YAML (recommended)
        .prompt()
        .map_err(|e| format!("Failed to get config format: {e}"))?;

    // 2. Ask for naming convention
    let convention_options = vec![
        "snake_case (recommended)",
        "kebab-case",
        "camelCase",
        "PascalCase",
    ];
    let naming_convention_str =
        Select::new("What naming convention do you use?", convention_options)
            .with_starting_cursor(0) // Default to snake_case (recommended)
            .prompt()
            .map_err(|e| format!("Failed to get naming convention: {e}"))?;

    #[allow(clippy::match_same_arms)]
    let naming_convention = match naming_convention_str {
        "snake_case (recommended)" => NamingConvention::from_pattern("snake_case"),
        "kebab-case" => NamingConvention::from_pattern("kebab-case"),
        "camelCase" => NamingConvention::from_pattern("camelCase"),
        "PascalCase" => NamingConvention::from_pattern("PascalCase"),
        _ => NamingConvention::from_pattern("snake_case"),
    }
    .map_err(|e| format!("Invalid naming convention: {e}"))?;

    // 3. Ask for data model
    let data_model_options = vec![DataModel::None, DataModel::Common, DataModel::Medallion];
    let data_model = Select::new(
        "What data model structure are you using?",
        data_model_options,
    )
    .with_starting_cursor(1) // Default to Common
    .prompt()
    .map_err(|e| format!("Failed to get data model: {e}"))?;

    // 4. Ask for strictness level
    let strictness_options = vec![Strictness::Basic, Strictness::Standard, Strictness::Strict];
    let strictness = Select::new("How strict do you want the rules?", strictness_options)
        .with_starting_cursor(1) // Default to Standard (recommended)
        .prompt()
        .map_err(|e| format!("Failed to get strictness level: {e}"))?;

    let (mut manifest_rules, mut catalog_rules) = match strictness {
        Strictness::Basic => (
            vec![
                ManifestSpecificRuleConfig::HasDescription {},
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
            ],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
            }],
        ),
        Strictness::Standard => (
            vec![
                ManifestSpecificRuleConfig::HasDescription {},
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::HasMetadataKeys {
                    required_keys: vec![],
                    custom_message: None,
                },
                ManifestSpecificRuleConfig::HasRefs {},
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
            ],
        ),
        Strictness::Strict => (
            vec![
                ManifestSpecificRuleConfig::HasDescription {},
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::HasMetadataKeys {
                    required_keys: vec![],
                    custom_message: None,
                },
                ManifestSpecificRuleConfig::HasRefs {},
                ManifestSpecificRuleConfig::HasTags {
                    required_tags: vec![],
                    criteria: HasTagsCriteria::default(),
                },
                ManifestSpecificRuleConfig::HasUniqueTest {
                    allowed_test_names: vec![],
                },
                ManifestSpecificRuleConfig::HasContractEnforced {},
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
                CatalogSpecificRuleConfig::ColumnsAllDocumented {},
            ],
        ),
    };

    // Add allowed_subfolders if a data model structure was selected
    if data_model != DataModel::None {
        manifest_rules.push(ManifestSpecificRuleConfig::AllowedSubfolders {
            allowed_subfolders: vec![],
            path_prefix: None,
            path_postfix: None,
        });
    }

    // 5. Ask for additional manifest rules
    let available_manifest_rules: Vec<_> = ManifestSpecificRuleConfig::iter()
        .filter(|r| {
            !manifest_rules
                .iter()
                .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(r))
        })
        .collect();

    if !available_manifest_rules.is_empty() {
        let selected_manifest = MultiSelect::new(
            "Select additional manifest rules (optional). These will provide examples for you to use:",
            available_manifest_rules,
        )
        .prompt()
        .map_err(|e| format!("Failed to get additional manifest rules: {e}"))?;

        for rule in selected_manifest {
            manifest_rules.push(rule);
        }
    }

    // 6. Ask for additional catalog rules
    let available_catalog_rules: Vec<_> = CatalogSpecificRuleConfig::iter()
        .filter(|r| {
            !catalog_rules
                .iter()
                .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(r))
        })
        .collect();

    if !available_catalog_rules.is_empty() {
        let selected_catalog = MultiSelect::new(
            "Select additional catalog rules (optional). These will proivide examples for you to use.",
            available_catalog_rules,
        )
        .prompt()
        .map_err(|e| format!("Failed to get additional catalog rules: {e}"))?;

        for rule in selected_catalog {
            catalog_rules.push(rule);
        }
    }

    Ok(QuestionnaireResult {
        format,
        naming_convention,
        data_model,
        manifest_rules,
        catalog_rules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    // ========================================================================
    // Enum Display Tests
    // ========================================================================

    #[test]
    fn test_config_format_display() {
        assert_eq!(ConfigFormat::Yaml.to_string(), "YAML (recommended)");
        assert_eq!(ConfigFormat::Toml.to_string(), "TOML");
        assert_eq!(ConfigFormat::Pyproject.to_string(), "pyproject.toml");
    }

    #[test]
    fn test_data_model_display() {
        assert_eq!(
            DataModel::Medallion.to_string(),
            "Medallion (bronze, silver, gold)"
        );
        assert_eq!(
            DataModel::Common.to_string(),
            "Common (staging, marts, intermediate)"
        );
        assert_eq!(
            DataModel::None.to_string(),
            "None (no folder structure enforcement)"
        );
    }

    #[test]
    fn test_strictness_display() {
        assert_eq!(
            Strictness::Basic.to_string(),
            "Basic - Essential rules only"
        );
        assert_eq!(
            Strictness::Standard.to_string(),
            "Standard (recommended) - Balanced set of rules"
        );
        assert_eq!(
            Strictness::Strict.to_string(),
            "Strict - Comprehensive rule coverage"
        );
    }

    // ========================================================================
    // Init Trait Tests
    // ========================================================================

    #[test]
    fn test_manifest_rule_init_description() {
        let descriptions: Vec<_> = ManifestSpecificRuleConfig::iter()
            .map(|r| r.init_description())
            .collect();
        assert!(descriptions.contains(&"has_description - Require descriptions for resources"));
        assert!(descriptions.contains(&"name_convention - Enforce naming conventions"));
        assert!(descriptions.contains(&"has_tags - Require specific tags"));
        assert!(descriptions.contains(&"is_not_orphaned - Check for orphaned sources"));
        assert!(descriptions.contains(&"has_unique_test - Require uniqueness tests"));
        assert!(descriptions.contains(&"has_contract_enforced - Require enforced contracts"));
        assert!(descriptions.contains(&"has_metadata_keys - Require metadata keys (e.g., owner)"));
        assert!(descriptions.contains(&"has_refs - Require use of ref() function"));
        assert!(descriptions.contains(&"max_code_lines - Limit code line count"));
        assert!(descriptions.contains(&"allowed_subfolders - Restrict subfolder usage"));
    }

    #[test]
    fn test_catalog_rule_init_description() {
        let descriptions: Vec<_> = CatalogSpecificRuleConfig::iter()
            .map(|r| r.init_description())
            .collect();
        assert!(descriptions.contains(&"columns_all_documented - All columns must exist in docs"));
        assert!(descriptions.contains(&"columns_have_description - All columns need descriptions"));
        assert!(
            descriptions.contains(&"columns_name_convention - Enforce column naming conventions")
        );
        assert!(descriptions.contains(&"columns_canonical_name - Enforce canonical column names"));
    }

    // ========================================================================
    // Enum Iterator Tests
    // ========================================================================

    #[test]
    fn test_manifest_rule_iter_count() {
        assert_eq!(ManifestSpecificRuleConfig::iter().count(), 10);
    }

    #[test]
    fn test_catalog_rule_iter_count() {
        assert_eq!(CatalogSpecificRuleConfig::iter().count(), 4);
    }
}
