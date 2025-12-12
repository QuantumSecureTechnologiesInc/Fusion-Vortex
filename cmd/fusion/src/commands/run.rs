use anyhow::{Context, Result};
use std::process::Command;
use tracing::info;

/// Run the project using Fusion Flux Engine
pub fn run(release: bool, args: &[String]) -> Result<()> {
    info!("Running project with Fusion Flux Engine");

    println!("╔════════════════════════════════════════════╗");
    println!("║   FUSION FLUX ENGINE - BUILD & RUN        ║");
    println!("╚════════════════════════════════════════════╝");
    println!();

    let flux_enabled =
        std::env::var("FUSION_FLUX_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

    // First build with Flux
    if flux_enabled {
        println!("🔨 Building with Fusion Flux Engine...");
        crate::commands::build::build(release, None, false)?;
        println!();
    }

    println!("🚀 Executing program...");
    println!();

    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    if release {
        cmd.arg("--release");
    }

    if !args.is_empty() {
        cmd.arg("--");
        cmd.args(args);
    }

    if flux_enabled {
        cmd.env("FUSION_FLUX_MODE", "active");
    }

    let status = cmd.status().context("Failed to execute run")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Run failed");
    }
}
