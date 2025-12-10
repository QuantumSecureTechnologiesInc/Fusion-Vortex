#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::path::PathBuf;

pub mod cli;
pub mod downloader;
pub mod lockfile;
pub mod manifest;
pub mod registry;
pub mod resolver;
pub mod utils;

// Note: Re-exports available but currently unused
// pub use lockfile::{LockFile, LockedPackage, PackageSource};
// pub use manifest::{Manifest, PackageInfo};

/// Package structure
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<Dependency>,
}

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid version format: {}", s));
        }

        let major = parts[0].parse().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse().map_err(|_| "Invalid patch version")?;

        Ok(Version::new(major, minor, patch))
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_req: VersionRequirement,
}

/// Version requirement (simplified semver)
#[derive(Debug, Clone)]
pub enum VersionRequirement {
    Exact(Version),
    Caret(Version), // ^1.2.3 (compatible)
    Tilde(Version), // ~1.2.3 (patch updates)
    Any,
}

impl VersionRequirement {
    pub fn matches(&self, version: &Version) -> bool {
        match self {
            VersionRequirement::Exact(req) => version == req,
            VersionRequirement::Caret(req) => version.major == req.major && version >= req,
            VersionRequirement::Tilde(req) => {
                version.major == req.major && version.minor == req.minor && version >= req
            }
            VersionRequirement::Any => true,
        }
    }
}

/// Package manager main structure
pub struct PackageManager {
    cache_dir: PathBuf,
    registry_url: String,
    installed_packages: HashMap<String, Package>,
}

impl PackageManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        PackageManager {
            cache_dir,
            registry_url: "https://packages.fusion-lang.org".to_string(),
            installed_packages: HashMap::new(),
        }
    }

    /// Install a package and its dependencies
    pub fn install(&mut self, name: &str, version_req: VersionRequirement) -> Result<(), String> {
        // TODO: Implement installation logic
        // 1. Resolve dependencies
        // 2. Download packages
        // 3. Verify checksums
        // 4. Extract to cache
        // 5. Update manifest

        println!("Installing {} with requirement: {:?}", name, version_req);
        Ok(())
    }

    /// Update all packages
    pub fn update(&mut self) -> Result<(), String> {
        // TODO: Check for updates and install
        Ok(())
    }

    /// Remove a package
    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        // TODO: Remove package and unused dependencies
        self.installed_packages.remove(name);
        Ok(())
    }

    /// List installed packages
    pub fn list(&self) -> Vec<&Package> {
        self.installed_packages.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        assert!(v2 > v1);
    }

    #[test]
    fn test_caret_requirement() {
        let req = VersionRequirement::Caret(Version::new(1, 2, 3));

        assert!(req.matches(&Version::new(1, 2, 3)));
        assert!(req.matches(&Version::new(1, 3, 0)));
        assert!(!req.matches(&Version::new(2, 0, 0)));
    }
}
