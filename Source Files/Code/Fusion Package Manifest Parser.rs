// src/pkg/manifest_parser.rs - Fusion.toml Manifest Parser

use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::result::Result as StdResult;

// --- Data Structures (Mirrors the Fusion.toml Schema) ---

// Corresponds to the [package] table
#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub edition: String, // e.g., "2025"
}

// Corresponds to the [dependencies] table
#[derive(Debug, Deserialize)]
pub struct Dependencies {
    #[serde(flatten)]
    pub packages: HashMap<String, DependencyDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allow flexible definition: dependency = "version" OR dependency = { ... }
pub enum DependencyDetail {
    Version(String), // e.g., fusion-web = "0.5"
    Detailed {
        version: String,
        features: Option<Vec<String>>,
        target: Option<String>,
    },
}

// Corresponds to the [build-options] table
#[derive(Debug, Deserialize)]
pub struct BuildOptions {
    pub target: String,
    pub opt_level: String,        // Optimization level for LLVM
    pub security_profile: String, // e.g., "FIPS_140_2_Strict"
}

// Top-level structure for Fusion.toml
#[derive(Debug, Deserialize)]
pub struct FusionManifest {
    pub package: PackageMetadata,
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "build-options")]
    pub build_options: BuildOptions,
}

// --- Parsing Utility ---

/// Reads and parses the Fusion.toml manifest from disk.
pub fn parse_manifest(path: &Path) -> StdResult<FusionManifest, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read manifest file {}: {}", path.display(), e))?;

    let manifest: FusionManifest =
        toml::from_str(&contents).map_err(|e| format!("Failed to parse TOML manifest: {}", e))?;

    // --- Semantic Validation ---
    // 1. Validate required fields (already done by serde)
    // 2. Validate build options against known compiler targets/profiles
    if manifest.build_options.opt_level != "Aggressive"
        && manifest.build_options.opt_level != "Default"
    {
        return Err(format!(
            "Invalid opt_level: {}. Must be 'Default' or 'Aggressive'.",
            manifest.build_options.opt_level
        ));
    }

    println!(
        "Manifest loaded successfully for '{}'.",
        manifest.package.name
    );
    Ok(manifest)
}

// --- Example Usage ---

/*
Example Fusion.toml file:
[package]
name = "quantum-suite"
version = "1.0.0"
edition = "2025"

[dependencies]
fusion-crypto = "2.1.0"
fusion-ml = { version = "0.9", features = ["gpu", "blas-link"] }

[build-options]
target = "x86-64"
opt_level = "Aggressive"
security_profile = "FIPS_140_2_Strict"
*/
