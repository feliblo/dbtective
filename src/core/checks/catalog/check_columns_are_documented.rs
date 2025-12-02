use crate::{
    cli::table::RuleResult,
    core::{
        checks::common_traits::Columnable, config::catalog_rule::CatalogRule, manifest::Manifest,
    },
};

#[allow(dead_code)]
fn check_columns_are_documented<T: Columnable>(
    columnable: &T,
    rule: &CatalogRule,
    manifest: &Manifest,
) -> Option<RuleResult> {
    let mut case_insensitive = false;

    if let Some(adapter_type) = &manifest.metadata.adapter_type {
        if adapter_type.eq_ignore_ascii_case("snowflake") {
            case_insensitive = true;
        }
    }

    println!(
        "Checking columns for {}: {}, {}, {}",
        T::get_object_type(columnable),
        T::get_object_string(columnable),
        rule.get_name(),
        case_insensitive
    );
    // TODO: Implement the rest of the function logic and return Some(RuleResult) or None as appropriate.
    todo!()
}
