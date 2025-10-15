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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Option<TypeRef>,
    #[serde(default)]
    pub doc: Option<String>,
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
