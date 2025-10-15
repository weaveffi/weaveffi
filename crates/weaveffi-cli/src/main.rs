use anyhow::{bail, Context, Result};
use camino::Utf8Path;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;
use weaveffi_core::codegen::{Orchestrator, WasmGenerator};
use weaveffi_core::validate::validate_api;
use weaveffi_ir::parse::parse_api_str;
use weaveffi_ir::ir::Api;
use weaveffi_gen_c::CGenerator;
use weaveffi_gen_swift::SwiftGenerator;
use weaveffi_gen_android::AndroidGenerator;
use weaveffi_gen_node::NodeGenerator;

#[derive(Parser, Debug)]
#[command(name = "weaveffi", version, about = "WeaveFFI CLI")] 
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New { name: String },
    Generate {
        /// Input IDL/IR file (yaml|yml|json|toml)
        input: String,
        /// Output directory for generated artifacts
        #[arg(short, long, default_value = "./generated")] out: String,
    },
    Doctor,
}

fn main() -> Result<()> {
    color_eyre::install().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => cmd_new(&name)?,
        Commands::Generate { input, out } => cmd_generate(&input, &out)?,
        Commands::Doctor => cmd_doctor()?,
    }
    Ok(())
}

fn cmd_new(_name: &str) -> Result<()> {
    // TODO: scaffold starter layout
    println!("new: not yet implemented");
    Ok(())
}

fn cmd_generate(input: &str, out: &str) -> Result<()> {
    let in_path = std::path::Path::new(input);
    let ext = in_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let format = match ext {
        "yml" | "yaml" => "yaml",
        "json" => "json",
        "toml" => "toml",
        other => bail!("unsupported input format: {} (expected yml|yaml|json|toml)", other),
    };
    let contents = std::fs::read_to_string(in_path)
        .with_context(|| format!("failed to read input file: {}", input))?;
    let api: Api = parse_api_str(&contents, format)
        .with_context(|| format!("failed to parse {} as {}", input, format))?;
    validate_api(&api).context("IR validation failed")?;

    let out_dir = Utf8Path::new(out);
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output directory: {}", out))?;

    let orchestrator = Orchestrator::new()
        .with_generator(&CGenerator)
        .with_generator(&SwiftGenerator)
        .with_generator(&AndroidGenerator)
        .with_generator(&NodeGenerator)
        .with_generator(&WasmGenerator);

    orchestrator.run(&api, out_dir)?;
    println!("Generated artifacts in {}", out);
    Ok(())
}

fn cmd_doctor() -> Result<()> {
    println!("doctor: not yet implemented");
    Ok(())
}
