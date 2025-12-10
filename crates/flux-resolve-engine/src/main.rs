use anyhow::{Context, Result};
use clap::Parser;
use flux_resolve_engine::resolve;
use std::path::PathBuf;

/// Simple CLI for the Flux Resolve Engine
#[derive(Parser, Debug)]
#[command(
    name = "flux-resolve-engine",
    version,
    about = "GPU‑accelerated dependency resolver (simplified)"
)]
struct Args {
    /// Path to a TOML manifest describing packages and their dependencies
    #[arg(short, long, value_name = "FILE")]
    manifest: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let manifest_path = &args.manifest;
    let manifest_str = std::fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read manifest {}", manifest_path.display()))?;
    let resolved = resolve(&manifest_str)?;
    println!("✅ Resolved order:");
    for pkg in resolved {
        println!("- {}", pkg);
    }
    Ok(())
}
