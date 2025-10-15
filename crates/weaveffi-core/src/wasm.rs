use anyhow::Result;
use camino::Utf8Path;
use crate::templates::{render_wasm_js_stub, render_wasm_readme};

pub fn write_minimal_wasm_stub(out_dir: &Utf8Path) -> Result<()> {
    let wasm_dir = out_dir.join("wasm");
    std::fs::create_dir_all(&wasm_dir)?;
    std::fs::write(wasm_dir.join("README.md"), render_wasm_readme())?;
    std::fs::write(wasm_dir.join("weaveffi_wasm.js"), render_wasm_js_stub())?;
    Ok(())
}
