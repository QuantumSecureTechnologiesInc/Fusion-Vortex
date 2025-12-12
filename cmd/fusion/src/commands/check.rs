use anyhow::{Context, Result};
use std::process::Command;
use tracing::info;

/// Check the project using Fusion Flux Engine
pub fn check(all: bool) -> Result<()> {
    info!("Checking project with Fusion Flux Engine");

    println!("╔════════════════════════════════════════════╗");
    println!("║   FUSION FLUX ENGINE - FAST CHECK         ║");
    println!("╚════════════════════════════════════════════╝");
    println!();

    let flux_enabled =
        std::env::var("FUSION_FLUX_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

    let mut cmd = Command::new("cargo");
    cmd.arg("check");

    if all {
        cmd.arg("--all");
    }

    // Set Flux environment
    if flux_enabled {
        cmd.env("FUSION_FLUX_MODE", "active");
        cmd.env("FUSION_CHECK_ENGINE", "flux");
        println!("🔍 Fast checking with Flux resolution...");
    } else {
        println!("⚠️  Checking without Flux (disabled)");
    }

    println!();

    let status = cmd.status().context("Failed to execute check")?;

    if status.success() {
        println!();
        println!("✅ Check completed successfully");
        Ok(())
    } else {
        anyhow::bail!("Check failed");
    }
}
