use crate::ir::Api;
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("YAML parse error at line {line}, column {column}: {message}")]
    Yaml { line: usize, column: usize, message: String },
    #[error("TOML parse error at line {line}, column {column}: {message}")]
    Toml { line: usize, column: usize, message: String },
    #[error("JSON parse error at line {line}, column {column}: {message}")]
    Json { line: usize, column: usize, message: String },
}

fn from_str_format<T: DeserializeOwned>(s: &str, format: &str) -> Result<T, ParseError> {
    match format {
        "yaml" | "yml" => serde_yaml::from_str(s).map_err(|e| {
            let (line, column) = e.location().map(|m| (m.line(), m.column())).unwrap_or((0, 0));
            ParseError::Yaml { line, column, message: e.to_string() }
        }),
        "json" => serde_json::from_str(s).map_err(|e| {
            ParseError::Json { line: e.line(), column: e.column(), message: e.to_string() }
        }),
        "toml" => toml::from_str(s).map_err(|e| {
            // toml::de::Error may not expose line/column in this version; include message only
            ParseError::Toml { line: 0, column: 0, message: e.to_string() }
        }),
        other => Err(ParseError::UnsupportedFormat(other.to_string())),
    }
}

pub fn parse_api_str(s: &str, format: &str) -> Result<Api, ParseError> {
    from_str_format::<Api>(s, format)
}
