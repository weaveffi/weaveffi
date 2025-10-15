use anyhow::Result;
use camino::Utf8Path;
use weaveffi_ir::ir::Api;
use crate::wasm::write_minimal_wasm_stub;

pub trait Generator {
    fn name(&self) -> &'static str;
    fn generate(&self, api: &Api, out_dir: &Utf8Path) -> Result<()>;
}

pub struct Orchestrator<'a> {
    generators: Vec<&'a dyn Generator>,
}

impl<'a> Orchestrator<'a> {
    pub fn new() -> Self { Self { generators: Vec::new() } }
    pub fn with_generator(mut self, gen: &'a dyn Generator) -> Self {
        self.generators.push(gen);
        self
    }
    pub fn run(&self, api: &Api, out_dir: &Utf8Path) -> Result<()> {
        for g in &self.generators {
            g.generate(api, out_dir)?;
        }
        Ok(())
    }
}

pub struct WasmGenerator;

impl Generator for WasmGenerator {
    fn name(&self) -> &'static str { "wasm" }
    fn generate(&self, _api: &Api, out_dir: &Utf8Path) -> Result<()> {
        write_minimal_wasm_stub(out_dir)
    }
}
