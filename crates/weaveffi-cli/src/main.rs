use anyhow::{bail, Context, Result};
use camino::Utf8Path;
use clap::{Parser, Subcommand};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
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

fn cmd_new(name: &str) -> Result<()> {
    let project_dir = Path::new(name);
    if project_dir.exists() {
        bail!("destination '{}' already exists; choose a new name or remove it", name);
    }
    fs::create_dir_all(project_dir)
        .with_context(|| format!("failed to create project directory: {}", name))?;

    let module_name = sanitize_module_name(name);
    let idl_path = project_dir.join("weaveffi.yml");
    let idl_contents = format!(
        concat!(
            "version: \"0.1.0\"\n",
            "modules:\n",
            "  - name: {module}\n",
            "    functions:\n",
            "      - name: add\n",
            "        params:\n",
            "          - {{ name: a, type: i32 }}\n",
            "          - {{ name: b, type: i32 }}\n",
            "        return: i32\n",
            "      - name: mul\n",
            "        params:\n",
            "          - {{ name: a, type: i32 }}\n",
            "          - {{ name: b, type: i32 }}\n",
            "        return: i32\n",
            "      - name: echo\n",
            "        params:\n",
            "          - {{ name: s, type: string }}\n",
            "        return: string\n"
        ),
        module = module_name
    );
    fs::write(&idl_path, idl_contents).with_context(|| format!("failed to write {}", idl_path.display()))?;

    let readme_path = project_dir.join("README.md");
    let readme = format!(
        concat!(
            "# {name}\n\n",
            "This project was bootstrapped with WeaveFFI.\n\n",
            "- Edit `weaveffi.yml` to define your API.\n",
            "- Generate outputs: `weaveffi generate weaveffi.yml -o ../generated` (or choose any out dir).\n",
            "- See docs for memory/error model and platform specifics.\n"
        ),
        name = name
    );
    fs::write(&readme_path, readme).with_context(|| format!("failed to write {}", readme_path.display()))?;

    println!("Initialized WeaveFFI project at {}", project_dir.display());
    println!("- IDL: {}", idl_path.display());
    println!("Next: run `weaveffi generate {}/weaveffi.yml -o generated`", name);
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
    println!("WeaveFFI Doctor: checking toolchain prerequisites\n");

    // Rust toolchain
    check_command_with_version("rustc", &["--version"], "Rust compiler", Some("Install via https://rustup.rs"));
    check_command_with_version("cargo", &["--version"], "Cargo (Rust package manager)", Some("Install via https://rustup.rs"));

    // Xcode (macOS only)
    if cfg!(target_os = "macos") {
        check_command_with_version("xcodebuild", &["-version"], "Xcode command-line tools", Some("Install Xcode from the App Store, then run `xcode-select --install`"));
    } else {
        println!("- Xcode: skipped (non-macOS)");
    }

    // Android NDK
    let ndk_hint = if cfg!(target_os = "macos") {
        Some("Install via Android Studio SDK Manager or `brew install android-ndk`. Set ANDROID_NDK_HOME.")
    } else {
        Some("Install via Android Studio SDK Manager. Set ANDROID_NDK_HOME.")
    };
    let ndk_ok = check_command_with_version("ndk-build", &["-v"], "Android NDK (ndk-build)", ndk_hint);
    if !ndk_ok {
        // fallback: environment variables
        let env_ok = env::var_os("ANDROID_NDK_HOME").map(|p| Path::new(&p).exists()).unwrap_or(false)
            || env::var_os("ANDROID_NDK_ROOT").map(|p| Path::new(&p).exists()).unwrap_or(false);
        if env_ok {
            println!("  note: ANDROID_NDK_HOME/ROOT is set and exists; ensure `ndk-build` is in PATH");
        }
    }

    // Node toolchain
    check_command_with_version("node", &["-v"], "Node.js", Some("Install from https://nodejs.org or with your package manager"));
    check_command_with_version("npm", &["-v"], "npm", Some("Install Node.js which includes npm, or use pnpm/yarn"));

    println!("\nDoctor completed. Address any missing items above.");
    Ok(())
}

fn sanitize_module_name(name: &str) -> String {
    let lowered = name.to_lowercase();
    let mut out = String::with_capacity(lowered.len());
    for ch in lowered.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else if matches!(ch, '-' | '_' | ' ' ) {
            out.push('_');
        }
        // drop any other characters
    }
    if out.is_empty() { String::from("module") } else { out }
}

fn check_command_with_version<S: AsRef<OsStr>>(cmd: &str, args: &[S], label: &str, hint: Option<&str>) -> bool {
    match Command::new(cmd).args(args).output() {
        Ok(out) => {
            if out.status.success() {
                let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if ver.is_empty() {
                    println!("- {}: OK ({})", label, cmd);
                } else {
                    println!("- {}: OK ({}: {})", label, cmd, ver);
                }
                true
            } else {
                println!("- {}: MISSING ({} exited with status {})", label, cmd, out.status);
                if let Some(h) = hint { println!("  hint: {}", h); }
                false
            }
        }
        Err(_) => {
            println!("- {}: MISSING ({} not found in PATH)", label, cmd);
            if let Some(h) = hint { println!("  hint: {}", h); }
            false
        }
    }
}
