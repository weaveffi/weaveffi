use weaveffi_ir::ir::{Api, Module};

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("module has no name")] NoModuleName,
}

pub fn validate_api(api: &Api) -> Result<(), ValidationError> {
    for m in &api.modules {
        validate_module(m)?;
    }
    Ok(())
}

fn validate_module(module: &Module) -> Result<(), ValidationError> {
    if module.name.trim().is_empty() {
        return Err(ValidationError::NoModuleName);
    }
    Ok(())
}
