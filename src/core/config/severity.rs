use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}
impl Severity {
    #[allow(dead_code)]
    pub const fn as_code(&self) -> u8 {
        match self {
            Self::Error => 1,
            Self::Warning => 0,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "FAIL",
            Self::Warning => "WARN",
        }
    }
}
