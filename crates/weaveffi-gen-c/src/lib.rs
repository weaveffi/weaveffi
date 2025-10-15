use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_core::templates::{render_c_convenience_c, render_c_header};
use weaveffi_ir::ir::Api;

pub struct CGenerator;

impl Generator for CGenerator {
    fn name(&self) -> &'static str { "c-header" }
    fn generate(&self, api: &Api, out_dir: &Utf8Path) -> Result<()> {
        info!("generating C header template");
        let dir = out_dir.join("c");
        std::fs::create_dir_all(&dir)?;
        let header = render_c_header(api);
        std::fs::write(dir.join("weaveffi.h"), header)?;
        let c_shim = render_c_convenience_c();
        std::fs::write(dir.join("weaveffi.c"), c_shim)?;
        Ok(())
    }
}
