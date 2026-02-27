use std::fmt::Write;

use super::questionnaire::{
    ConfigFormat, DataModelMethodology, LayeringStrategy, QuestionnaireResult,
};
use crate::core::config::catalog_rule::CatalogSpecificRuleConfig;
use crate::core::config::check_config_options::HasTagsCriteria;
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;
use crate::core::config::naming_convention::NamingConvention;
use crate::core::config::rule_category::RuleCategory;
use strum::IntoEnumIterator;

/// Convert a `snake_case` base like `t_lnk` into `camelCase` `tLnk`
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert a `snake_case` base like `t_lnk` into `PascalCase` `TLnk`
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

#[derive(Debug)]
pub struct InitConfig {
    pub format: ConfigFormat,
    pub manifest_rules: Vec<ManifestSpecificRuleConfig>,
    pub catalog_rules: Vec<CatalogSpecificRuleConfig>,
    pub layering_strategy: LayeringStrategy,
    pub methodology: DataModelMethodology,
    pub naming_convention: NamingConvention,
    pub auto_parse_command: Option<String>,
}

impl InitConfig {
    pub fn from_questionnaire(result: &QuestionnaireResult) -> Self {
        let manifest_rules = Self::build_manifest_rules(result);
        let catalog_rules = Self::build_catalog_rules(result);

        Self {
            format: result.format,
            manifest_rules,
            catalog_rules,
            layering_strategy: result.layering_strategy,
            methodology: result.methodology,
            naming_convention: result.naming_convention.clone(),
            auto_parse_command: result.auto_parse_command.clone(),
        }
    }

    fn build_manifest_rules(result: &QuestionnaireResult) -> Vec<ManifestSpecificRuleConfig> {
        ManifestSpecificRuleConfig::iter()
            .filter(|rule| {
                result
                    .manifest_rules
                    .iter()
                    .any(|r| std::mem::discriminant(r) == std::mem::discriminant(rule))
            })
            .map(|rule| Self::create_manifest_rule(&rule, result))
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    fn create_manifest_rule(
        rule: &ManifestSpecificRuleConfig,
        result: &QuestionnaireResult,
    ) -> ManifestSpecificRuleConfig {
        match rule {
            ManifestSpecificRuleConfig::AllowedSubfolders { .. } => {
                let subfolders = match result.layering_strategy {
                    LayeringStrategy::Medallion => vec![
                        "bronze".to_string(),
                        "silver".to_string(),
                        "gold".to_string(),
                    ],
                    LayeringStrategy::Common => vec![
                        "staging".to_string(),
                        "warehouse".to_string(),
                        "marts".to_string(),
                    ],
                    LayeringStrategy::None => vec![],
                };
                ManifestSpecificRuleConfig::AllowedSubfolders {
                    allowed_subfolders: subfolders,
                    path_prefix: None,
                    path_postfix: None,
                }
            }
            ManifestSpecificRuleConfig::CodeContainsRefs { .. } => {
                ManifestSpecificRuleConfig::CodeContainsRefs {}
            }
            ManifestSpecificRuleConfig::HasContractEnforced { .. } => {
                ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }
            }
            ManifestSpecificRuleConfig::HasDescription { .. } => {
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                }
            }
            ManifestSpecificRuleConfig::CodeForbiddenPatterns { .. } => {
                let mut patterns = vec!["SELECT *".to_string()];
                match result.layering_strategy {
                    LayeringStrategy::Medallion => {
                        patterns.extend([
                            "bronze.".to_string(),
                            "silver.".to_string(),
                            "gold.".to_string(),
                        ]);
                    }
                    LayeringStrategy::Common => {
                        patterns.extend([
                            "staging.".to_string(),
                            "warehouse.".to_string(),
                            "marts.".to_string(),
                        ]);
                    }
                    LayeringStrategy::None => {}
                }
                ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                    forbidden_patterns: patterns,
                    case_sensitive: false,
                }
            }
            ManifestSpecificRuleConfig::HasMetadataKeys { .. } => {
                ManifestSpecificRuleConfig::HasMetadataKeys {
                    required_keys: vec!["owner".to_string()],
                    custom_message: None,
                }
            }
            ManifestSpecificRuleConfig::HasRefs { .. } => ManifestSpecificRuleConfig::HasRefs {},
            ManifestSpecificRuleConfig::HasTags { .. } => ManifestSpecificRuleConfig::HasTags {
                required_tags: vec![
                    "daily".to_string(),
                    "weekly".to_string(),
                    "monthly".to_string(),
                    "yearly".to_string(),
                    "inactive".to_string(),
                ],
                criteria: HasTagsCriteria::OneOf,
            },
            ManifestSpecificRuleConfig::HasUniqueTest { .. } => {
                ManifestSpecificRuleConfig::HasUniqueTest {
                    allowed_test_names: vec![],
                }
            }
            ManifestSpecificRuleConfig::IsNotOrphaned { .. } => {
                ManifestSpecificRuleConfig::IsNotOrphaned {
                    allowed_references: vec![],
                }
            }
            ManifestSpecificRuleConfig::CodeMaxLines { .. } => {
                ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 200 }
            }
            ManifestSpecificRuleConfig::MaxDownstreamDependencies { .. } => {
                ManifestSpecificRuleConfig::MaxDownstreamDependencies {
                    max_downstream: 5,
                    exclude_types: vec![],
                }
            }
            ManifestSpecificRuleConfig::CodeMaxJoins { .. } => {
                ManifestSpecificRuleConfig::CodeMaxJoins { max_joins: 5 }
            }
            ManifestSpecificRuleConfig::CodeNoHardcodedRefs { .. } => {
                ManifestSpecificRuleConfig::CodeNoHardcodedRefs {}
            }
            ManifestSpecificRuleConfig::MaxUpstreamDependencies { .. } => {
                ManifestSpecificRuleConfig::MaxUpstreamDependencies {
                    max_upstream: 5,
                    exclude_types: vec![],
                }
            }
            ManifestSpecificRuleConfig::NameConvention { .. } => {
                ManifestSpecificRuleConfig::NameConvention {
                    convention: result.naming_convention.clone(),
                }
            }
            ManifestSpecificRuleConfig::SourcesHaveFreshness { .. } => {
                ManifestSpecificRuleConfig::SourcesHaveFreshness {}
            }
            ManifestSpecificRuleConfig::SourcesHaveLoader { .. } => {
                ManifestSpecificRuleConfig::SourcesHaveLoader {}
            }
        }
    }

    fn build_catalog_rules(result: &QuestionnaireResult) -> Vec<CatalogSpecificRuleConfig> {
        CatalogSpecificRuleConfig::iter()
            .filter(|rule| {
                result
                    .catalog_rules
                    .iter()
                    .any(|r| std::mem::discriminant(r) == std::mem::discriminant(rule))
            })
            .map(|rule| Self::create_catalog_rule(&rule, result))
            .collect()
    }

    fn create_catalog_rule(
        rule: &CatalogSpecificRuleConfig,
        result: &QuestionnaireResult,
    ) -> CatalogSpecificRuleConfig {
        match rule {
            CatalogSpecificRuleConfig::ColumnsAllDocumented { .. } => {
                CatalogSpecificRuleConfig::ColumnsAllDocumented {}
            }
            CatalogSpecificRuleConfig::ColumnsCanonicalName { .. } => {
                CatalogSpecificRuleConfig::ColumnsCanonicalName {
                    canonical: "user_id".to_string(),
                    invalid_names: vec![
                        crate::core::config::check_config_options::ColumnNamePattern::Literal(
                            "userid".to_string(),
                        ),
                        crate::core::config::check_config_options::ColumnNamePattern::Literal(
                            "UserId".to_string(),
                        ),
                    ],
                    exceptions: None,
                }
            }
            CatalogSpecificRuleConfig::ColumnsHaveDataType { .. } => {
                CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage: None }
            }
            CatalogSpecificRuleConfig::ColumnsHaveDescription { .. } => {
                CatalogSpecificRuleConfig::ColumnsHaveDescription {}
            }
            CatalogSpecificRuleConfig::ColumnsNameConvention { .. } => {
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: result.naming_convention.clone(),
                    data_types: None,
                    use_database_columns: true,
                }
            }
        }
    }

    /// Returns (`final_layer`, `middle_layer`) names based on layering strategy
    const fn layer_names(&self) -> (&str, &str) {
        match self.layering_strategy {
            LayeringStrategy::Medallion => ("gold", "silver"),
            LayeringStrategy::Common | LayeringStrategy::None => ("marts", "warehouse"),
        }
    }

    /// Convert a methodology prefix (e.g. "dim", "fact", "hub") into the
    /// regex pattern and display form matching the user's naming convention.
    ///
    /// Returns `(pattern, display)` — e.g. `("^dim_", "dim_")` for `snake_case`,
    /// `("^dim-", "dim-")` for `kebab-case`, `("^dim[A-Z]", "dim")` for `camelCase`,
    /// `("^Dim[A-Z]", "Dim")` for `PascalCase`.
    fn prefix_for_convention(&self, base: &str) -> (String, String) {
        match self.naming_convention.name() {
            "kebab-case" => {
                let kebab = base.replace('_', "-");
                (format!("^{kebab}-"), format!("{kebab}-"))
            }
            "camelCase" => {
                let camel = to_camel_case(base);
                (format!("^{camel}[A-Z]"), camel)
            }
            "PascalCase" => {
                let pascal = to_pascal_case(base);
                (format!("^{pascal}[A-Z]"), pascal)
            }
            // snake_case and custom regex — fall back to snake_case style
            _ => (format!("^{base}_"), format!("{base}_")),
        }
    }

    /// Category order for manifest rule section banners in generated config files
    const MANIFEST_CATEGORIES_ORDER: &'static [RuleCategory] = &[
        RuleCategory::Documentation,
        RuleCategory::Naming,
        RuleCategory::Governance,
        RuleCategory::Testing,
        RuleCategory::Structure,
        RuleCategory::Performance,
    ];

    /// Category order for catalog rule section banners in generated config files
    const CATALOG_CATEGORIES_ORDER: &'static [RuleCategory] =
        &[RuleCategory::Documentation, RuleCategory::Naming];

    /// Generate a section banner comment
    fn section_banner(name: &str, indent: &str) -> String {
        let inner = format!("## {name} ##");
        let border = "#".repeat(inner.len());
        format!("{indent}{border}\n{indent}{inner}\n{indent}{border}\n")
    }

    /// Generate methodology-specific YAML rule blocks
    fn methodology_rules_yaml(&self) -> Vec<String> {
        match self.methodology {
            DataModelMethodology::Kimball => self.kimball_rules_yaml(),
            DataModelMethodology::Inmon => self.inmon_rules_yaml(),
            DataModelMethodology::DataVault => self.data_vault_rules_yaml(),
            DataModelMethodology::None => vec![],
        }
    }

    fn kimball_rules_yaml(&self) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        let (dim_pattern, dim_display) = self.prefix_for_convention("dim");
        let (fact_pattern, fact_display) = self.prefix_for_convention("fact");
        match self.layering_strategy {
            // Medallion: dims, facts, and marts all live under the final layer (gold)
            LayeringStrategy::Medallion => vec![
                format!(
                    r#"  - name: "{final_layer}_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["dimensions", "facts", "marts"]
    includes: ["models/{final_layer}"]
    description: "The {final_layer} layer must contain dimensions, facts, and marts subfolders""#
                ),
                format!(
                    r#"  - name: "dimension_naming"
    type: "name_convention"
    pattern: "{dim_pattern}"
    includes: ["models/{final_layer}/dimensions"]
    description: "Dimension models must start with {dim_display}""#
                ),
                format!(
                    r#"  - name: "fact_naming"
    type: "name_convention"
    pattern: "{fact_pattern}"
    includes: ["models/{final_layer}/facts"]
    description: "Fact models must start with {fact_display}""#
                ),
            ],
            // Common: dims and facts live in the middle layer (warehouse),
            // marts is a separate final layer
            LayeringStrategy::Common => vec![
                format!(
                    r#"  - name: "{middle_layer}_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["dimensions", "facts"]
    includes: ["models/{middle_layer}"]
    description: "The {middle_layer} layer must contain dimensions and facts subfolders""#
                ),
                format!(
                    r#"  - name: "dimension_naming"
    type: "name_convention"
    pattern: "{dim_pattern}"
    includes: ["models/{middle_layer}/dimensions"]
    description: "Dimension models must start with {dim_display}""#
                ),
                format!(
                    r#"  - name: "fact_naming"
    type: "name_convention"
    pattern: "{fact_pattern}"
    includes: ["models/{middle_layer}/facts"]
    description: "Fact models must start with {fact_display}""#
                ),
            ],
            LayeringStrategy::None => vec![],
        }
    }

    fn inmon_rules_yaml(&self) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        vec![
            format!(
                r#"  - name: "{middle_layer}_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["core", "reference", "bridge"]
    includes: ["models/{middle_layer}"]
    description: "The {middle_layer} layer must contain normalized enterprise model components""#
            ),
            format!(
                r#"  - name: "{final_layer}_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["finance", "marketing", "operations"]
    includes: ["models/{final_layer}"]
    description: "The {final_layer} layer contains departmental data marts (customize these)""#
            ),
            format!(
                r#"  - name: "enterprise_model_contracts"
    type: "has_contract_enforced"
    includes: ["models/{middle_layer}"]
    description: "Normalized enterprise model tables must have enforced contracts""#
            ),
            format!(
                r#"  - name: "enterprise_model_unique_keys"
    type: "has_unique_test"
    includes: ["models/{middle_layer}"]
    description: "All normalized tables must have primary key uniqueness tests""#
            ),
        ]
    }

    #[allow(clippy::too_many_lines)]
    fn data_vault_rules_yaml(&self) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        let (hub_pattern, hub_display) = self.prefix_for_convention("hub");
        let (lnk_pattern, lnk_display) = self.prefix_for_convention("lnk");
        let (sat_pattern, sat_display) = self.prefix_for_convention("sat");
        let (t_lnk_pattern, t_lnk_display) = self.prefix_for_convention("t_lnk");
        let (dim_pattern, dim_display) = self.prefix_for_convention("dim");
        let (fact_pattern, fact_display) = self.prefix_for_convention("fact");
        vec![
            format!(
                r#"  - name: "{middle_layer}_vault_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["hubs", "links", "satellites", "t_links"]
    includes: ["models/{middle_layer}"]
    description: "The {middle_layer} layer must contain Data Vault components""#
            ),
            format!(
                r#"  - name: "{final_layer}_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: ["business_vault", "information_mart"]
    includes: ["models/{final_layer}"]
    description: "The {final_layer} layer contains business vault and information marts""#
            ),
            format!(
                r#"  - name: "hub_naming"
    type: "name_convention"
    pattern: "{hub_pattern}"
    includes: ["models/{middle_layer}/hubs"]
    description: "Hub models must start with {hub_display}""#
            ),
            format!(
                r#"  - name: "link_naming"
    type: "name_convention"
    pattern: "{lnk_pattern}"
    includes: ["models/{middle_layer}/links"]
    description: "Link models must start with {lnk_display}""#
            ),
            format!(
                r#"  - name: "satellite_naming"
    type: "name_convention"
    pattern: "{sat_pattern}"
    includes: ["models/{middle_layer}/satellites"]
    description: "Satellite models must start with {sat_display}""#
            ),
            format!(
                r#"  - name: "t_link_naming"
    type: "name_convention"
    pattern: "{t_lnk_pattern}"
    includes: ["models/{middle_layer}/t_links"]
    description: "Transactional link models must start with {t_lnk_display}""#
            ),
            format!(
                r#"  - name: "vault_contracts"
    type: "has_contract_enforced"
    includes: ["models/{middle_layer}"]
    description: "All vault tables must have enforced contracts""#
            ),
            format!(
                r#"  - name: "hub_unique_keys"
    type: "has_unique_test"
    includes: ["models/{middle_layer}/hubs"]
    description: "All hubs must have a uniqueness test on the hash key""#
            ),
            format!(
                r#"  - name: "link_unique_keys"
    type: "has_unique_test"
    includes: ["models/{middle_layer}/links"]
    description: "All links must have a uniqueness test on the hash key""#
            ),
            format!(
                r#"  - name: "info_mart_dim_naming"
    type: "name_convention"
    pattern: "{dim_pattern}"
    includes: ["models/{final_layer}/information_mart"]
    description: "Information mart dimension models must start with {dim_display}""#
            ),
            format!(
                r#"  - name: "info_mart_fact_naming"
    type: "name_convention"
    pattern: "{fact_pattern}"
    includes: ["models/{final_layer}/information_mart"]
    description: "Information mart fact models must start with {fact_display}""#
            ),
        ]
    }

    /// Generate methodology-specific TOML rule blocks
    fn methodology_rules_toml(&self, section: &str) -> Vec<String> {
        match self.methodology {
            DataModelMethodology::Kimball => self.kimball_rules_toml(section),
            DataModelMethodology::Inmon => self.inmon_rules_toml(section),
            DataModelMethodology::DataVault => self.data_vault_rules_toml(section),
            DataModelMethodology::None => vec![],
        }
    }

    fn kimball_rules_toml(&self, section: &str) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        let (dim_pattern, dim_display) = self.prefix_for_convention("dim");
        let (fact_pattern, fact_display) = self.prefix_for_convention("fact");
        match self.layering_strategy {
            // Medallion: dims, facts, and marts all live under the final layer (gold)
            LayeringStrategy::Medallion => vec![
                format!(
                    r#"[[{section}]]
name = "{final_layer}_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["dimensions", "facts", "marts"]
includes = ["models/{final_layer}"]
description = "The {final_layer} layer must contain dimensions, facts, and marts subfolders""#
                ),
                format!(
                    r#"[[{section}]]
name = "dimension_naming"
type = "name_convention"
pattern = "{dim_pattern}"
includes = ["models/{final_layer}/dimensions"]
description = "Dimension models must start with {dim_display}""#
                ),
                format!(
                    r#"[[{section}]]
name = "fact_naming"
type = "name_convention"
pattern = "{fact_pattern}"
includes = ["models/{final_layer}/facts"]
description = "Fact models must start with {fact_display}""#
                ),
            ],
            // Common: dims and facts live in the middle layer (warehouse),
            // marts is a separate final layer
            LayeringStrategy::Common => vec![
                format!(
                    r#"[[{section}]]
name = "{middle_layer}_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["dimensions", "facts"]
includes = ["models/{middle_layer}"]
description = "The {middle_layer} layer must contain dimensions and facts subfolders""#
                ),
                format!(
                    r#"[[{section}]]
name = "dimension_naming"
type = "name_convention"
pattern = "{dim_pattern}"
includes = ["models/{middle_layer}/dimensions"]
description = "Dimension models must start with {dim_display}""#
                ),
                format!(
                    r#"[[{section}]]
name = "fact_naming"
type = "name_convention"
pattern = "{fact_pattern}"
includes = ["models/{middle_layer}/facts"]
description = "Fact models must start with {fact_display}""#
                ),
            ],
            LayeringStrategy::None => vec![],
        }
    }

    fn inmon_rules_toml(&self, section: &str) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        vec![
            format!(
                r#"[[{section}]]
name = "{middle_layer}_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["core", "reference", "bridge"]
includes = ["models/{middle_layer}"]
description = "The {middle_layer} layer must contain normalized enterprise model components""#
            ),
            format!(
                r#"[[{section}]]
name = "{final_layer}_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["finance", "marketing", "operations"]
includes = ["models/{final_layer}"]
description = "The {final_layer} layer contains departmental data marts (customize these)""#
            ),
            format!(
                r#"[[{section}]]
name = "enterprise_model_contracts"
type = "has_contract_enforced"
includes = ["models/{middle_layer}"]
description = "Normalized enterprise model tables must have enforced contracts""#
            ),
            format!(
                r#"[[{section}]]
name = "enterprise_model_unique_keys"
type = "has_unique_test"
includes = ["models/{middle_layer}"]
description = "All normalized tables must have primary key uniqueness tests""#
            ),
        ]
    }

    #[allow(clippy::too_many_lines)]
    fn data_vault_rules_toml(&self, section: &str) -> Vec<String> {
        let (final_layer, middle_layer) = self.layer_names();
        let (hub_pattern, hub_display) = self.prefix_for_convention("hub");
        let (lnk_pattern, lnk_display) = self.prefix_for_convention("lnk");
        let (sat_pattern, sat_display) = self.prefix_for_convention("sat");
        let (t_lnk_pattern, t_lnk_display) = self.prefix_for_convention("t_lnk");
        let (dim_pattern, dim_display) = self.prefix_for_convention("dim");
        let (fact_pattern, fact_display) = self.prefix_for_convention("fact");
        vec![
            format!(
                r#"[[{section}]]
name = "{middle_layer}_vault_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["hubs", "links", "satellites", "t_links"]
includes = ["models/{middle_layer}"]
description = "The {middle_layer} layer must contain Data Vault components""#
            ),
            format!(
                r#"[[{section}]]
name = "{final_layer}_subfolders"
type = "allowed_subfolders"
allowed_subfolders = ["business_vault", "information_mart"]
includes = ["models/{final_layer}"]
description = "The {final_layer} layer contains business vault and information marts""#
            ),
            format!(
                r#"[[{section}]]
name = "hub_naming"
type = "name_convention"
pattern = "{hub_pattern}"
includes = ["models/{middle_layer}/hubs"]
description = "Hub models must start with {hub_display}""#
            ),
            format!(
                r#"[[{section}]]
name = "link_naming"
type = "name_convention"
pattern = "{lnk_pattern}"
includes = ["models/{middle_layer}/links"]
description = "Link models must start with {lnk_display}""#
            ),
            format!(
                r#"[[{section}]]
name = "satellite_naming"
type = "name_convention"
pattern = "{sat_pattern}"
includes = ["models/{middle_layer}/satellites"]
description = "Satellite models must start with {sat_display}""#
            ),
            format!(
                r#"[[{section}]]
name = "t_link_naming"
type = "name_convention"
pattern = "{t_lnk_pattern}"
includes = ["models/{middle_layer}/t_links"]
description = "Transactional link models must start with {t_lnk_display}""#
            ),
            format!(
                r#"[[{section}]]
name = "vault_contracts"
type = "has_contract_enforced"
includes = ["models/{middle_layer}"]
description = "All vault tables must have enforced contracts""#
            ),
            format!(
                r#"[[{section}]]
name = "hub_unique_keys"
type = "has_unique_test"
includes = ["models/{middle_layer}/hubs"]
description = "All hubs must have a uniqueness test on the hash key""#
            ),
            format!(
                r#"[[{section}]]
name = "link_unique_keys"
type = "has_unique_test"
includes = ["models/{middle_layer}/links"]
description = "All links must have a uniqueness test on the hash key""#
            ),
            format!(
                r#"[[{section}]]
name = "info_mart_dim_naming"
type = "name_convention"
pattern = "{dim_pattern}"
includes = ["models/{final_layer}/information_mart"]
description = "Information mart dimension models must start with {dim_display}""#
            ),
            format!(
                r#"[[{section}]]
name = "info_mart_fact_naming"
type = "name_convention"
pattern = "{fact_pattern}"
includes = ["models/{final_layer}/information_mart"]
description = "Information mart fact models must start with {fact_display}""#
            ),
        ]
    }

    pub fn to_yaml(&self) -> String {
        let mut content = String::from(
            "# dbtective configuration file\n\
             # Documentation: https://feliblo.github.io/dbtective/docs/config\n\
             # Rules: https://feliblo.github.io/dbtective/docs/rules/\n\n",
        );

        // Auto-parse command configuration
        if let Some(ref cmd) = self.auto_parse_command {
            let _ = writeln!(content, "config:\n  auto_parse_command: \"{cmd}\"\n");
        } else {
            content.push_str("# config:\n#   auto_parse_command: \"dbt parse\"\n\n");
        }

        content.push_str("manifest_tests:\n");

        // Emit manifest rules grouped by category
        for category in Self::MANIFEST_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .manifest_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), "  "));
            for rule in rules {
                content.push_str(&self.manifest_rule_to_yaml(rule));
                content.push_str("\n\n");
            }
        }

        // Data Modeling section (methodology + layering rules)
        let methodology_rules = self.methodology_rules_yaml();
        let marts_rule = self.marts_exposure_rule_yaml();
        if !methodology_rules.is_empty() || marts_rule.is_some() {
            content.push_str(&Self::section_banner("Data Model/Layering Specific", "  "));
            for rule_yaml in methodology_rules {
                content.push_str(&rule_yaml);
                content.push_str("\n\n");
            }
            if let Some(extra) = marts_rule {
                content.push_str(&extra);
                content.push_str("\n\n");
            }
        }

        content.push_str("catalog_tests:\n");

        // Emit catalog rules grouped by category
        for category in Self::CATALOG_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .catalog_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), "  "));
            for rule in rules {
                content.push_str(&self.catalog_rule_to_yaml(rule));
                content.push_str("\n\n");
            }
        }

        content.trim_end().to_string()
    }

    pub fn to_toml(&self) -> String {
        let mut content = String::from(
            "# dbtective configuration file\n\
             # Documentation: https://feliblo.github.io/dbtective/docs/config\n\
             # Rules: https://feliblo.github.io/dbtective/docs/rules/\n\n",
        );

        // Auto-parse command configuration
        if let Some(ref cmd) = self.auto_parse_command {
            let _ = writeln!(content, "[config]\nauto_parse_command = \"{cmd}\"\n");
        } else {
            content.push_str("# [config]\n# auto_parse_command = \"dbt parse\"\n\n");
        }

        // Emit manifest rules grouped by category
        for category in Self::MANIFEST_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .manifest_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), ""));
            for rule in rules {
                content.push_str(&self.manifest_rule_to_toml(rule, "manifest_tests"));
                content.push_str("\n\n");
            }
        }

        // Data Modeling section (methodology + layering rules)
        let methodology_rules = self.methodology_rules_toml("manifest_tests");
        let marts_rule = self.marts_exposure_rule_toml("manifest_tests");
        if !methodology_rules.is_empty() || marts_rule.is_some() {
            content.push_str(&Self::section_banner("Data Model/Layering Specific", ""));
            for rule_toml in methodology_rules {
                content.push_str(&rule_toml);
                content.push_str("\n\n");
            }
            if let Some(extra) = marts_rule {
                content.push_str(&extra);
                content.push_str("\n\n");
            }
        }

        // Emit catalog rules grouped by category
        for category in Self::CATALOG_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .catalog_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), ""));
            for rule in rules {
                content.push_str(&self.catalog_rule_to_toml(rule, "catalog_tests"));
                content.push_str("\n\n");
            }
        }

        content.trim_end().to_string()
    }

    pub fn to_pyproject(&self) -> String {
        let mut content = String::from(
            "\n# dbtective configuration\n\
             # Documentation: https://feliblo.github.io/dbtective/docs/config\n\
             # Rules: https://feliblo.github.io/dbtective/docs/rules/\n\n\
             [tool.dbtective]\n\n",
        );

        // Auto-parse command configuration
        if let Some(ref cmd) = self.auto_parse_command {
            let _ = writeln!(
                content,
                "[tool.dbtective.config]\nauto_parse_command = \"{cmd}\"\n"
            );
        } else {
            content.push_str("# [tool.dbtective.config]\n# auto_parse_command = \"dbt parse\"\n\n");
        }

        // Emit manifest rules grouped by category
        for category in Self::MANIFEST_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .manifest_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), ""));
            for rule in rules {
                content
                    .push_str(&self.manifest_rule_to_toml(rule, "tool.dbtective.manifest_tests"));
                content.push_str("\n\n");
            }
        }

        // Data Modeling section (methodology + layering rules)
        let methodology_rules = self.methodology_rules_toml("tool.dbtective.manifest_tests");
        let marts_rule = self.marts_exposure_rule_toml("tool.dbtective.manifest_tests");
        if !methodology_rules.is_empty() || marts_rule.is_some() {
            content.push_str(&Self::section_banner("Data Model/Layering Specific", ""));
            for rule_toml in methodology_rules {
                content.push_str(&rule_toml);
                content.push_str("\n\n");
            }
            if let Some(extra) = marts_rule {
                content.push_str(&extra);
                content.push_str("\n\n");
            }
        }

        // Emit catalog rules grouped by category
        for category in Self::CATALOG_CATEGORIES_ORDER {
            let rules: Vec<_> = self
                .catalog_rules
                .iter()
                .filter(|r| &r.default_category() == category)
                .collect();
            if rules.is_empty() {
                continue;
            }
            content.push_str(&Self::section_banner(category.as_str(), ""));
            for rule in rules {
                content.push_str(&self.catalog_rule_to_toml(rule, "tool.dbtective.catalog_tests"));
                content.push_str("\n\n");
            }
        }

        content.trim_end().to_string()
    }

    #[allow(clippy::too_many_lines)]
    fn manifest_rule_to_yaml(&self, rule: &ManifestSpecificRuleConfig) -> String {
        match rule {
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders, ..
            } => {
                let folders_str = allowed_subfolders
                    .iter()
                    .map(|f| format!("\"{f}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"  - name: "allowed_subfolders"
    type: "allowed_subfolders"
    allowed_subfolders: [{folders_str}]"#
                )
            }
            ManifestSpecificRuleConfig::CodeContainsRefs {} => r#"  - name: "code_contains_refs"
    type: "code_contains_refs""#
                .to_string(),
            ManifestSpecificRuleConfig::HasContractEnforced { .. } => {
                let includes = match self.layering_strategy {
                    LayeringStrategy::Medallion => Some(vec!["models/silver", "models/gold"]),
                    LayeringStrategy::Common => Some(vec!["models/marts", "models/warehouse"]),
                    LayeringStrategy::None => return String::new(),
                };

                let mut s = r#"  - name: "has_contract_enforced"
    type: "has_contract_enforced""#
                    .to_string();

                if let Some(folders) = includes {
                    let folders_str = folders
                        .iter()
                        .map(|f| format!("\"{f}\""))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(s, "\n    includes: [{folders_str}]");
                }

                s
            }
            ManifestSpecificRuleConfig::HasDescription { .. } => r#"  - name: "has_description"
    type: "has_description"
"#
            .to_string(),
            ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns,
                case_sensitive,
            } => {
                let patterns_str = forbidden_patterns
                    .iter()
                    .map(|p| format!("\"{p}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                let mut s = format!(
                    r#"  - name: "code_forbidden_patterns"
    type: "code_forbidden_patterns"
    forbidden_patterns: [{patterns_str}]"#
                );
                if *case_sensitive {
                    s.push_str("\n    case_sensitive: true");
                }

                s
            }
            ManifestSpecificRuleConfig::HasMetadataKeys { required_keys, .. } => {
                let keys_str = required_keys
                    .iter()
                    .map(|k| format!("\"{k}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"  - name: "has_owner"
    type: "has_metadata_keys"
    required_keys: [{keys_str}]"#
                )
            }
            ManifestSpecificRuleConfig::HasRefs {} => r#"  - name: "refs_must_be_used"
    type: "has_refs""#
                .to_string(),
            ManifestSpecificRuleConfig::HasTags {
                required_tags,
                criteria,
            } => {
                let tags_str = required_tags
                    .iter()
                    .map(|t| format!("\"{t}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                let criteria_str = match criteria {
                    HasTagsCriteria::All => "all",
                    HasTagsCriteria::Any => "any",
                    HasTagsCriteria::OneOf => "one_of",
                };
                format!(
                    r#"  - name: "require_execution_tags"
    type: "has_tags"
    required_tags: [{tags_str}]
    criteria: "{criteria_str}"
    description: "Resources need to have at least one of the required tags. To decide when a resource should be run.""#
                )
            }
            ManifestSpecificRuleConfig::HasUniqueTest { .. } => r#"  - name: "has_unique_test"
    type: "has_unique_test""#
                .to_string(),
            ManifestSpecificRuleConfig::IsNotOrphaned { .. } => {
                let (final_layer, _) = self.layer_names();
                let include_path = match (&self.layering_strategy, &self.methodology) {
                    (LayeringStrategy::None, _) => {
                        return r#"  - name: "is_not_orphaned"
    type: "is_not_orphaned""#
                            .to_string();
                    }
                    (_, DataModelMethodology::Kimball) => match self.layering_strategy {
                        LayeringStrategy::Medallion => format!("models/{final_layer}/marts"),
                        _ => format!("models/{final_layer}"),
                    },
                    (_, DataModelMethodology::DataVault) => {
                        format!("models/{final_layer}/information_mart")
                    }
                    _ => format!("models/{final_layer}"),
                };
                format!(
                    r#"  - name: "all_marts_are_exposed"
    type: "is_not_orphaned"
    description: "All {final_layer} models should be referenced by at least one exposure."
    applies_to: ["models"]
    includes: ["{include_path}"]
    allowed_references: ["exposures"]"#
                )
            }
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines } => {
                format!(
                    r#"  - name: "code_max_lines"
    type: "code_max_lines"
    max_lines: {max_lines}"#
                )
            }
            ManifestSpecificRuleConfig::MaxDownstreamDependencies { max_downstream, .. } => {
                format!(
                    r#"  - name: "max_downstream_dependencies"
    description: "Reduce model fanout complexity"
    type: "max_downstream_dependencies"
    max_downstream: {max_downstream}
    applies_to: ["models"]

  - name: "source_max_downstream_dependencies"
    description: "Sources should not be referenced by more than 1 model (use a staging model)"
    type: "max_downstream_dependencies"
    max_downstream: 1
    applies_to: ["sources"]"#
                )
            }
            ManifestSpecificRuleConfig::CodeMaxJoins { max_joins } => {
                format!(
                    r#"  - name: "code_max_joins"
    description: "Reduce model complexity"
    type: "code_max_joins"
    max_joins: {max_joins}"#
                )
            }
            ManifestSpecificRuleConfig::CodeNoHardcodedRefs {} => {
                r#"  - name: "code_no_hardcoded_refs"
    description: "Detect hardcoded table references (schema.table) in SQL code"
    type: "code_no_hardcoded_refs""#
                    .to_string()
            }
            ManifestSpecificRuleConfig::MaxUpstreamDependencies { max_upstream, .. } => {
                format!(
                    r#"  - name: "max_upstream_dependencies"
    description: "Models should not select from too many ref() and source() calls"
    type: "max_upstream_dependencies"
    max_upstream: {max_upstream}"#
                )
            }
            ManifestSpecificRuleConfig::NameConvention { convention } => {
                format!(
                    r#"  - name: "naming_convention"
    type: "name_convention"
    pattern: "{}""#,
                    convention.name()
                )
            }
            ManifestSpecificRuleConfig::SourcesHaveFreshness {} => {
                r#"  - name: "sources_have_freshness"
    type: "sources_have_freshness"
"#
                .to_string()
            }
            ManifestSpecificRuleConfig::SourcesHaveLoader {} => r#"  - name: "sources_have_loader"
    type: "sources_have_loader"
"#
            .to_string(),
        }
    }

    fn marts_exposure_rule_yaml(&self) -> Option<String> {
        if matches!(self.layering_strategy, LayeringStrategy::None) {
            return None;
        }
        let (final_layer, _) = self.layer_names();
        let folder = match (&self.methodology, &self.layering_strategy) {
            // Kimball+Medallion: marts subfolder within gold
            (DataModelMethodology::Kimball, LayeringStrategy::Medallion) => {
                format!("models/{final_layer}/marts")
            }
            (DataModelMethodology::DataVault, _) => {
                format!("models/{final_layer}/information_mart")
            }
            // Kimball+Common and everything else: the whole final layer
            _ => format!("models/{final_layer}"),
        };
        Some(format!(
            r#"  - name: "all_marts_are_exposed"
    type: "is_not_orphaned"
    applies_to: ["models"]
    includes: ["{folder}"]
    allowed_references: ["exposures"]
    severity: "warning""#
        ))
    }

    fn marts_exposure_rule_toml(&self, section: &str) -> Option<String> {
        if matches!(self.layering_strategy, LayeringStrategy::None) {
            return None;
        }
        let (final_layer, _) = self.layer_names();
        let folder = match (&self.methodology, &self.layering_strategy) {
            // Kimball+Medallion: marts subfolder within gold
            (DataModelMethodology::Kimball, LayeringStrategy::Medallion) => {
                format!("models/{final_layer}/marts")
            }
            (DataModelMethodology::DataVault, _) => {
                format!("models/{final_layer}/information_mart")
            }
            // Kimball+Common and everything else: the whole final layer
            _ => format!("models/{final_layer}"),
        };
        Some(format!(
            r#"[[{section}]]
name = "all_marts_are_exposed"
type = "is_not_orphaned"
applies_to = ["models"]
includes = ["{folder}"]
allowed_references = ["exposures"]
severity = "warning""#
        ))
    }

    fn catalog_rule_to_yaml(&self, rule: &CatalogSpecificRuleConfig) -> String {
        match rule {
            CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                r#"  - name: "all_columns_documented"
    type: "columns_all_documented"
    description: "All columns must exist in the documentation."
    severity: "warning""#
                    .to_string()
            }
            CatalogSpecificRuleConfig::ColumnsCanonicalName {
                canonical,
                invalid_names,
                ..
            } => {
                let invalid_str = invalid_names
                    .iter()
                    .map(|n| match n {
                        crate::core::config::check_config_options::ColumnNamePattern::Literal(
                            s,
                        ) => format!("\"{s}\""),
                        crate::core::config::check_config_options::ColumnNamePattern::Regex(r) => {
                            format!("\"{}\"", r.as_str())
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"  - name: "canonical_column_names"
    type: "columns_canonical_name"
    canonical: "{canonical}"
    invalid_names: [{invalid_str}]"#
                )
            }
            CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage } => {
                let includes = match self.layering_strategy {
                    LayeringStrategy::Medallion => Some(vec!["models/silver", "models/gold"]),
                    LayeringStrategy::Common => Some(vec!["models/marts", "models/warehouse"]),
                    LayeringStrategy::None => None,
                };

                let mut s = r#"  - name: "columns_have_data_type"
    type: "columns_have_data_type"
    description: "All columns must have data types defined.""#
                    .to_string();

                if let Some(threshold) = min_coverage {
                    let _ = write!(s, "\n    min_coverage: {threshold}");
                }

                if let Some(folders) = includes {
                    let folders_str = folders
                        .iter()
                        .map(|f| format!("\"{f}\""))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(s, "\n    includes: [{folders_str}]");
                }

                s
            }
            CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                r#"  - name: "all_columns_described"
    type: "columns_have_description"
    description: "All columns must have a description.""#
                    .to_string()
            }
            CatalogSpecificRuleConfig::ColumnsNameConvention { convention, .. } => {
                let convention_name = convention.name();
                format!(
                    r#"  - name: "column_names_{convention_name}"
    type: "columns_name_convention"
    description: "All column names must be {convention_name}."
    pattern: "{convention_name}""#
                )
            }
        }
    }
    #[allow(clippy::too_many_lines)]
    fn manifest_rule_to_toml(&self, rule: &ManifestSpecificRuleConfig, section: &str) -> String {
        match rule {
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders, ..
            } => {
                let folders_str = allowed_subfolders
                    .iter()
                    .map(|f| format!("\"{f}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"[[{section}]]
name = "allowed_subfolders"
type = "allowed_subfolders"
allowed_subfolders = [{folders_str}]"#
                )
            }
            ManifestSpecificRuleConfig::CodeContainsRefs {} => {
                format!(
                    r#"[[{section}]]
name = "code_contains_refs"
type = "code_contains_refs""#
                )
            }
            ManifestSpecificRuleConfig::HasContractEnforced { .. } => {
                let includes = match self.layering_strategy {
                    LayeringStrategy::Medallion => Some(vec!["models/silver", "models/gold"]),
                    LayeringStrategy::Common => Some(vec!["models/marts", "models/warehouse"]),
                    LayeringStrategy::None => return String::new(),
                };

                let mut s = format!(
                    r#"[[{section}]]
name = "has_contract_enforced"
type = "has_contract_enforced""#
                );

                if let Some(folders) = includes {
                    let folders_str = folders
                        .iter()
                        .map(|f| format!("\"{f}\""))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(s, "\nincludes = [{folders_str}]");
                }

                s
            }
            ManifestSpecificRuleConfig::HasDescription { .. } => {
                format!(
                    r#"[[{section}]]
name = "has_description"
type = "has_description"
"#
                )
            }
            ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns,
                case_sensitive,
            } => {
                let patterns_str = forbidden_patterns
                    .iter()
                    .map(|p| format!("\"{p}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                let mut s = format!(
                    r#"[[{section}]]
name = "code_forbidden_patterns"
type = "code_forbidden_patterns"
forbidden_patterns = [{patterns_str}]"#
                );
                if *case_sensitive {
                    s.push_str("\ncase_sensitive = true");
                }
                s
            }
            ManifestSpecificRuleConfig::HasMetadataKeys { required_keys, .. } => {
                let keys_str = required_keys
                    .iter()
                    .map(|k| format!("\"{k}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"[[{section}]]
name = "has_owner"
type = "has_metadata_keys"
required_keys = [{keys_str}]"#
                )
            }
            ManifestSpecificRuleConfig::HasRefs {} => {
                format!(
                    r#"[[{section}]]
name = "refs_must_be_used"
type = "has_refs""#
                )
            }
            ManifestSpecificRuleConfig::HasTags {
                required_tags,
                criteria,
            } => {
                let tags_str = required_tags
                    .iter()
                    .map(|t| format!("\"{t}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                let criteria_str = match criteria {
                    HasTagsCriteria::All => "all",
                    HasTagsCriteria::Any => "any",
                    HasTagsCriteria::OneOf => "one_of",
                };
                format!(
                    r#"[[{section}]]
name = "require_execution_tags"
type = "has_tags"
required_tags = [{tags_str}]
criteria = "{criteria_str}"
description = "Resources need to have at least one of the required tags. To decide when a resource should be run.""#
                )
            }
            ManifestSpecificRuleConfig::HasUniqueTest { .. } => {
                format!(
                    r#"[[{section}]]
name = "has_unique_test"
type = "has_unique_test""#
                )
            }
            ManifestSpecificRuleConfig::IsNotOrphaned { .. } => {
                let (final_layer, _) = self.layer_names();
                let include_path = match (&self.layering_strategy, &self.methodology) {
                    (LayeringStrategy::None, _) => {
                        return format!(
                            r#"[[{section}]]
name = "is_not_orphaned"
type = "is_not_orphaned""#
                        );
                    }
                    (_, DataModelMethodology::Kimball) => match self.layering_strategy {
                        LayeringStrategy::Medallion => format!("models/{final_layer}/marts"),
                        _ => format!("models/{final_layer}"),
                    },
                    (_, DataModelMethodology::DataVault) => {
                        format!("models/{final_layer}/information_mart")
                    }
                    _ => format!("models/{final_layer}"),
                };
                format!(
                    r#"[[{section}]]
name = "all_marts_are_exposed"
type = "is_not_orphaned"
description = "All {final_layer} models should be referenced by at least one exposure."
applies_to = ["models"]
includes = ["{include_path}"]
allowed_references = ["exposures"]"#
                )
            }
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines } => {
                format!(
                    r#"[[{section}]]
name = "code_max_lines"
type = "code_max_lines"
max_lines = {max_lines}"#
                )
            }
            ManifestSpecificRuleConfig::MaxDownstreamDependencies { max_downstream, .. } => {
                format!(
                    r#"[[{section}]]
name = "max_downstream_dependencies"
description = "Reduce model fanout complexity"
type = "max_downstream_dependencies"
max_downstream = {max_downstream}
applies_to = ["models"]

[[{section}]]
name = "source_max_downstream_dependencies"
description = "Sources should not be referenced by more than 1 model (use a staging model)"
type = "max_downstream_dependencies"
max_downstream = 1
applies_to = ["sources"]"#
                )
            }
            ManifestSpecificRuleConfig::CodeMaxJoins { max_joins } => {
                format!(
                    r#"[[{section}]]
name = "code_max_joins"
description = "Reduce model complexity"
type = "code_max_joins"
max_joins = {max_joins}"#
                )
            }
            ManifestSpecificRuleConfig::CodeNoHardcodedRefs {} => {
                format!(
                    r#"[[{section}]]
name = "code_no_hardcoded_refs"
description = "Detect hardcoded table references (schema.table) in SQL code"
type = "code_no_hardcoded_refs""#
                )
            }
            ManifestSpecificRuleConfig::MaxUpstreamDependencies { max_upstream, .. } => {
                format!(
                    r#"[[{section}]]
name = "max_upstream_dependencies"
description = "Models should not select from too many ref() and source() calls"
type = "max_upstream_dependencies"
max_upstream = {max_upstream}"#
                )
            }
            ManifestSpecificRuleConfig::NameConvention { convention } => {
                format!(
                    r#"[[{section}]]
name = "naming_convention"
type = "name_convention"
pattern = "{}""#,
                    convention.name()
                )
            }
            ManifestSpecificRuleConfig::SourcesHaveFreshness {} => {
                format!(
                    r#"[[{section}]]
name = "sources_have_freshness"
type = "sources_have_freshness"
"#
                )
            }
            ManifestSpecificRuleConfig::SourcesHaveLoader {} => {
                format!(
                    r#"[[{section}]]
name = "sources_have_loader"
type = "sources_have_loader"
"#
                )
            }
        }
    }

    fn catalog_rule_to_toml(&self, rule: &CatalogSpecificRuleConfig, section: &str) -> String {
        match rule {
            CatalogSpecificRuleConfig::ColumnsAllDocumented {} => {
                format!(
                    r#"[[{section}]]
name = "all_columns_documented"
type = "columns_all_documented"
description = "All columns must exist in the documentation.""#
                )
            }
            CatalogSpecificRuleConfig::ColumnsCanonicalName {
                canonical,
                invalid_names,
                ..
            } => {
                let invalid_str = invalid_names
                    .iter()
                    .map(|n| match n {
                        crate::core::config::check_config_options::ColumnNamePattern::Literal(
                            s,
                        ) => format!("\"{s}\""),
                        crate::core::config::check_config_options::ColumnNamePattern::Regex(r) => {
                            format!("\"{}\"", r.as_str())
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    r#"[[{section}]]
name = "canonical_column_names"
type = "columns_canonical_name"
canonical = "{canonical}"
invalid_names = [{invalid_str}]"#
                )
            }
            CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage } => {
                let includes = match self.layering_strategy {
                    LayeringStrategy::Medallion => Some(vec!["models/silver", "models/gold"]),
                    LayeringStrategy::Common => Some(vec!["models/marts", "models/warehouse"]),
                    LayeringStrategy::None => None,
                };

                let mut s = format!(
                    r#"[[{section}]]
name = "columns_have_data_type"
type = "columns_have_data_type"
description = "All columns must have data types defined.""#
                );

                if let Some(threshold) = min_coverage {
                    let _ = write!(s, "\nmin_coverage = {threshold}");
                }

                if let Some(folders) = includes {
                    let folders_str = folders
                        .iter()
                        .map(|f| format!("\"{f}\""))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let _ = write!(s, "\nincludes = [{folders_str}]");
                }

                s
            }
            CatalogSpecificRuleConfig::ColumnsHaveDescription {} => {
                format!(
                    r#"[[{section}]]
name = "all_columns_described"
type = "columns_have_description"
description = "All columns must have a description.""#
                )
            }
            CatalogSpecificRuleConfig::ColumnsNameConvention { convention, .. } => {
                let convention_name = convention.name();
                format!(
                    r#"[[{section}]]
name = "column_names_{convention_name}"
type = "columns_name_convention"
description = "All column names must be {convention_name}."
pattern = "{convention_name}""#
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::check_config_options::{ColumnNamePattern, HasTagsCriteria};
    use crate::core::config::naming_convention::NamingConvention;

    // Helper function to create a minimal questionnaire result
    fn create_questionnaire_result(
        layering_strategy: LayeringStrategy,
        manifest_rules: Vec<ManifestSpecificRuleConfig>,
        catalog_rules: Vec<CatalogSpecificRuleConfig>,
        naming_convention: NamingConvention,
    ) -> QuestionnaireResult {
        create_questionnaire_result_with_methodology(
            layering_strategy,
            DataModelMethodology::None,
            manifest_rules,
            catalog_rules,
            naming_convention,
        )
    }

    fn create_questionnaire_result_with_methodology(
        layering_strategy: LayeringStrategy,
        methodology: DataModelMethodology,
        manifest_rules: Vec<ManifestSpecificRuleConfig>,
        catalog_rules: Vec<CatalogSpecificRuleConfig>,
        naming_convention: NamingConvention,
    ) -> QuestionnaireResult {
        QuestionnaireResult {
            format: super::super::questionnaire::ConfigFormat::Yaml,
            naming_convention,
            layering_strategy,
            methodology,
            manifest_rules,
            catalog_rules,
            auto_parse_command: None,
        }
    }

    // ========================================================================
    // AllowedSubfolders Data Model Mapping Tests
    // ========================================================================

    #[test]
    fn test_allowed_subfolders_medallion_mapping() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders, ..
            } => {
                assert_eq!(allowed_subfolders.len(), 3);
                assert_eq!(allowed_subfolders[0], "bronze");
                assert_eq!(allowed_subfolders[1], "silver");
                assert_eq!(allowed_subfolders[2], "gold");
            }
            _ => panic!("Expected AllowedSubfolders rule"),
        }
    }

    #[test]
    fn test_allowed_subfolders_common_mapping() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders, ..
            } => {
                assert_eq!(allowed_subfolders.len(), 3);
                assert_eq!(allowed_subfolders[0], "staging");
                assert_eq!(allowed_subfolders[1], "warehouse");
                assert_eq!(allowed_subfolders[2], "marts");
            }
            _ => panic!("Expected AllowedSubfolders rule"),
        }
    }

    #[test]
    fn test_allowed_subfolders_none_mapping() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders, ..
            } => {
                assert_eq!(allowed_subfolders.len(), 0);
                assert!(allowed_subfolders.is_empty());
            }
            _ => panic!("Expected AllowedSubfolders rule"),
        }
    }

    // ========================================================================
    // ManifestRule Creation Tests - All 10 Rule Types
    // ========================================================================

    #[test]
    fn test_create_manifest_rule_has_description() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        assert!(matches!(
            config.manifest_rules[0],
            ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None
            }
        ));
    }

    #[test]
    fn test_create_manifest_rule_name_convention() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            convention.clone(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::NameConvention { convention: c } => {
                assert_eq!(c.name(), convention.name());
            }
            _ => panic!("Expected NameConvention rule"),
        }
    }

    #[test]
    fn test_create_manifest_rule_has_tags() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasTags {
                required_tags: vec![],
                criteria: HasTagsCriteria::default(),
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::HasTags {
                required_tags,
                criteria,
            } => {
                assert_eq!(required_tags.len(), 5);
                assert!(required_tags.contains(&"daily".to_string()));
                assert!(required_tags.contains(&"weekly".to_string()));
                assert!(required_tags.contains(&"monthly".to_string()));
                assert!(required_tags.contains(&"yearly".to_string()));
                assert!(required_tags.contains(&"inactive".to_string()));
                assert!(matches!(criteria, HasTagsCriteria::OneOf));
            }
            _ => panic!("Expected HasTags rule"),
        }
    }

    #[test]
    fn test_create_manifest_rule_is_not_orphaned() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::IsNotOrphaned { allowed_references } => {
                assert!(allowed_references.is_empty());
            }
            _ => panic!("Expected IsNotOrphaned rule"),
        }
    }

    #[test]
    fn test_create_manifest_rule_has_unique_test() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasUniqueTest {
                allowed_test_names: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::HasUniqueTest { allowed_test_names } => {
                assert!(allowed_test_names.is_empty());
            }
            _ => panic!("Expected HasUniqueTest rule"),
        }
    }

    #[test]
    fn test_create_manifest_rule_has_contract_enforced() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        assert!(matches!(
            config.manifest_rules[0],
            ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }
        ));
    }

    #[test]
    fn test_create_manifest_rule_has_metadata_keys() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec![],
                custom_message: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys,
                custom_message,
            } => {
                assert_eq!(required_keys.len(), 1);
                assert_eq!(required_keys[0], "owner");
                assert!(custom_message.is_none());
            }
            _ => panic!("Expected HasMetadataKeys rule"),
        }
    }

    #[test]
    fn test_create_manifest_rule_has_refs() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasRefs {}],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        assert!(matches!(
            config.manifest_rules[0],
            ManifestSpecificRuleConfig::HasRefs {}
        ));
    }

    #[test]
    fn test_create_manifest_rule_max_code_lines() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 0 }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::CodeMaxLines { max_lines } => {
                assert_eq!(*max_lines, 200);
            }
            _ => panic!("Expected CodeMaxLines rule"),
        }
    }

    // ========================================================================
    // CatalogRule Creation Tests - All 4 Rule Types
    // ========================================================================

    #[test]
    fn test_create_catalog_rule_columns_all_documented() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsAllDocumented {}],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 1);
        assert!(matches!(
            config.catalog_rules[0],
            CatalogSpecificRuleConfig::ColumnsAllDocumented {}
        ));
    }

    #[test]
    fn test_create_catalog_rule_columns_have_description() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsHaveDescription {}],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 1);
        assert!(matches!(
            config.catalog_rules[0],
            CatalogSpecificRuleConfig::ColumnsHaveDescription {}
        ));
    }

    #[test]
    fn test_create_catalog_rule_columns_name_convention() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            convention.clone(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 1);
        match &config.catalog_rules[0] {
            CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: c,
                data_types,
                ..
            } => {
                assert_eq!(c.name(), convention.name());
                assert!(data_types.is_none());
            }
            _ => panic!("Expected ColumnsNameConvention rule"),
        }
    }

    #[test]
    fn test_create_catalog_rule_columns_canonical_name() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsCanonicalName {
                canonical: String::new(),
                invalid_names: vec![],
                exceptions: None,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 1);
        match &config.catalog_rules[0] {
            CatalogSpecificRuleConfig::ColumnsCanonicalName {
                canonical,
                invalid_names,
                exceptions,
            } => {
                assert_eq!(canonical, "user_id");
                assert_eq!(invalid_names.len(), 2);
                assert!(matches!(
                    invalid_names[0],
                    ColumnNamePattern::Literal(ref s) if s == "userid"
                ));
                assert!(matches!(
                    invalid_names[1],
                    ColumnNamePattern::Literal(ref s) if s == "UserId"
                ));
                assert!(exceptions.is_none());
            }
            _ => panic!("Expected ColumnsCanonicalName rule"),
        }
    }

    // ========================================================================
    // Strictness Level Tests
    // ========================================================================

    #[test]
    fn test_strictness_basic_manifest_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
            ],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 2);

        // Verify both expected rules are present
        let has_description = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::HasDescription { .. }));
        let has_name_convention = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::NameConvention { .. }));

        assert!(has_description, "Expected HasDescription rule");
        assert!(has_name_convention, "Expected NameConvention rule");
    }

    #[test]
    fn test_strictness_basic_catalog_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 1);
        assert!(matches!(
            config.catalog_rules[0],
            CatalogSpecificRuleConfig::ColumnsNameConvention { .. }
        ));
    }

    #[test]
    fn test_strictness_standard_manifest_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::HasMetadataKeys {
                    required_keys: vec![],
                    custom_message: None,
                },
                ManifestSpecificRuleConfig::HasRefs {},
            ],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 4);

        let has_description = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::HasDescription { .. }));
        let has_name_convention = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::NameConvention { .. }));
        let has_metadata_keys = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::HasMetadataKeys { .. }));
        let has_refs = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::HasRefs {}));

        assert!(has_description);
        assert!(has_name_convention);
        assert!(has_metadata_keys);
        assert!(has_refs);
    }

    #[test]
    fn test_strictness_standard_catalog_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
            ],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 2);

        let has_name_convention = config
            .catalog_rules
            .iter()
            .any(|r| matches!(r, CatalogSpecificRuleConfig::ColumnsNameConvention { .. }));
        let has_description = config
            .catalog_rules
            .iter()
            .any(|r| matches!(r, CatalogSpecificRuleConfig::ColumnsHaveDescription {}));

        assert!(has_name_convention);
        assert!(has_description);
    }

    #[test]
    fn test_strictness_strict_manifest_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
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
                ManifestSpecificRuleConfig::HasContractEnforced { access_level: None },
                ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                    forbidden_patterns: vec![],
                    case_sensitive: false,
                },
                ManifestSpecificRuleConfig::CodeMaxJoins { max_joins: 0 },
                ManifestSpecificRuleConfig::SourcesHaveLoader {},
                ManifestSpecificRuleConfig::SourcesHaveFreshness {},
            ],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 11);
    }

    #[test]
    fn test_strictness_strict_catalog_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
                CatalogSpecificRuleConfig::ColumnsAllDocumented {},
            ],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 3);
    }

    // ========================================================================
    // Data Model Integration Tests
    // ========================================================================

    #[test]
    fn test_medallion_with_standard_strictness() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::HasMetadataKeys {
                    required_keys: vec![],
                    custom_message: None,
                },
                ManifestSpecificRuleConfig::HasRefs {},
                ManifestSpecificRuleConfig::AllowedSubfolders {
                    allowed_subfolders: vec![],
                    path_prefix: None,
                    path_postfix: None,
                }, // Added by data model
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
            ],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        // Verify AllowedSubfolders is present with Medallion folders
        let has_allowed_subfolders = config.manifest_rules.iter().any(|r| {
            matches!(
                r,
                ManifestSpecificRuleConfig::AllowedSubfolders {
                    allowed_subfolders,
                    ..
                } if allowed_subfolders == &vec!["bronze".to_string(), "silver".to_string(), "gold".to_string()]
            )
        });
        assert!(has_allowed_subfolders);
    }

    #[test]
    fn test_common_with_basic_strictness() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::AllowedSubfolders {
                    allowed_subfolders: vec![],
                    path_prefix: None,
                    path_postfix: None,
                },
            ],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        let has_allowed_subfolders = config.manifest_rules.iter().any(|r| {
            matches!(
                r,
                ManifestSpecificRuleConfig::AllowedSubfolders {
                    allowed_subfolders,
                    ..
                } if allowed_subfolders == &vec!["staging".to_string(), "warehouse".to_string(), "marts".to_string()]
            )
        });
        assert!(has_allowed_subfolders);
    }

    #[test]
    fn test_none_with_strict_strictness_no_allowed_subfolders() {
        // When LayeringStrategy::None is used with Strict, AllowedSubfolders should NOT be added
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
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
                ManifestSpecificRuleConfig::HasContractEnforced { access_level: None },
                // Note: AllowedSubfolders NOT added when LayeringStrategy::None
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
                CatalogSpecificRuleConfig::ColumnsAllDocumented {},
            ],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        // Verify AllowedSubfolders is NOT present
        let has_allowed_subfolders = config
            .manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::AllowedSubfolders { .. }));
        assert!(!has_allowed_subfolders);
    }

    // ========================================================================
    // HasContractEnforced Init Tests - Data Model Includes
    // ========================================================================

    #[test]
    fn test_contract_enforced_medallion_yaml_includes_silver_gold() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "has_contract_enforced""#));
        assert!(yaml.contains(r#"includes: ["models/silver", "models/gold"]"#));
    }

    #[test]
    fn test_contract_enforced_common_yaml_includes_marts() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "has_contract_enforced""#));
        assert!(yaml.contains(r#"includes: ["models/marts", "models/warehouse"]"#));
    }

    #[test]
    fn test_contract_enforced_none_yaml_no_includes() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(!yaml.contains(r#"type: "has_contract_enforced""#));
        assert!(!yaml.contains("includes:"));
    }

    // ========================================================================
    // IsNotOrphaned Init Tests - Data Model Includes
    // ========================================================================

    #[test]
    fn test_is_not_orphaned_common_yaml_all_marts_exposed() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"name: "all_marts_are_exposed""#));
        assert!(yaml.contains(r#"type: "is_not_orphaned""#));
        assert!(yaml.contains(r#"includes: ["models/marts"]"#));
        assert!(yaml.contains(r#"allowed_references: ["exposures"]"#));
    }

    #[test]
    fn test_is_not_orphaned_medallion_yaml_all_gold_exposed() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"name: "all_marts_are_exposed""#));
        assert!(yaml.contains(r#"type: "is_not_orphaned""#));
        assert!(yaml.contains(r#"includes: ["models/gold"]"#));
        assert!(yaml.contains(r#"allowed_references: ["exposures"]"#));
    }

    #[test]
    fn test_is_not_orphaned_none_yaml_generic() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"name: "is_not_orphaned""#));
        assert!(yaml.contains(r#"type: "is_not_orphaned""#));
        assert!(!yaml.contains("all_marts_are_exposed"));
        assert!(!yaml.contains("all_marts_are_exposed"));
    }

    #[test]
    fn test_is_not_orphaned_common_toml_all_marts_exposed() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains(r#"name = "all_marts_are_exposed""#));
        assert!(toml.contains(r#"type = "is_not_orphaned""#));
        assert!(toml.contains(r#"includes = ["models/marts"]"#));
        assert!(toml.contains(r#"allowed_references = ["exposures"]"#));
    }

    #[test]
    fn test_contract_enforced_medallion_toml_includes_silver_gold() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::HasContractEnforced { access_level: None }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains(r#"type = "has_contract_enforced""#));
        assert!(toml.contains(r#"includes = ["models/silver", "models/gold"]"#));
    }

    // ========================================================================
    // CodeForbiddenPatterns Init Tests - Data Model Patterns
    // ========================================================================

    #[test]
    fn test_forbidden_code_medallion_patterns() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns: vec![],
                case_sensitive: false,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns, ..
            } => {
                assert!(forbidden_patterns.contains(&"SELECT *".to_string()));
                assert!(forbidden_patterns.contains(&"bronze.".to_string()));
                assert!(forbidden_patterns.contains(&"silver.".to_string()));
                assert!(forbidden_patterns.contains(&"gold.".to_string()));
                assert_eq!(forbidden_patterns.len(), 4);
            }
            _ => panic!("Expected CodeForbiddenPatterns rule"),
        }
    }

    #[test]
    fn test_forbidden_code_common_patterns() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns: vec![],
                case_sensitive: false,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns, ..
            } => {
                assert!(forbidden_patterns.contains(&"SELECT *".to_string()));
                assert!(forbidden_patterns.contains(&"staging.".to_string()));
                assert!(forbidden_patterns.contains(&"warehouse.".to_string()));
                assert!(forbidden_patterns.contains(&"marts.".to_string()));
                assert_eq!(forbidden_patterns.len(), 4);
            }
            _ => panic!("Expected CodeForbiddenPatterns rule"),
        }
    }

    #[test]
    fn test_forbidden_code_none_no_schema_patterns() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns: vec![],
                case_sensitive: false,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns, ..
            } => {
                assert!(forbidden_patterns.contains(&"SELECT *".to_string()));
                assert_eq!(forbidden_patterns.len(), 1);
            }
            _ => panic!("Expected CodeForbiddenPatterns rule"),
        }
    }

    #[test]
    fn test_forbidden_code_medallion_yaml_output() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns: vec![],
                case_sensitive: false,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "code_forbidden_patterns""#));
        assert!(yaml.contains(r#""bronze.""#));
        assert!(yaml.contains(r#""silver.""#));
        assert!(yaml.contains(r#""gold.""#));
    }

    #[test]
    fn test_forbidden_code_common_toml_output() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::CodeForbiddenPatterns {
                forbidden_patterns: vec![],
                case_sensitive: false,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains(r#"type = "code_forbidden_patterns""#));
        assert!(toml.contains(r#""staging.""#));
        assert!(toml.contains(r#""warehouse.""#));
        assert!(toml.contains(r#""marts.""#));
    }

    // ========================================================================
    // Naming Convention Tests
    // ========================================================================

    #[test]
    fn test_naming_convention_snake_case() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            convention,
        );

        let config = InitConfig::from_questionnaire(&result);

        // Check manifest rule
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::NameConvention { convention: c } => {
                assert_eq!(c.name(), "snake_case");
            }
            _ => panic!("Expected NameConvention rule"),
        }

        // Check catalog rule
        match &config.catalog_rules[0] {
            CatalogSpecificRuleConfig::ColumnsNameConvention { convention: c, .. } => {
                assert_eq!(c.name(), "snake_case");
            }
            _ => panic!("Expected ColumnsNameConvention rule"),
        }
    }

    #[test]
    fn test_naming_convention_kebab_case() {
        let convention = NamingConvention::from_pattern("kebab-case").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            convention,
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::NameConvention { convention: c } => {
                assert_eq!(c.name(), "kebab-case");
            }
            _ => panic!("Expected NameConvention rule"),
        }
    }

    #[test]
    fn test_naming_convention_camel_case() {
        let convention = NamingConvention::from_pattern("camelCase").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            convention,
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::NameConvention { convention: c } => {
                assert_eq!(c.name(), "camelCase");
            }
            _ => panic!("Expected NameConvention rule"),
        }
    }

    #[test]
    fn test_naming_convention_pascal_case() {
        let convention = NamingConvention::from_pattern("PascalCase").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            convention,
        );

        let config = InitConfig::from_questionnaire(&result);
        match &config.manifest_rules[0] {
            ManifestSpecificRuleConfig::NameConvention { convention: c } => {
                assert_eq!(c.name(), "PascalCase");
            }
            _ => panic!("Expected NameConvention rule"),
        }
    }

    // ========================================================================
    // YAML Serialization Tests
    // ========================================================================

    #[test]
    fn test_to_yaml_basic_structure() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        // Verify basic structure
        assert!(yaml.contains("# dbtective configuration file"));
        assert!(yaml.contains("manifest_tests:"));
        assert!(yaml.contains("catalog_tests:"));
        assert!(yaml.contains("has_description"));
        assert!(yaml.contains("columns_name_convention"));
    }

    #[test]
    fn test_to_yaml_has_description() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"name: "has_description""#));
        assert!(yaml.contains(r#"type: "has_description""#));
    }

    #[test]
    fn test_to_yaml_name_convention() {
        let convention = NamingConvention::from_pattern("snake_case").unwrap();
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            convention,
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"name: "naming_convention""#));
        assert!(yaml.contains(r#"type: "name_convention""#));
        assert!(yaml.contains(r#"pattern: "snake_case""#));
    }

    #[test]
    fn test_to_yaml_has_tags() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasTags {
                required_tags: vec![],
                criteria: HasTagsCriteria::default(),
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "has_tags""#));
        assert!(yaml.contains(r#""daily""#));
        assert!(yaml.contains(r#""monthly""#));
        assert!(yaml.contains(r#""yearly""#));
        assert!(yaml.contains(r#""inactive""#));
        assert!(yaml.contains(r#"criteria: "one_of""#));
    }

    #[test]
    fn test_to_yaml_allowed_subfolders_medallion() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "allowed_subfolders""#));
        assert!(yaml.contains(r#""bronze""#));
        assert!(yaml.contains(r#""silver""#));
        assert!(yaml.contains(r#""gold""#));
    }

    #[test]
    fn test_to_yaml_columns_canonical_name() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsCanonicalName {
                canonical: String::new(),
                invalid_names: vec![],
                exceptions: None,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains(r#"type: "columns_canonical_name""#));
        assert!(yaml.contains(r#"canonical: "user_id""#));
        assert!(yaml.contains(r#""userid""#));
        assert!(yaml.contains(r#""UserId""#));
    }

    // ========================================================================
    // TOML Serialization Tests
    // ========================================================================

    #[test]
    fn test_to_toml_basic_structure() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains("# dbtective configuration file"));
        assert!(toml.contains("[[manifest_tests]]"));
        assert!(toml.contains("[[catalog_tests]]"));
    }

    #[test]
    fn test_to_toml_has_description() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains(r"[[manifest_tests]]"));
        assert!(toml.contains(r#"name = "has_description""#));
        assert!(toml.contains(r#"type = "has_description""#));
    }

    #[test]
    fn test_to_toml_allowed_subfolders() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            vec![ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();

        assert!(toml.contains(r#"type = "allowed_subfolders""#));
        assert!(toml.contains(r#""staging""#));
        assert!(toml.contains(r#""marts""#));
        assert!(toml.contains(r#""warehouse""#));
    }

    // ========================================================================
    // Pyproject Serialization Tests
    // ========================================================================

    #[test]
    fn test_to_pyproject_basic_structure() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let pyproject = config.to_pyproject();

        assert!(pyproject.contains("# dbtective configuration"));
        assert!(pyproject.contains("[tool.dbtective]"));
        assert!(pyproject.contains("[[tool.dbtective.manifest_tests]]"));
        assert!(pyproject.contains("[[tool.dbtective.catalog_tests]]"));
    }

    #[test]
    fn test_to_pyproject_has_description() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let pyproject = config.to_pyproject();

        assert!(pyproject.contains(r"[[tool.dbtective.manifest_tests]]"));
        assert!(pyproject.contains(r#"name = "has_description""#));
        assert!(pyproject.contains(r#"type = "has_description""#));
    }

    // ========================================================================
    // Rule Ordering Tests (EnumIter)
    // ========================================================================

    #[test]
    fn test_manifest_rules_order_matches_enum_iter() {
        use strum::IntoEnumIterator;
        let all_rules: Vec<_> = ManifestSpecificRuleConfig::iter().collect();

        let result = create_questionnaire_result(
            LayeringStrategy::None,
            all_rules.clone(),
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        // Verify all rules are present
        assert_eq!(config.manifest_rules.len(), all_rules.len());

        // Rules should be in the same order as EnumIter (alphabetical)
        assert!(matches!(
            config.manifest_rules[0],
            ManifestSpecificRuleConfig::AllowedSubfolders { .. }
        ));
        assert!(matches!(
            config.manifest_rules[1],
            ManifestSpecificRuleConfig::CodeContainsRefs {}
        ));
    }

    #[test]
    fn test_catalog_rules_order_matches_enum_iter() {
        use strum::IntoEnumIterator;
        let all_rules: Vec<_> = CatalogSpecificRuleConfig::iter().collect();

        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            all_rules.clone(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);

        // Verify all rules are present
        assert_eq!(config.catalog_rules.len(), all_rules.len());
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[test]
    fn test_empty_manifest_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 0);
        assert_eq!(config.catalog_rules.len(), 1);

        let yaml = config.to_yaml();
        assert!(yaml.contains("manifest_tests:"));
        assert!(yaml.contains("catalog_tests:"));
    }

    #[test]
    fn test_empty_catalog_rules() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 1);
        assert_eq!(config.catalog_rules.len(), 0);
    }

    #[test]
    fn test_all_manifest_rules_enabled() {
        use strum::IntoEnumIterator;
        let all_rules: Vec<_> = ManifestSpecificRuleConfig::iter().collect();

        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            all_rules,
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.manifest_rules.len(), 18); // All 18 manifest rules
    }

    #[test]
    fn test_all_catalog_rules_enabled() {
        use strum::IntoEnumIterator;
        let all_rules: Vec<_> = CatalogSpecificRuleConfig::iter().collect();

        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            all_rules,
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.catalog_rules.len(), 5); // All 5 catalog rules
    }

    // ========================================================================
    // Methodology Rule Generation Tests
    // ========================================================================

    #[test]
    fn test_kimball_medallion_generates_dimension_subfolder_rule() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "gold_subfolders""#));
        assert!(yaml.contains(r#""dimensions""#));
        assert!(yaml.contains(r#""facts""#));
        assert!(yaml.contains(r#"includes: ["models/gold"]"#));
    }

    #[test]
    fn test_kimball_medallion_generates_naming_rules() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "dimension_naming""#));
        assert!(yaml.contains(r#"pattern: "^dim_""#));
        assert!(yaml.contains(r#"includes: ["models/gold/dimensions"]"#));
        assert!(yaml.contains(r#"name: "fact_naming""#));
        assert!(yaml.contains(r#"pattern: "^fact_""#));
        assert!(yaml.contains(r#"includes: ["models/gold/facts"]"#));
    }

    #[test]
    fn test_kimball_common_uses_warehouse_for_dims_facts() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        // Dims and facts live in the warehouse (middle) layer
        assert!(yaml.contains(r#"name: "warehouse_subfolders""#));
        assert!(yaml.contains(r#"includes: ["models/warehouse"]"#));
        assert!(yaml.contains(r#"includes: ["models/warehouse/dimensions"]"#));
        assert!(yaml.contains(r#"includes: ["models/warehouse/facts"]"#));
    }

    #[test]
    fn test_data_vault_medallion_generates_vault_subfolders() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "silver_vault_subfolders""#));
        assert!(yaml.contains(r#""hubs""#));
        assert!(yaml.contains(r#""links""#));
        assert!(yaml.contains(r#""satellites""#));
        assert!(yaml.contains(r#""t_links""#));
        assert!(yaml.contains(r#"includes: ["models/silver"]"#));
    }

    #[test]
    fn test_data_vault_medallion_generates_hub_naming_rule() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "hub_naming""#));
        assert!(yaml.contains(r#"pattern: "^hub_""#));
        assert!(yaml.contains(r#"includes: ["models/silver/hubs"]"#));
    }

    #[test]
    fn test_data_vault_generates_all_naming_rules() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"pattern: "^hub_""#));
        assert!(yaml.contains(r#"pattern: "^lnk_""#));
        assert!(yaml.contains(r#"pattern: "^sat_""#));
        assert!(yaml.contains(r#"pattern: "^t_lnk_""#));
        assert!(yaml.contains(r#"name: "vault_contracts""#));
        assert!(yaml.contains(r#"name: "hub_unique_keys""#));
        assert!(yaml.contains(r#"name: "link_unique_keys""#));
    }

    #[test]
    fn test_data_vault_generates_info_mart_naming() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "info_mart_dim_naming""#));
        assert!(yaml.contains(r#"name: "info_mart_fact_naming""#));
        assert!(yaml.contains(r#"includes: ["models/gold/information_mart"]"#));
    }

    #[test]
    fn test_data_vault_common_uses_intermediate_layer() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "warehouse_vault_subfolders""#));
        assert!(yaml.contains(r#"includes: ["models/warehouse"]"#));
        assert!(yaml.contains(r#"includes: ["models/warehouse/hubs"]"#));
    }

    #[test]
    fn test_inmon_medallion_generates_enterprise_subfolders() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Inmon,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "silver_subfolders""#));
        assert!(yaml.contains(r#""core""#));
        assert!(yaml.contains(r#""reference""#));
        assert!(yaml.contains(r#""bridge""#));
    }

    #[test]
    fn test_inmon_generates_department_subfolders() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Inmon,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "gold_subfolders""#));
        assert!(yaml.contains(r#""finance""#));
        assert!(yaml.contains(r#""marketing""#));
        assert!(yaml.contains(r#""operations""#));
    }

    #[test]
    fn test_inmon_generates_contract_and_unique_rules() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Inmon,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"name: "enterprise_model_contracts""#));
        assert!(yaml.contains(r#"name: "enterprise_model_unique_keys""#));
    }

    #[test]
    fn test_none_methodology_generates_no_extra_rules() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        // Should not contain any methodology-specific rule names
        assert!(!yaml.contains("dimension_naming"));
        assert!(!yaml.contains("fact_naming"));
        assert!(!yaml.contains("hub_naming"));
        assert!(!yaml.contains("enterprise_model"));
    }

    #[test]
    fn test_kimball_medallion_toml_output() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();
        assert!(toml.contains(r#"name = "gold_subfolders""#));
        assert!(toml.contains(r#"name = "dimension_naming""#));
        assert!(toml.contains(r#"name = "fact_naming""#));
        assert!(toml.contains(r#"pattern = "^dim_""#));
        assert!(toml.contains(r#"pattern = "^fact_""#));
    }

    #[test]
    fn test_data_vault_common_toml_output() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();
        assert!(toml.contains(r#"name = "warehouse_vault_subfolders""#));
        assert!(toml.contains(r#"name = "hub_naming""#));
        assert!(toml.contains(r#"name = "vault_contracts""#));
    }

    // ========================================================================
    // Layer Names Helper Tests
    // ========================================================================

    #[test]
    fn test_layer_names_medallion() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            Vec::new(),
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.layer_names(), ("gold", "silver"));
    }

    #[test]
    fn test_layer_names_common() {
        let result = create_questionnaire_result(
            LayeringStrategy::Common,
            Vec::new(),
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.layer_names(), ("marts", "warehouse"));
    }

    #[test]
    fn test_layer_names_none_defaults_to_common() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            Vec::new(),
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        assert_eq!(config.layer_names(), ("marts", "warehouse"));
    }

    // ========================================================================
    // IsNotOrphaned Methodology-Aware Tests
    // ========================================================================

    #[test]
    fn test_is_not_orphaned_kimball_scoped_to_facts() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"includes: ["models/gold/facts"]"#));
    }

    #[test]
    fn test_is_not_orphaned_data_vault_scoped_to_info_mart() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"includes: ["models/gold/information_mart"]"#));
    }

    #[test]
    fn test_is_not_orphaned_inmon_scoped_to_full_layer() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Inmon,
            vec![ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: vec![],
            }],
            Vec::new(),
            NamingConvention::default(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains(r#"includes: ["models/gold"]"#));
    }

    // ========================================================================
    // Section Banner Tests
    // ========================================================================

    #[test]
    fn test_section_banner_format() {
        let banner = InitConfig::section_banner("documentation", "  ");
        assert!(banner.contains("## documentation ##"));
        assert!(banner.starts_with("  #####"));
        // Verify top and bottom borders match inner width
        let lines: Vec<_> = banner.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].trim(), lines[2].trim());
        assert_eq!(lines[0].trim().len(), lines[1].trim().len());
    }

    #[test]
    fn test_yaml_output_contains_section_banners() {
        let result = create_questionnaire_result(
            LayeringStrategy::Medallion,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::HasTags {
                    required_tags: vec![],
                    criteria: HasTagsCriteria::default(),
                },
                ManifestSpecificRuleConfig::HasUniqueTest {
                    allowed_test_names: vec![],
                },
                ManifestSpecificRuleConfig::IsNotOrphaned {
                    allowed_references: vec![],
                },
                ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 0 },
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
            ],
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        // Verify manifest section banners
        assert!(yaml.contains("## documentation ##"));
        assert!(yaml.contains("## naming ##"));
        assert!(yaml.contains("## governance ##"));
        assert!(yaml.contains("## testing ##"));
        assert!(yaml.contains("## structure ##"));
        assert!(yaml.contains("## performance ##"));

        // Verify catalog section banners
        let catalog_section = yaml.split("catalog_tests:").nth(1).unwrap();
        assert!(catalog_section.contains("## documentation ##"));
        assert!(catalog_section.contains("## naming ##"));
    }

    #[test]
    fn test_yaml_output_contains_data_modeling_banner() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(yaml.contains("## Data Model/Layering Specific ##"));
        assert!(yaml.contains("dimension_naming"));
        assert!(yaml.contains("fact_naming"));
    }

    #[test]
    fn test_toml_output_contains_section_banners() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::HasTags {
                    required_tags: vec![],
                    criteria: HasTagsCriteria::default(),
                },
            ],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let config = InitConfig {
            format: ConfigFormat::Toml,
            ..config
        };
        let toml = config.to_toml();

        assert!(toml.contains("## documentation ##"));
        assert!(toml.contains("## governance ##"));
        // Banners should not be indented in TOML
        assert!(toml.contains("\n## documentation ##\n"));
    }

    #[test]
    fn test_yaml_banners_are_indented() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        // YAML banners should be indented with 2 spaces
        assert!(yaml.contains("  ## documentation ##"));
    }

    #[test]
    fn test_empty_category_does_not_produce_banner() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        // Only Documentation category has rules, other banners should not appear
        assert!(yaml.contains("## documentation ##"));
        assert!(!yaml.contains("## naming ##"));
        assert!(!yaml.contains("## governance ##"));
        assert!(!yaml.contains("## testing ##"));
        assert!(!yaml.contains("## structure ##"));
        assert!(!yaml.contains("## performance ##"));
    }

    #[test]
    fn test_no_data_modeling_banner_when_no_methodology() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            }],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        assert!(!yaml.contains("## Data Model/Layering Specific ##"));
    }

    #[test]
    fn test_rules_grouped_under_correct_banner() {
        let result = create_questionnaire_result(
            LayeringStrategy::None,
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::CodeMaxLines { max_lines: 0 },
            ],
            Vec::new(),
            NamingConvention::default(),
        );

        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();

        // Documentation banner should come before Performance banner
        let doc_pos = yaml.find("## documentation ##").unwrap();
        let perf_pos = yaml.find("## performance ##").unwrap();
        assert!(doc_pos < perf_pos);

        // has_description should come after documentation banner and before performance banner
        let desc_pos = yaml.find("has_description").unwrap();
        assert!(desc_pos > doc_pos);
        assert!(desc_pos < perf_pos);

        // code_max_lines should come after performance banner
        let max_lines_pos = yaml.find("code_max_lines").unwrap();
        assert!(max_lines_pos > perf_pos);
    }

    // ========================================================================
    // Helper Function Tests (to_camel_case, to_pascal_case)
    // ========================================================================

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("dim"), "dim");
        assert_eq!(to_camel_case("t_lnk"), "tLnk");
        assert_eq!(to_camel_case("hub"), "hub");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("dim"), "Dim");
        assert_eq!(to_pascal_case("t_lnk"), "TLnk");
        assert_eq!(to_pascal_case("hub"), "Hub");
    }

    // ========================================================================
    // prefix_for_convention Tests
    // ========================================================================

    #[test]
    fn test_prefix_for_convention_snake_case() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("snake_case").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let (pattern, display) = config.prefix_for_convention("dim");
        assert_eq!(pattern, "^dim_");
        assert_eq!(display, "dim_");
    }

    #[test]
    fn test_prefix_for_convention_kebab_case() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("kebab-case").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let (pattern, display) = config.prefix_for_convention("dim");
        assert_eq!(pattern, "^dim-");
        assert_eq!(display, "dim-");

        let (pattern, display) = config.prefix_for_convention("t_lnk");
        assert_eq!(pattern, "^t-lnk-");
        assert_eq!(display, "t-lnk-");
    }

    #[test]
    fn test_prefix_for_convention_camel_case() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("camelCase").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let (pattern, display) = config.prefix_for_convention("dim");
        assert_eq!(pattern, "^dim[A-Z]");
        assert_eq!(display, "dim");

        let (pattern, display) = config.prefix_for_convention("t_lnk");
        assert_eq!(pattern, "^tLnk[A-Z]");
        assert_eq!(display, "tLnk");
    }

    #[test]
    fn test_prefix_for_convention_pascal_case() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("PascalCase").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let (pattern, display) = config.prefix_for_convention("dim");
        assert_eq!(pattern, "^Dim[A-Z]");
        assert_eq!(display, "Dim");

        let (pattern, display) = config.prefix_for_convention("t_lnk");
        assert_eq!(pattern, "^TLnk[A-Z]");
        assert_eq!(display, "TLnk");
    }

    // ========================================================================
    // Methodology + Naming Convention Combination Tests
    // ========================================================================

    #[test]
    fn test_kimball_with_kebab_case_yaml() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("kebab-case").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains("^dim-"));
        assert!(yaml.contains("^fact-"));
    }

    #[test]
    fn test_kimball_with_camel_case_yaml() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::Kimball,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("camelCase").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains("^dim[A-Z]"));
        assert!(yaml.contains("^fact[A-Z]"));
    }

    #[test]
    fn test_data_vault_with_pascal_case_yaml() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Common,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("PascalCase").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let yaml = config.to_yaml();
        assert!(yaml.contains("^Hub[A-Z]"));
        assert!(yaml.contains("^Lnk[A-Z]"));
        assert!(yaml.contains("^Sat[A-Z]"));
        assert!(yaml.contains("^TLnk[A-Z]"));
    }

    #[test]
    fn test_data_vault_with_kebab_case_toml() {
        let result = create_questionnaire_result_with_methodology(
            LayeringStrategy::Medallion,
            DataModelMethodology::DataVault,
            vec![ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            }],
            Vec::new(),
            NamingConvention::from_pattern("kebab-case").unwrap(),
        );
        let config = InitConfig::from_questionnaire(&result);
        let toml = config.to_toml();
        assert!(toml.contains("^hub-"));
        assert!(toml.contains("^lnk-"));
        assert!(toml.contains("^sat-"));
        assert!(toml.contains("^t-lnk-"));
    }
}
