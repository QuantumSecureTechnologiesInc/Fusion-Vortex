//! Configuration types for the Fusion Monolith Core
//!
//! This module provides all configuration options for the unified toolchain.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for the Fusion Monolith
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonolithConfig {
    /// Version string
    pub version: String,
    /// Project root directory
    pub project_root: PathBuf,
    /// Target directory for artifacts
    pub target_dir: PathBuf,
    /// Build profile (debug, release, etc.)
    pub profile: BuildProfile,
    /// LSP configuration
    pub lsp: LspConfig,
    /// Security auditor configuration
    pub security: SecurityConfig,
    /// TUI dashboard configuration
    pub tui: TuiConfig,
    /// Agent configuration
    pub agents: AgentConfig,
    /// CUDA/GPU configuration
    pub gpu: GpuConfig,
    /// Watch mode configuration
    pub watch: WatchConfig,
}

impl Default for MonolithConfig {
    fn default() -> Self {
        Self {
            version: crate::VERSION.to_string(),
            project_root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            target_dir: PathBuf::from("target"),
            profile: BuildProfile::Debug,
            lsp: LspConfig::default(),
            security: SecurityConfig::default(),
            tui: TuiConfig::default(),
            agents: AgentConfig::default(),
            gpu: GpuConfig::default(),
            watch: WatchConfig::default(),
        }
    }
}

impl MonolithConfig {
    /// Creates a new configuration with the specified project root
    pub fn with_project_root(project_root: PathBuf) -> Self {
        Self {
            project_root: project_root.clone(),
            target_dir: project_root.join("target"),
            ..Default::default()
        }
    }

    /// Load configuration from a fusion.toml file
    pub fn load_from_file(path: &std::path::Path) -> crate::error::MonolithResult<Self> {
        let content = std::fs::read_to_string(path)?;
        // For now, we use JSON. In production, use TOML.
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::error::MonolithResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Build profile configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildProfile {
    /// Debug build with full symbols and no optimizations
    Debug,
    /// Release build with full optimizations
    Release,
    /// Release build with debug info
    RelWithDebInfo,
    /// Minimum size release build
    MinSizeRel,
    /// Benchmarking profile
    Bench,
}

impl Default for BuildProfile {
    fn default() -> Self {
        Self::Debug
    }
}

impl std::fmt::Display for BuildProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "debug"),
            Self::Release => write!(f, "release"),
            Self::RelWithDebInfo => write!(f, "relwithdebinfo"),
            Self::MinSizeRel => write!(f, "minsizerel"),
            Self::Bench => write!(f, "bench"),
        }
    }
}

/// LSP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// Enable LSP server
    pub enabled: bool,
    /// LSP server port
    pub port: u16,
    /// Enable adaptive L1 caching
    pub adaptive_caching: bool,
    /// Cache size limit (number of entries)
    pub cache_size: usize,
    /// Minimum access count before caching
    pub cache_threshold: u64,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 9000,
            adaptive_caching: true,
            cache_size: 1000,
            cache_threshold: 3,
        }
    }
}

/// Security auditor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable security auditing
    pub enabled: bool,
    /// Path to advisory database
    pub advisory_db_path: Option<PathBuf>,
    /// Auto-update advisory database
    pub auto_update: bool,
    /// Fail build on vulnerabilities
    pub fail_on_vulnerability: bool,
    /// Minimum severity to fail on
    pub min_severity: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            advisory_db_path: None,
            auto_update: true,
            fail_on_vulnerability: true,
            min_severity: "medium".to_string(),
        }
    }
}

/// TUI dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Enable TUI dashboard
    pub enabled: bool,
    /// Refresh interval in milliseconds
    pub refresh_ms: u64,
    /// Show progress bars
    pub show_progress: bool,
    /// Show live logs
    pub show_logs: bool,
    /// Maximum log buffer size
    pub log_buffer_size: usize,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh_ms: 100,
            show_progress: true,
            show_logs: true,
            log_buffer_size: 1000,
        }
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Enable autonomous agents
    pub enabled: bool,
    /// Maximum concurrent agents
    pub max_concurrent: usize,
    /// Agent timeout in seconds
    pub timeout_secs: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent: 4,
            timeout_secs: 3600,
        }
    }
}

/// GPU/CUDA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Enable GPU acceleration
    pub enabled: bool,
    /// CUDA device index
    pub device_index: i32,
    /// Use GPU for dependency resolution (requires 10k+ deps)
    pub gpu_resolve: bool,
    /// Minimum dependency count for GPU resolution
    pub gpu_resolve_threshold: usize,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device_index: 0,
            gpu_resolve: false,
            gpu_resolve_threshold: 10000,
        }
    }
}

/// Watch mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    /// Debounce time in milliseconds
    pub debounce_ms: u64,
    /// Paths to watch (relative to project root)
    pub watch_paths: Vec<String>,
    /// Paths to ignore
    pub ignore_paths: Vec<String>,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 500,
            watch_paths: vec!["src".to_string(), "Cargo.toml".to_string()],
            ignore_paths: vec!["target".to_string(), ".git".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MonolithConfig::default();
        assert_eq!(config.profile, BuildProfile::Debug);
        assert!(config.lsp.enabled);
        assert!(config.security.enabled);
    }

    #[test]
    fn test_build_profile_display() {
        assert_eq!(BuildProfile::Release.to_string(), "release");
        assert_eq!(BuildProfile::Debug.to_string(), "debug");
    }
}
