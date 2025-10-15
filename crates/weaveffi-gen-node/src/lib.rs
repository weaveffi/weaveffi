use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;

pub struct NodeGenerator;

impl Generator for NodeGenerator {
    fn name(&self) -> &'static str { "node" }
    fn generate(&self, _api: &Api, _out_dir: &Utf8Path) -> Result<()> {
        info!("generating Node.js ffi-napi template");
        Ok(())
    }
}
