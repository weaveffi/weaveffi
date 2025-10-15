use anyhow::Result;
use camino::Utf8Path;
use tracing::info;
use weaveffi_core::codegen::Generator;
use weaveffi_core::templates::render_swift_wrapper;
use weaveffi_ir::ir::Api;

pub struct SwiftGenerator;

impl Generator for SwiftGenerator {
    fn name(&self) -> &'static str { "swift" }
    fn generate(&self, _api: &Api, out_dir: &Utf8Path) -> Result<()> {
        info!("generating SwiftPM System Library template");
        let dir = out_dir.join("swift");
        let module_dir = dir.join("WeaveFFI");
        std::fs::create_dir_all(&module_dir)?;
        // Package.swift
        let package = r#"// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: \"WeaveFFI\",
    products: [
        .library(name: \"WeaveFFI\", targets: [\"WeaveFFI\"]),
    ],
    targets: [
        .systemLibrary(name: \"WeaveFFI\", pkgConfig: nil)
    ]
)
"#;
        std::fs::write(dir.join("Package.swift"), package)?;
        // module.modulemap
        let modulemap = r#"module WeaveFFI [system] {
  header \"../../c/weaveffi.h\"
  link \"weaveffi\"
  export *
}
"#;
        std::fs::write(module_dir.join("module.modulemap"), modulemap)?;
        // Thin Swift wrapper generated from IR
        let swift_wrapper = render_swift_wrapper(_api);
        let src_dir = dir.join("Sources").join("WeaveFFI");
        std::fs::create_dir_all(&src_dir)?;
        std::fs::write(src_dir.join("WeaveFFI.swift"), swift_wrapper)?;
        Ok(())
    }
}
