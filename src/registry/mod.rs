// src/registry/mod.rs - Package Registry Infrastructure for v0.2.0
#![allow(dead_code)]
// REST API server, database integration, and package management

pub mod api;
pub mod auth;
pub mod database;
pub mod packages;
pub mod storage;

use std::error::Error;
use std::fmt;

/// Registry error types
#[derive(Debug, Clone)]
pub enum RegistryError {
    /// Package not found
    PackageNotFound(String),
    /// Version not found
    VersionNotFound(String, String),
    /// Authentication failed
    AuthenticationFailed(String),
    /// Authorization failed
    Unauthorized(String),
    /// Package already exists
    PackageExists(String),
    /// Invalid package format
    InvalidPackage(String),
    /// Database error
    DatabaseError(String),
    /// Storage error
    StorageError(String),
    /// Network error
    NetworkError(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::PackageNotFound(name) => write!(f, "Package not found: {}", name),
            RegistryError::VersionNotFound(name, ver) => {
                write!(f, "Version {} not found for package {}", ver, name)
            }
            RegistryError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            RegistryError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            RegistryError::PackageExists(name) => write!(f, "Package already exists: {}", name),
            RegistryError::InvalidPackage(msg) => write!(f, "Invalid package: {}", msg),
            RegistryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RegistryError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            RegistryError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl Error for RegistryError {}

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection string
    pub database_url: String,
    /// Storage backend
    pub storage_backend: StorageBackend,
    /// Storage path
    pub storage_path: String,
    /// Enable authentication
    pub require_auth: bool,
    /// Maximum package size (bytes)
    pub max_package_size: usize,
}

/// Storage backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    /// Local filesystem
    FileSystem,
    /// Amazon S3
    S3,
    /// Azure Blob Storage
    Azure,
    /// Google Cloud Storage
    GCS,
}

impl StorageBackend {
    pub fn name(&self) -> &'static str {
        match self {
            StorageBackend::FileSystem => "FileSystem",
            StorageBackend::S3 => "S3",
            StorageBackend::Azure => "Azure Blob",
            StorageBackend::GCS => "Google Cloud Storage",
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgresql://localhost/fusion_registry".to_string(),
            storage_backend: StorageBackend::FileSystem,
            storage_path: "./packages".to_string(),
            require_auth: true,
            max_package_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

impl RegistryConfig {
    /// Create production configuration
    pub fn production() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 443,
            require_auth: true,
            storage_backend: StorageBackend::S3,
            max_package_size: 100 * 1024 * 1024,
            ..Default::default()
        }
    }

    /// Create development configuration
    pub fn development() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            require_auth: false,
            storage_backend: StorageBackend::FileSystem,
            ..Default::default()
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), RegistryError> {
        if self.port == 0 {
            return Err(RegistryError::InvalidPackage(
                "Invalid port number".to_string(),
            ));
        }

        if self.max_package_size == 0 {
            return Err(RegistryError::InvalidPackage(
                "Invalid max package size".to_string(),
            ));
        }

        Ok(())
    }
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Authors
    pub authors: Vec<String>,
    /// License
    pub license: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// Keywords
    pub keywords: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    /// Creation timestamp
    pub created_at: u64,
    /// Download count
    pub downloads: u64,
}

impl PackageMetadata {
    /// Create new package metadata
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            authors: Vec::new(),
            license: None,
            repository: None,
            keywords: Vec::new(),
            dependencies: Vec::new(),
            created_at: current_timestamp(),
            downloads: 0,
        }
    }

    /// Add author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set license
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep: Dependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Get full package identifier
    pub fn full_name(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Version requirement
    pub version_req: String,
    /// Optional dependency
    pub optional: bool,
}

impl Dependency {
    pub fn new(name: impl Into<String>, version_req: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version_req: version_req.into(),
            optional: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

/// Registry statistics
#[derive(Debug, Default, Clone)]
pub struct RegistryStats {
    /// Total packages
    pub total_packages: usize,
    /// Total versions
    pub total_versions: usize,
    /// Total downloads
    pub total_downloads: u64,
    /// Total users
    pub total_users: usize,
    /// Storage used (bytes)
    pub storage_used: u64,
}

impl RegistryStats {
    /// Print statistics summary
    pub fn print_summary(&self) {
        println!("\n📦 Registry Statistics:");
        println!("  Packages: {}", self.total_packages);
        println!("  Versions: {}", self.total_versions);
        println!("  Downloads: {}", self.total_downloads);
        println!("  Users: {}", self.total_users);
        println!(
            "  Storage: {:.2} MB",
            self.storage_used as f64 / (1024.0 * 1024.0)
        );
    }
}

/// Get current timestamp
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_config_default() {
        let config = RegistryConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.require_auth);
    }

    #[test]
    fn test_registry_config_production() {
        let config = RegistryConfig::production();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 443);
        assert_eq!(config.storage_backend, StorageBackend::S3);
    }

    #[test]
    fn test_package_metadata() {
        let metadata = PackageMetadata::new("test-package", "1.0.0")
            .with_author("Test Author")
            .with_description("A test package")
            .with_license("MIT");

        assert_eq!(metadata.name, "test-package");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.authors.len(), 1);
        assert_eq!(metadata.full_name(), "test-package@1.0.0");
    }

    #[test]
    fn test_dependency() {
        let dep = Dependency::new("some-lib", "^1.0.0").optional();
        assert_eq!(dep.name, "some-lib");
        assert!(dep.optional);
    }

    #[test]
    fn test_storage_backend_names() {
        assert_eq!(StorageBackend::FileSystem.name(), "FileSystem");
        assert_eq!(StorageBackend::S3.name(), "S3");
    }
}
