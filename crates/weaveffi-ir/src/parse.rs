use crate::ir::Api;
use serde::de::DeserializeOwned;

fn from_str_format<T: DeserializeOwned>(s: &str, format: &str) -> Result<T, String> {
    match format {
        "yaml" | "yml" => serde_yaml::from_str(s).map_err(|e| e.to_string()),
        "json" => serde_json::from_str(s).map_err(|e| e.to_string()),
        "toml" => toml::from_str(s).map_err(|e| e.to_string()),
        _ => Err(format!("unsupported format: {}", format)),
    }
}

pub fn parse_api_str(s: &str, format: &str) -> Result<Api, String> {
    from_str_format::<Api>(s, format)
}
