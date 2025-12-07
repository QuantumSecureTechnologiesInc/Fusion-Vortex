#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Dependency, Package, Version, VersionRequirement};
use std::collections::{HashMap, HashSet};

/// Dependency resolver for package installation
pub struct DependencyResolver {
    packages: HashMap<String, Vec<Package>>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        DependencyResolver {
            packages: HashMap::new(),
        }
    }

    /// Add available package versions
    pub fn add_package(&mut self, package: Package) {
        self.packages
            .entry(package.name.clone())
            .or_insert_with(Vec::new)
            .push(package);
    }

    /// Resolve dependencies to a consistent set
    pub fn resolve(&self, root_deps: &[Dependency]) -> Result<Vec<Package>, String> {
        let mut resolved: HashMap<String, Package> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        for dep in root_deps {
            self.resolve_recursive(dep, &mut resolved, &mut visited)?;
        }

        Ok(resolved.into_values().collect())
    }

    fn resolve_recursive(
        &self,
        dep: &Dependency,
        resolved: &mut HashMap<String, Package>,
        visited: &mut HashSet<String>,
    ) -> Result<(), String> {
        // Check for circular dependencies
        if visited.contains(&dep.name) {
            return Err(format!("Circular dependency detected: {}", dep.name));
        }

        visited.insert(dep.name.clone());

        // Find compatible version
        let package = self.find_compatible_version(&dep.name, &dep.version_req)?;

        // Check for conflicts
        if let Some(existing) = resolved.get(&dep.name) {
            if existing.version != package.version {
                return Err(format!(
                    "Version conflict for {}: {} vs {}",
                    dep.name,
                    existing.version.to_string(),
                    package.version.to_string()
                ));
            }
            return Ok(());
        }

        // Resolve dependencies of this package
        for sub_dep in &package.dependencies {
            self.resolve_recursive(sub_dep, resolved, visited)?;
        }

        resolved.insert(dep.name.clone(), package);
        visited.remove(&dep.name);

        Ok(())
    }

    fn find_compatible_version(
        &self,
        name: &str,
        req: &VersionRequirement,
    ) -> Result<Package, String> {
        let versions = self
            .packages
            .get(name)
            .ok_or_else(|| format!("Package not found: {}", name))?;

        // Find highest compatible version
        let compatible: Vec<&Package> = versions
            .iter()
            .filter(|p| req.matches(&p.version))
            .collect();

        if compatible.is_empty() {
            return Err(format!("No compatible version found for {}", name));
        }

        // Return highest version
        let best = compatible.iter().max_by_key(|p| &p.version).unwrap();

        Ok((*best).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_resolution() {
        let mut resolver = DependencyResolver::new();

        // Add available packages
        resolver.add_package(Package {
            name: "foo".to_string(),
            version: Version::new(1, 0, 0),
            authors: vec![],
            description: None,
            license: None,
            repository: None,
            dependencies: vec![],
        });

        let deps = vec![Dependency {
            name: "foo".to_string(),
            version_req: VersionRequirement::Caret(Version::new(1, 0, 0)),
        }];

        let resolved = resolver.resolve(&deps).unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].name, "foo");
    }
}
