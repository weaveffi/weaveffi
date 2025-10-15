use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;

pub struct CGenerator;

impl Generator for CGenerator {
    fn name(&self) -> &'static str { "c-header" }
    fn generate(&self, _api: &Api, _out_dir: &Utf8Path) -> Result<()> {
        info!("generating C header template");
        Ok(())
    }
}
