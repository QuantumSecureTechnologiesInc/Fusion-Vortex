//! Dependency graph resolution for the Fusion build system.
use std::path::{Path, PathBuf};
use anyhow::Result;

pub struct BuildGraph {
    pub root_name: String,
    pub packages: Vec<PackageNode>,
}

pub struct PackageNode {
    pub name: String,
    pub path: PathBuf,
}

impl BuildGraph {
    pub fn topological_sort(&self) -> Result<Vec<&PackageNode>> {
        Ok(self.packages.iter().collect())
    }
}

pub fn resolve_dependencies(root: &Path) -> Result<BuildGraph> {
    Ok(BuildGraph {
        root_name: "main".to_string(),
        packages: vec![PackageNode {
            name: "main".to_string(),
            path: root.to_path_buf(),
        }],
    })
}