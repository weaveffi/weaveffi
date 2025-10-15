use std::collections::{BTreeMap, BTreeSet};
use weaveffi_ir::ir::{Api, ErrorDomain, Function, Module, Param, TypeRef};

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("module has no name")] NoModuleName,
    #[error("duplicate module name: {0}")] DuplicateModuleName(String),
    #[error("invalid module name '{0}': {1}")] InvalidModuleName(String, &'static str),
    #[error("duplicate function name in module '{module}': {function}")]
    DuplicateFunctionName { module: String, function: String },
    #[error("duplicate param name in function '{function}' of module '{module}': {param}")]
    DuplicateParamName { module: String, function: String, param: String },
    #[error("reserved keyword used: {0}")] ReservedKeyword(String),
    #[error("async functions are not supported in 0.1.0: {module}::{function}")]
    AsyncNotSupported { module: String, function: String },
    #[error("error domain missing name in module '{0}'")]
    ErrorDomainMissingName(String),
    #[error("duplicate error code name in module '{module}': {name}")]
    DuplicateErrorName { module: String, name: String },
    #[error("duplicate error numeric code in module '{module}': {code}")]
    DuplicateErrorCode { module: String, code: i32 },
    #[error("invalid error code in module '{module}' for '{name}': must be non-zero")]
    InvalidErrorCode { module: String, name: String },
    #[error("function name collides with error domain name in module '{module}': {name}")]
    NameCollisionWithErrorDomain { module: String, name: String },
}

const RESERVED: &[&str] = &[
    "if", "else", "for", "while", "loop", "match", "type", "return", "async",
    "await", "break", "continue", "fn", "struct", "enum", "mod", "use",
];

pub fn validate_api(api: &Api) -> Result<(), ValidationError> {
    let mut module_names = BTreeSet::new();
    for m in &api.modules {
        if !module_names.insert(m.name.clone()) {
            return Err(ValidationError::DuplicateModuleName(m.name.clone()));
        }
        validate_module(m)?;
    }
    Ok(())
}

fn validate_module(module: &Module) -> Result<(), ValidationError> {
    if module.name.trim().is_empty() {
        return Err(ValidationError::NoModuleName);
    }
    if RESERVED.contains(&module.name.as_str()) {
        return Err(ValidationError::InvalidModuleName(module.name.clone(), "reserved word"));
    }

    let mut function_names = BTreeSet::new();
    for f in &module.functions {
        if !function_names.insert(f.name.clone()) {
            return Err(ValidationError::DuplicateFunctionName { module: module.name.clone(), function: f.name.clone() });
        }
        validate_function(module, f)?;
    }

    if let Some(errors) = &module.errors {
        validate_error_domain(module, errors, &function_names)?;
    }

    Ok(())
}

fn validate_function(module: &Module, f: &Function) -> Result<(), ValidationError> {
    if RESERVED.contains(&f.name.as_str()) {
        return Err(ValidationError::ReservedKeyword(f.name.clone()));
    }
    if f.r#async {
        return Err(ValidationError::AsyncNotSupported { module: module.name.clone(), function: f.name.clone() });
    }

    let mut param_names = BTreeSet::new();
    for p in &f.params {
        validate_param(module, &f.name, p)?;
        if !param_names.insert(p.name.clone()) {
            return Err(ValidationError::DuplicateParamName { module: module.name.clone(), function: f.name.clone(), param: p.name.clone() });
        }
    }

    // Returns type is already constrained by TypeRef enum; enforce no nested structs (not present in v0)
    if let Some(ret) = &f.returns {
        validate_type_ref(ret);
    }

    Ok(())
}

fn validate_param(_module: &Module, _function: &str, p: &Param) -> Result<(), ValidationError> {
    if RESERVED.contains(&p.name.as_str()) {
        return Err(ValidationError::ReservedKeyword(p.name.clone()));
    }
    validate_type_ref(&p.ty);
    Ok(())
}

fn validate_type_ref(_t: &TypeRef) {
    // Currently no-op since TypeRef only includes supported primitives/handles
}

fn validate_error_domain(module: &Module, errors: &ErrorDomain, function_names: &BTreeSet<String>) -> Result<(), ValidationError> {
    if errors.name.trim().is_empty() {
        return Err(ValidationError::ErrorDomainMissingName(module.name.clone()));
    }
    if function_names.contains(&errors.name) {
        return Err(ValidationError::NameCollisionWithErrorDomain { module: module.name.clone(), name: errors.name.clone() });
    }

    let mut by_name: BTreeSet<String> = BTreeSet::new();
    let mut by_code: BTreeMap<i32, String> = BTreeMap::new();
    for c in &errors.codes {
        if c.code == 0 {
            return Err(ValidationError::InvalidErrorCode { module: module.name.clone(), name: c.name.clone() });
        }
        if !by_name.insert(c.name.clone()) {
            return Err(ValidationError::DuplicateErrorName { module: module.name.clone(), name: c.name.clone() });
        }
        if let Some(existing) = by_code.insert(c.code, c.name.clone()) {
            let _ = existing; // keep clippy happy: we only need to know it existed
            return Err(ValidationError::DuplicateErrorCode { module: module.name.clone(), code: c.code });
        }
    }
    Ok(())
}
