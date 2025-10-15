use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_ir::ir::Api;
use weaveffi_core::templates::{render_node_index_ts, render_node_dts};

pub struct NodeGenerator;

impl Generator for NodeGenerator {
    fn name(&self) -> &'static str { "node" }
    fn generate(&self, api: &Api, out_dir: &Utf8Path) -> Result<()> {
        info!("generating Node.js ffi-napi template");
        let dir = out_dir.join("node");
        std::fs::create_dir_all(&dir)?;
        // package.json
        let package_json = r#"{
  \"name\": \"weaveffi\",
  \"version\": \"0.1.0\",
  \"type\": \"module\",
  \"dependencies\": { \"ffi-napi\": \"^4.0.3\", \"ref-napi\": \"^3.0.3\" },
  \"scripts\": { \"build\": \"echo 'no build'\" }
}
"#;
        std::fs::write(dir.join("package.json"), package_json)?;
        // index.ts and types.d.ts generated from IR
        std::fs::write(dir.join("index.ts"), render_node_index_ts(api))?;
        std::fs::write(dir.join("types.d.ts"), render_node_dts(api))?;
        Ok(())
    }
}
