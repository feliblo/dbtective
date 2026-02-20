use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleCategory {
    Documentation,
    Naming,
    Testing,
    Governance,
    Structure,
    Performance,
}

impl RuleCategory {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Documentation => "documentation",
            Self::Naming => "naming",
            Self::Testing => "testing",
            Self::Governance => "governance",
            Self::Structure => "structure",
            Self::Performance => "performance",
        }
    }
}

impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
