#![allow(dead_code)]
#![allow(unused_variables)]

use super::Version;
use std::fs;
use std::path::PathBuf;

/// Package downloader
pub struct Downloader {
    cache_dir: PathBuf,
}

impl Downloader {
    pub fn new(cache_dir: PathBuf) -> Self {
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }

        Downloader { cache_dir }
    }

    /// Download package from registry
    pub fn download(&self, name: &str, version: &Version, url: &str) -> Result<PathBuf, String> {
        // TODO: Implement actual download logic
        // 1. Construct download URL
        // 2. Send HTTP request
        // 3. Stream to cache directory
        // 4. Verify checksum
        // 5. Extract if compressed

        let cache_path = self.cache_path(name, version);
        println!(
            "Downloading {} {} to {:?}...",
            name,
            version.to_string(),
            cache_path
        );

        Ok(cache_path)
    }

    /// Get cached package path
    pub fn cache_path(&self, name: &str, version: &Version) -> PathBuf {
        self.cache_dir.join(name).join(version.to_string())
    }

    /// Check if package is cached
    pub fn is_cached(&self, name: &str, version: &Version) -> bool {
        self.cache_path(name, version).exists()
    }

    /// Clear cache for a package
    pub fn clear_cache(&self, name: &str, version: Option<&Version>) -> Result<(), String> {
        let path = if let Some(v) = version {
            self.cache_path(name, v)
        } else {
            self.cache_dir.join(name)
        };

        if path.exists() {
            fs::remove_dir_all(&path).map_err(|e| format!("Failed to clear cache: {}", e))?;
        }

        Ok(())
    }

    /// Get cache size in bytes
    pub fn cache_size(&self) -> u64 {
        // TODO: Calculate total cache size
        0
    }

    /// List cached packages
    pub fn list_cached(&self) -> Vec<(String, Version)> {
        let mut packages = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    // TODO: Read versions for this package
                    packages.push((name, Version::new(1, 0, 0)));
                }
            }
        }

        packages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_downloader() {
        let cache_dir = env::temp_dir().join("fusion-test-cache");
        let downloader = Downloader::new(cache_dir);

        let name = "test-package";
        let version = Version::new(1, 0, 0);

        assert!(!downloader.is_cached(name, &version));
    }
}
