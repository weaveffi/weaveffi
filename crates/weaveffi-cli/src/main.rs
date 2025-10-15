use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "weaveffi", version, about = "WeaveFFI CLI")] 
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New { name: String },
    Generate { #[arg(short, long, default_value = ".")] out: String },
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
        Commands::Generate { out } => cmd_generate(&out)?,
        Commands::Doctor => cmd_doctor()?,
    }
    Ok(())
}

fn cmd_new(_name: &str) -> Result<()> {
    // TODO: scaffold starter layout
    println!("new: not yet implemented");
    Ok(())
}

fn cmd_generate(_out: &str) -> Result<()> {
    // TODO: parse IDL, validate IR, run generators
    println!("generate: not yet implemented");
    Ok(())
}

fn cmd_doctor() -> Result<()> {
    println!("doctor: not yet implemented");
    Ok(())
}
