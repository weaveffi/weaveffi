use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;

pub struct AndroidGenerator;

impl Generator for AndroidGenerator {
    fn name(&self) -> &'static str { "android" }
    fn generate(&self, _api: &Api, _out_dir: &Utf8Path) -> Result<()> {
        info!("generating Android JNI + Gradle template");
        Ok(())
    }
}
