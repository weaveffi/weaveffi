use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    pub version: String,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    /// Optional error domain for this module
    #[serde(default)]
    pub errors: Option<ErrorDomain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    /// Use key "return" in serialized formats
    #[serde(rename = "return")]
    pub returns: Option<TypeRef>,
    #[serde(default)]
    pub doc: Option<String>,
    /// Async not supported for 0.1.0; present for forward-compat
    #[serde(default, rename = "async")]
    pub r#async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TypeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeRef {
    #[serde(rename = "i32")] I32,
    #[serde(rename = "u32")] U32,
    #[serde(rename = "i64")] I64,
    #[serde(rename = "f64")] F64,
    #[serde(rename = "bool")] Bool,
    #[serde(rename = "string")] StringUtf8,
    #[serde(rename = "bytes")] Bytes,
    #[serde(rename = "handle")] Handle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDomain {
    pub name: String,
    pub codes: Vec<ErrorCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    /// Symbolic name, e.g. "InvalidInput"
    pub name: String,
    /// Numeric code (non-zero)
    pub code: i32,
    /// Human-readable message
    pub message: String,
}
