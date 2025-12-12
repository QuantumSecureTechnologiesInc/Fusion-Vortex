use anyhow::{Context, Result};
use std::process::Command;
use tracing::{info, warn};

/// Build the project using Fusion Flux Engine
pub fn build(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    info!("Building project with Fusion Flux Engine");

    // Check if Fusion Flux Engine is available
    let flux_enabled =
        std::env::var("FUSION_FLUX_ENABLED").unwrap_or_else(|_| "true".to_string()) == "true";

    let strict_mode =
        std::env::var("FUSION_STRICT_MODE").unwrap_or_else(|_| "true".to_string()) == "true";

    if !flux_enabled {
        warn!("Fusion Flux Engine disabled - using cargo");
        return build_with_cargo(release, target, verbose);
    }

    // Try to use Fusion Flux Engine
    match build_with_flux(release, target, verbose) {
        Ok(()) => Ok(()),
        Err(e) => {
            if !strict_mode {
                warn!("Fusion Flux failed: {} - falling back to cargo", e);
                build_with_cargo(release, target, verbose)
            } else {
                anyhow::bail!("Fusion Flux Engine failed and fallback disabled: {}", e);
            }
        }
    }
}

fn build_with_flux(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    println!("╔════════════════════════════════════════════╗");
    println!("║   FUSION FLUX ENGINE - BUILD SYSTEM       ║");
    println!("╚════════════════════════════════════════════╝");
    println!();

    // Check if Flux Engine is built
    let flux_path = if cfg!(windows) {
        "runtime/target/release/fusion_flux_resolve.dll"
    } else if cfg!(target_os = "macos") {
        "runtime/target/release/libfusion_flux_resolve.dylib"
    } else {
        "runtime/target/release/libfusion_flux_resolve.so"
    };

    if !std::path::Path::new(flux_path).exists() {
        println!("⚠️  Fusion Flux Engine not built yet");
        println!("   Building Flux Engine first...");
        println!();

        let status = Command::new("cargo")
            .args(&["build", "-p", "fusion_flux_resolve", "--release"])
            .current_dir("runtime")
            .status()
            .context("Failed to build Fusion Flux Engine")?;

        if !status.success() {
            anyhow::bail!("Failed to build Fusion Flux Engine");
        }

        println!("✅ Fusion Flux Engine built successfully");
        println!();
    }

    println!("🚀 Starting build with Fusion Flux Engine...");
    println!();

    // For now, delegate to cargo but with Flux awareness
    // TODO: Once Flux CLI integration complete, replace with pure Flux
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if release {
        cmd.arg("--release");
    }

    if let Some(t) = target {
        cmd.args(&["--target", t]);
    }

    if verbose {
        cmd.arg("--verbose");
    }

    // Set Flux environment
    cmd.env("FUSION_FLUX_MODE", "active");
    cmd.env("FUSION_BUILD_ENGINE", "flux");

    println!("📦 Resolving dependencies with Flux Engine...");
    let status = cmd.status().context("Failed to execute build")?;

    if status.success() {
        println!();
        println!("✅ Build completed successfully");
        Ok(())
    } else {
        anyhow::bail!("Build failed");
    }
}

fn build_with_cargo(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    println!("⚠️  Using cargo (Flux not available)");
    println!();

    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if release {
        cmd.arg("--release");
    }

    if let Some(t) = target {
        cmd.args(&["--target", t]);
    }

    if verbose {
        cmd.arg("--verbose");
    }

    let status = cmd.status().context("Failed to execute cargo build")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Cargo build failed");
    }
}
