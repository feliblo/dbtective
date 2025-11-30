use strum_macros::EnumString;

#[derive(EnumString, Debug, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HasTagsCriteria {
    #[default]
    All,
    Any,
    OneOf,
}
