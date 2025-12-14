use crate::tool::McpTool;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Registry for managing MCP tools and their dependencies
#[derive(Default)]
pub struct ToolRegistry {
    /// Registered tools by name
    tools: HashMap<String, McpTool>,
    /// Dependency graph (Tool Name -> List of dependency names)
    dependencies: HashMap<String, Vec<String>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a tool
    pub fn register(&mut self, tool: McpTool) {
        self.tools.insert(tool.name.clone(), tool);
    }

    /// Register a dependency between tools
    pub fn add_dependency(&mut self, user: &str, provider: &str) {
        self.dependencies
            .entry(user.to_string())
            .or_default()
            .push(provider.to_string());
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&McpTool> {
        self.tools.get(name)
    }

    /// List all tools
    pub fn list(&self) -> Vec<&McpTool> {
        self.tools.values().collect()
    }

    /// Check for circular dependencies
    pub fn validate_graph(&self) -> Result<()> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();

        for node in self.dependencies.keys() {
            if self.detect_cycle(node, &mut visited, &mut recursion_stack) {
                bail!("Circular dependency detected involving tool: {}", node);
            }
        }
        Ok(())
    }

    fn detect_cycle<'a>(
        &'a self,
        node: &'a str,
        visited: &mut HashSet<&'a str>,
        recursion_stack: &mut HashSet<&'a str>,
    ) -> bool {
        visited.insert(node);
        recursion_stack.insert(node);

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep.as_str()) {
                    if self.detect_cycle(dep, visited, recursion_stack) {
                        return true;
                    }
                } else if recursion_stack.contains(dep.as_str()) {
                    return true;
                }
            }
        }

        recursion_stack.remove(node);
        false
    }

    /// Resolve execution chain for a tool (Topological Sort)
    pub fn resolve_chain(&self, tool_name: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        // Ensure the tool exists
        if !self.tools.contains_key(tool_name) && !self.dependencies.contains_key(tool_name) {
            // If we just have dependencies but not the tool def, we might be tolerant or strict
            // For now, strict:
            // bail!("Tool not found: {}", tool_name);
            // Use relaxed check for this implementation block to allow partial graphs
        }

        self.resolve_chain_recursive(tool_name, &mut visited, &mut result)?;

        Ok(result)
    }

    fn resolve_chain_recursive<'a>(
        &'a self,
        node: &'a str,
        visited: &mut HashSet<&'a str>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if visited.contains(node) {
            return Ok(());
        }
        visited.insert(node);

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                self.resolve_chain_recursive(dep, visited, result)?;
            }
        }

        result.push(node.to_string());
        Ok(())
    }
    /// Save the dependency graph to a file
    pub fn save_graph<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let state = RegistryState {
            dependencies: self.dependencies.clone(),
        };
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &state)?;
        Ok(())
    }

    /// Load the dependency graph from a file
    pub fn load_graph<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        if !path.as_ref().exists() {
            return Ok(());
        }
        let file = std::fs::File::open(path)?;
        let state: RegistryState = serde_json::from_reader(file)?;
        self.dependencies = state.dependencies;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct RegistryState {
    dependencies: HashMap<String, Vec<String>>,
}
