use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;

pub struct SwiftGenerator;

impl Generator for SwiftGenerator {
    fn name(&self) -> &'static str { "swift" }
    fn generate(&self, _api: &Api, _out_dir: &Utf8Path) -> Result<()> {
        info!("generating SwiftPM System Library template");
        Ok(())
    }
}
