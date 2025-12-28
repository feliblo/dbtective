use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Meta(pub Value);

impl Meta {
    pub fn missing_keys<'a>(&self, required_keys: &'a [String]) -> Vec<&'a String> {
        match &self.0 {
            Value::Object(map) => required_keys
                .iter()
                .filter(|key| !map.contains_key(key.as_str()))
                .collect(),
            _ => required_keys.iter().collect(),
        }
    }
}
