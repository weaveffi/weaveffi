use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;
use weaveffi_core::templates::{render_node_dts};

pub struct NodeGenerator;

impl Generator for NodeGenerator {
    fn name(&self) -> &'static str { "node" }
    fn generate(&self, api: &Api, out_dir: &Utf8Path) -> Result<()> {
        info!("generating Node.js N-API loader and types");
        let dir = out_dir.join("node");
        std::fs::create_dir_all(&dir)?;
        // Simple loader that expects a compiled addon next to it
        std::fs::write(dir.join("index.js"), "module.exports = require('./index.node')\n")?;
        std::fs::write(dir.join("types.d.ts"), render_node_dts(api))?;
        std::fs::write(dir.join("package.json"), "{\n  \"name\": \"weaveffi\",\n  \"version\": \"0.1.0\",\n  \"main\": \"index.js\",\n  \"types\": \"types.d.ts\"\n}\n")?;
        Ok(())
    }
}
