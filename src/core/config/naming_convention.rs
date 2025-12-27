use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use std::fmt;

/// Serde is used to deserialize conventions immediately into regex patterns.
///
/// Supports common conventions (`snake_case`, `kebab-case`, `camelCase`, `PascalCase`)
/// as well as custom regex patterns.
/// Patterns like `snake_case` are mapped to their corresponding regex patterns.
/// Custom regex patterns are used directly.
/// Erros are returned if the regex pattern is invalid.
#[derive(Debug, Clone)]
pub struct NamingConvention {
    pub regex: Regex,
    // Display name
    pub convention_name: String,
}

impl NamingConvention {
    /// Creates a new `NamingConvention` from a pattern string.
    /// Maps common naming conventions to regex patterns or uses the provided pattern directly.
    ///
    /// # Errors
    /// Returns an error if the pattern is an invalid regex.
    pub fn from_pattern(pattern: &str) -> Result<Self, regex::Error> {
        let (regex_str, convention) = match pattern {
            "snake_case" | "snakecase" => (r"^[a-z][a-z0-9_]*$", "snake_case"),
            "kebab_case" | "kebabcase" | "kebab-case" => (r"^[a-z][a-z0-9-]*$", "kebab-case"),
            "camelCase" | "camel_case" | "camelcase" => (r"^[a-z][a-zA-Z0-9]*$", "camelCase"),
            "pascal_case" | "pascalcase" | "pascal-case" | "PascalCase" => {
                (r"^[A-Z][a-zA-Z0-9]*$", "PascalCase")
            }
            _ => (pattern, pattern),
        };

        let regex = Regex::new(regex_str)?;
        Ok(Self {
            regex,
            convention_name: convention.to_string(),
        })
    }

    pub fn is_match(&self, name: &str) -> bool {
        self.regex.is_match(name)
    }

    pub fn name(&self) -> &str {
        &self.convention_name
    }
}

impl<'de> Deserialize<'de> for NamingConvention {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pattern = String::deserialize(deserializer)?;
        Self::from_pattern(&pattern)
            .map_err(|e| de::Error::custom(format!("Invalid regex pattern '{pattern}': {e}")))
    }
}

impl fmt::Display for NamingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.convention_name)
    }
}

impl Default for NamingConvention {
    fn default() -> Self {
        // Default to snake_case as it's the most common convention
        Self::from_pattern("snake_case").expect("snake_case is a valid pattern")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        let conv = NamingConvention::from_pattern("snake_case").unwrap();
        assert_eq!(conv.name(), "snake_case");
        assert!(conv.is_match("hello_world"));
        assert!(conv.is_match("test123"));
        assert!(!conv.is_match("HelloWorld"));
        assert!(!conv.is_match("hello-world"));
    }

    #[test]
    fn test_kebab_case() {
        let conv = NamingConvention::from_pattern("kebab-case").unwrap();
        assert_eq!(conv.name(), "kebab-case");
        assert!(conv.is_match("hello-world"));
        assert!(!conv.is_match("hello_world"));
    }

    #[test]
    fn test_camel_case() {
        let conv = NamingConvention::from_pattern("camelCase").unwrap();
        assert_eq!(conv.name(), "camelCase");
        assert!(conv.is_match("helloWorld"));
        assert!(!conv.is_match("HelloWorld"));
        assert!(!conv.is_match("hello_world"));
    }

    #[test]
    fn test_pascal_case() {
        let conv = NamingConvention::from_pattern("PascalCase").unwrap();
        assert_eq!(conv.name(), "PascalCase");
        assert!(conv.is_match("HelloWorld"));
        assert!(!conv.is_match("helloWorld"));
    }

    #[test]
    fn test_custom_regex() {
        let conv = NamingConvention::from_pattern(r"^[A-Z]{3}-[0-9]{4}$").unwrap();
        assert_eq!(conv.name(), r"^[A-Z]{3}-[0-9]{4}$");
        assert!(conv.is_match("ABC-1234"));
        assert!(!conv.is_match("AB-123"));
    }

    #[test]
    fn test_invalid_regex() {
        let result = NamingConvention::from_pattern("*[invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize() {
        let json = r#""snake_case""#;
        let conv: NamingConvention = serde_json::from_str(json).unwrap();
        assert_eq!(conv.name(), "snake_case");
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = r#""*[invalid""#;
        let result: Result<NamingConvention, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
