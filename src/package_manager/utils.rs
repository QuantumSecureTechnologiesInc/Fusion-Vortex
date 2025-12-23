#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Package, Version, VersionRequirement};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Utility functions for package management
pub struct PackageUtils;

impl PackageUtils {
    /// Get default cache directory
    pub fn default_cache_dir() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".fusion").join("cache")
        } else if let Ok(userprofile) = std::env::var("USERPROFILE") {
            PathBuf::from(userprofile).join(".fusion").join("cache")
        } else {
            PathBuf::from(".fusion").join("cache")
        }
    }

    /// Get default config directory
    pub fn default_config_dir() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".fusion").join("config")
        } else if let Ok(userprofile) = std::env::var("USERPROFILE") {
            PathBuf::from(userprofile).join(".fusion").join("config")
        } else {
            PathBuf::from(".fusion").join("config")
        }
    }

    /// Find fusion.toml in current or parent directories
    pub fn find_manifest() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;

        loop {
            let manifest_path = current.join("fusion.toml");
            if manifest_path.exists() {
                return Some(manifest_path);
            }

            if !current.pop() {
                break;
            }
        }

        None
    }

    /// Get project root (directory containing fusion.toml)
    pub fn project_root() -> Option<PathBuf> {
        Self::find_manifest()?.parent().map(|p| p.to_path_buf())
    }

    /// Validate package name
    pub fn validate_package_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }

        if name.len() > 64 {
            return Err("Package name too long (max 64 characters)".to_string());
        }

        // Check for valid characters (lowercase, digits, hyphens)
        for c in name.chars() {
            if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-' && c != '_' {
                return Err(format!("Invalid character '{}' in package name", c));
            }
        }

        // Cannot start or end with hyphen
        if name.starts_with('-') || name.ends_with('-') {
            return Err("Package name cannot start or end with hyphen".to_string());
        }

        Ok(())
    }

    /// Validate version string
    pub fn validate_version(version: &str) -> Result<(), String> {
        Version::parse(version)?;
        Ok(())
    }

    /// Compute SHA256 checksum of file
    pub fn compute_checksum(path: &Path) -> Result<String, String> {
        let data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();

        Ok(format!("sha256:{:x}", result))
    }

    /// Verify file checksum
    pub fn verify_checksum(path: &Path, expected: &str) -> Result<bool, String> {
        let actual = Self::compute_checksum(path)?;
        Ok(actual == expected)
    }

    /// Pretty print package tree
    pub fn print_dependency_tree(package: &Package, indent: usize) {
        let prefix = "  ".repeat(indent);
        println!(
            "{}├─ {} v{}",
            prefix,
            package.name,
            package.version.to_string()
        );

        for dep in &package.dependencies {
            println!(
                "{}│  └─ {} {}",
                prefix,
                dep.name,
                format_version_req(&dep.version_req)
            );
        }
    }

    /// Format file size
    pub fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Create directory structure for package
    pub fn ensure_package_dir(
        cache_dir: &Path,
        name: &str,
        version: &Version,
    ) -> Result<PathBuf, String> {
        let pkg_dir = cache_dir.join(name).join(version.to_string());

        fs::create_dir_all(&pkg_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        Ok(pkg_dir)
    }
}

/// Format version requirement for display
pub fn format_version_req(req: &VersionRequirement) -> String {
    match req {
        VersionRequirement::Exact(v) => format!("={}", v.to_string()),
        VersionRequirement::Caret(v) => format!("^{}", v.to_string()),
        VersionRequirement::Tilde(v) => format!("~{}", v.to_string()),
        VersionRequirement::Any => "*".to_string(),
    }
}

/// Package name validator
pub struct PackageNameValidator;

impl PackageNameValidator {
    /// Reserved package names that cannot be used
    pub fn reserved_names() -> Vec<&'static str> {
        vec![
            "fusion", "std", "core", "test", "bench", "doc", "build", "target", "src", "tests",
            "examples", "benches",
        ]
    }

    /// Check if name is reserved
    pub fn is_reserved(name: &str) -> bool {
        Self::reserved_names().contains(&name)
    }

    /// Suggest alternative package name
    pub fn suggest_alternative(name: &str) -> String {
        if Self::is_reserved(name) {
            format!("{}-pkg", name)
        } else {
            name.to_lowercase()
                .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "-")
        }
    }
}

/// Version requirement parser
pub struct VersionReqParser;

impl VersionReqParser {
    /// Parse version requirement string
    pub fn parse(req_str: &str) -> Result<VersionRequirement, String> {
        let req_str = req_str.trim();

        if req_str == "*" || req_str.is_empty() {
            return Ok(VersionRequirement::Any);
        }

        if let Some(version_str) = req_str.strip_prefix("^") {
            let version = Version::parse(version_str)?;
            return Ok(VersionRequirement::Caret(version));
        }

        if let Some(version_str) = req_str.strip_prefix("~") {
            let version = Version::parse(version_str)?;
            return Ok(VersionRequirement::Tilde(version));
        }

        if let Some(version_str) = req_str.strip_prefix("=") {
            let version = Version::parse(version_str)?;
            return Ok(VersionRequirement::Exact(version));
        }

        // Default to caret requirement
        let version = Version::parse(req_str)?;
        Ok(VersionRequirement::Caret(version))
    }
}

#[cfg(test)]]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name() {
        assert!(PackageUtils::validate_package_name("my-package").is_ok());
        assert!(PackageUtils::validate_package_name("my_package").is_ok());
        assert!(PackageUtils::validate_package_name("package123").is_ok());

        assert!(PackageUtils::validate_package_name("").is_err());
        assert!(PackageUtils::validate_package_name("My-Package").is_err());
        assert!(PackageUtils::validate_package_name("-invalid").is_err());
        assert!(PackageUtils::validate_package_name("invalid-").is_err());
    }

    #[test]
    fn test_sha256_checksum() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_checksum.txt");

        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(b"Hello, Fusion!").unwrap();
        drop(file);

        let checksum = PackageUtils::compute_checksum(&test_file).unwrap();
        assert!(checksum.starts_with("sha256:"));
        assert!(checksum.len() > 7); // "sha256:" + hex

        // Verify checksum
        assert!(PackageUtils::verify_checksum(&test_file, &checksum).unwrap());

        fs::remove_file(test_file).ok();
    }

    #[test]
    fn test_reserved_names() {
        assert!(PackageNameValidator::is_reserved("fusion"));
        assert!(PackageNameValidator::is_reserved("std"));
        assert!(!PackageNameValidator::is_reserved("my-package"));
    }

    #[test]
    fn test_version_req_parser() {
        let req = VersionReqParser::parse("^1.2.3").unwrap();
        assert!(matches!(req, VersionRequirement::Caret(_)));

        let req = VersionReqParser::parse("~1.2.3").unwrap();
        assert!(matches!(req, VersionRequirement::Tilde(_)));

        let req = VersionReqParser::parse("=1.2.3").unwrap();
        assert!(matches!(req, VersionRequirement::Exact(_)));

        let req = VersionReqParser::parse("*").unwrap();
        assert!(matches!(req, VersionRequirement::Any));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(PackageUtils::format_size(100), "100 B");
        assert_eq!(PackageUtils::format_size(1024), "1.00 KB");
        assert_eq!(PackageUtils::format_size(1024 * 1024), "1.00 MB");
    }
}
