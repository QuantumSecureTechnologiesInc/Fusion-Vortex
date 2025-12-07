// src/module_resolver/mod.rs - Module Resolution and Dependency Graph

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::Declaration;
use crate::parser::Parser;

/// Represents a resolved module with its dependencies
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub is_public: bool,
    pub dependencies: Vec<String>, // Module names this module depends on
}

/// Module resolver - finds and builds dependency graph
#[allow(dead_code)]
pub struct ModuleResolver {
    modules: HashMap<String, Module>,
    entry_point: PathBuf,
}

impl ModuleResolver {
    #[allow(dead_code)]
    pub fn new(entry_point: PathBuf) -> Self {
        ModuleResolver {
            modules: HashMap::new(),
            entry_point,
        }
    }

    /// Resolve all modules starting from the entry point
    #[allow(dead_code)]
    pub fn resolve(&mut self) -> Result<Vec<String>, String> {
        // Get base directory for module resolution
        let base_dir = self
            .entry_point
            .parent()
            .ok_or("Invalid entry point path")?
            .to_path_buf();

        // Start resolution from entry point
        let entry_name = self
            .entry_point
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid entry point filename")?
            .to_string();

        self.resolve_module(&entry_name, &base_dir, false)?;

        // Build compilation order via topological sort
        self.topological_sort()
    }

    /// Recursively resolve a single module and its dependencies
    #[allow(dead_code)]
    fn resolve_module(
        &mut self,
        module_name: &str,
        base_dir: &Path,
        is_public: bool,
    ) -> Result<(), String> {
        // Check if already resolved
        if self.modules.contains_key(module_name) {
            return Ok(());
        }

        // Find the module file
        let module_path = self.find_module_file(module_name, base_dir)?;

        // Read and parse the module
        let source = fs::read_to_string(&module_path)
            .map_err(|e| format!("Failed to read module '{}': {}", module_name, e))?;

        let mut parser = Parser::new(&source);
        let declarations = parser
            .parse_program()
            .map_err(|e| format!("Failed to parse module '{}': {}", module_name, e))?;

        // Extract module declarations and use statements
        let mut dependencies = Vec::new();

        for decl in &declarations {
            match decl {
                Declaration::ModuleDecl {
                    name: child_name,
                    is_public: child_public,
                } => {
                    // Recursively resolve child module
                    dependencies.push(child_name.clone());
                    self.resolve_module(child_name, base_dir, *child_public)?;
                }
                Declaration::UseDecl { path, .. } => {
                    // Add first component of use path as dependency
                    if let Some(first) = path.first() {
                        if !dependencies.contains(first) {
                            dependencies.push(first.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        // Store resolved module
        self.modules.insert(
            module_name.to_string(),
            Module {
                name: module_name.to_string(),
                path: module_path,
                is_public,
                dependencies,
            },
        );

        Ok(())
    }

    /// Find the .fu file for a given module name
    #[allow(dead_code)]
    fn find_module_file(&self, module_name: &str, base_dir: &Path) -> Result<PathBuf, String> {
        // Try direct file: module_name.fu
        let direct_path = base_dir.join(format!("{}.fu", module_name));
        if direct_path.exists() {
            return Ok(direct_path);
        }

        // Try directory with mod.fu: module_name/mod.fu
        let dir_path = base_dir.join(module_name).join("mod.fu");
        if dir_path.exists() {
            return Ok(dir_path);
        }

        Err(format!(
            "Module '{}' not found. Tried:\n  {}\n  {}",
            module_name,
            direct_path.display(),
            dir_path.display()
        ))
    }

    /// Perform topological sort to determine compilation order
    #[allow(dead_code)]
    fn topological_sort(&self) -> Result<Vec<String>, String> {
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();
        let mut order = Vec::new();

        for module_name in self.modules.keys() {
            if !visited.contains(module_name) {
                self.visit(module_name, &mut visited, &mut temp_mark, &mut order)?;
            }
        }

        order.reverse(); // Reverse to get correct dependency order
        Ok(order)
    }

    /// DFS visit for topological sort
    #[allow(dead_code)]
    fn visit(
        &self,
        module_name: &str,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if temp_mark.contains(module_name) {
            // Circular dependency detected
            return Err(format!(
                "Circular dependency detected involving module '{}'",
                module_name
            ));
        }

        if visited.contains(module_name) {
            return Ok(());
        }

        temp_mark.insert(module_name.to_string());

        if let Some(module) = self.modules.get(module_name) {
            for dep in &module.dependencies {
                if self.modules.contains_key(dep) {
                    self.visit(dep, visited, temp_mark, order)?;
                }
                // Ignore dependencies that aren't in our module set (like stdlib)
            }
        }

        temp_mark.remove(module_name);
        visited.insert(module_name.to_string());
        order.push(module_name.to_string());

        Ok(())
    }

    /// Get the module for a given name
    #[allow(dead_code)]
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    /// Get all resolved modules
    #[allow(dead_code)]
    pub fn all_modules(&self) -> &HashMap<String, Module> {
        &self.modules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_module_resolution() {
        // Create temporary directory
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create main.fu
        let main_path = base_path.join("main.fu");
        let mut main_file = fs::File::create(&main_path).unwrap();
        writeln!(main_file, "pub mod utils;").unwrap();
        writeln!(main_file, "use utils::helper;").unwrap();
        writeln!(main_file, "fn main() -> int {{ return 0; }}").unwrap();

        // Create utils.fu
        let utils_path = base_path.join("utils.fu");
        let mut utils_file = fs::File::create(&utils_path).unwrap();
        writeln!(utils_file, "pub fn helper() -> int {{ return 42; }}").unwrap();

        // Resolve modules
        let mut resolver = ModuleResolver::new(main_path);
        let order = resolver.resolve().unwrap();

        // Verify resolution
        assert!(order.contains(&"main".to_string()));
        assert!(order.contains(&"utils".to_string()));
        assert_eq!(order.len(), 2);

        // utils should come before main in compilation order
        let utils_idx = order.iter().position(|x| x == "utils").unwrap();
        let main_idx = order.iter().position(|x| x == "main").unwrap();
        assert!(utils_idx < main_idx);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create a.fu with dependency on b
        let a_path = base_path.join("a.fu");
        let mut a_file = fs::File::create(&a_path).unwrap();
        writeln!(a_file, "pub mod b;").unwrap();

        // Create b.fu with dependency on a (circular!)
        let b_path = base_path.join("b.fu");
        let mut b_file = fs::File::create(&b_path).unwrap();
        writeln!(b_file, "pub mod a;").unwrap();

        // Try to resolve - should fail
        let mut resolver = ModuleResolver::new(a_path);
        let result = resolver.resolve();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular dependency detected"));
    }
}
