//! Forge Build System & Package Manager
//! Addresses: No package manager, No build caching, No dependency graph.
//! Integrates the core Fusion Runtime Triad: Supernova, Nebula, Pulsar.
use crate::types::*;

use crate::codegen::CodegenConfig;

/// Represents the target runtime execution environment for the Fusion manifest.
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeTarget {
    /// Legacy, synchronous, deterministic execution kernel (v2.0)
    Nebula,
    /// High-performance, asynchronous, PQC-hardened, hardware-aware kernel (v3.0)
    Supernova,
    /// Ultra-lightweight, zero-dependency embedded/WASM target
    Pulsar,
}

/// Represents a package dependency in the dependency graph.
#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: FString,
    pub version: FString,
    pub path: Option<FString>,
}

/// Represents a parsed `fusion.toml` manifest file.
#[derive(Clone, Debug)]
pub struct ProjectManifest {
    pub name: FString,
    pub version: FString,
    pub authors: FVec<FString>,
    pub runtime: RuntimeTarget,
    pub dependencies: FVec<Dependency>,
    pub build_cache_enabled: FBool,
}

pub struct Forge {
    pub manifest: ProjectManifest,
    pub cache_dir: FString,
}

impl Forge {
    /// Initializes the Forge build system for a given project directory.
    pub fn new(project_root: &str) -> Result<Self, FString> {
        // Stub: In a full implementation, this parses fusion.toml.
        // Defaulting to Supernova as the flagship modern runtime.
        let manifest = ProjectManifest {
            name: "fusion_project".to_string(),
            version: "0.1.0".to_string(),
            authors: vec![],
            runtime: RuntimeTarget::Supernova,
            dependencies: vec![],
            build_cache_enabled: true,
        };

        Ok(Self {
            manifest,
            cache_dir: format!("{}/.fusion_cache", project_root),
        })
    }

    /// Resolves the dependency graph. (Addresses "No dependency graph")
    pub fn resolve_dependencies(&self) -> Result<FVec<FString>, FString> {
        let mut resolved_paths = Vec::new();
        for dep in &self.manifest.dependencies {
            // Placeholder for actual registry/git fetching
            let dep_path = dep.path.clone().unwrap_or_else(|| {
                format!("{}/{}-{}", self.cache_dir, dep.name, dep.version)
            });
            resolved_paths.push(dep_path);
        }
        Ok(resolved_paths)
    }

    /// Configures the codegen backend to link against the correct runtime.
    pub fn configure_target(&self, mut base_config: CodegenConfig) -> CodegenConfig {
        match self.manifest.runtime {
            RuntimeTarget::Supernova => {
                // Link asynchronous HyperRing and PQC NeuralSeal
                base_config.link_libs.push("supernova_rt".to_string());
                base_config.link_libs.push("qst_neuralseal_pqc".to_string());
            }
            RuntimeTarget::Nebula => {
                // Link legacy deterministic sync kernel
                base_config.link_libs.push("nebula_rt_v2".to_string());
            }
            RuntimeTarget::Pulsar => {
                // Strip dependencies for Wasm/Embedded, link minimal static PQC
                base_config.target_triple = "wasm32-unknown-unknown".to_string();
                base_config.link_libs.push("pulsar_micro_rt".to_string());
            }
        }
        base_config
    }
}