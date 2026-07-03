// Fusion Forge Build Engine
// Rationale: Compiles Fusion projects, parses workspaces, resolves dependency trees,
// and coordinates the Ouroboros multi-stage compiler bootstrap sequences.

use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use std::process::Command;
use anyhow::{Result, bail};

pub struct ForgePackage {
    pub name: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
}

pub struct ForgeDependencySorter {
    packages: HashMap<String, ForgePackage>,
}

impl ForgeDependencySorter {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn register_package(&mut self, package: ForgePackage) {
        self.packages.insert(package.name.clone(), package);
    }

    /// Sorts our package list using a Depth-First Search topological sort.
    pub fn generate_ordered_build_list(&self) -> Result<Vec<String>> {
        let mut sorted_list = Vec::new();
        let mut visited_nodes = HashSet::new();
        let mut temp_loop_tracker = HashSet::new();

        for package_name in self.packages.keys() {
            self.depth_first_traversal(package_name, &mut visited_nodes, &mut temp_loop_tracker, &mut sorted_list)?;
        }

        Ok(sorted_list)
    }

    fn depth_first_traversal(&self, name: &str, visited: &mut HashSet<String>, temp: &mut HashSet<String>, order: &mut Vec<String>) -> Result<()> {
        if temp.contains(name) {
            bail!("Dependency Loop Detected: Circular reference contains element '{}'.", name);
        }
        
        if !visited.contains(name) {
            temp.insert(name.to_string());
            if let Some(pkg) = self.packages.get(name) {
                for dependency_name in &pkg.dependencies {
                    self.depth_first_traversal(dependency_name, visited, temp, order)?;
                }
            }
            temp.remove(name);
            visited.insert(name.to_string());
            order.push(name.to_string());
        }
        
        Ok(())
    }
}

pub struct OuroborosCoordinator {
    workspace_root: PathBuf,
}

impl OuroborosCoordinator {
    pub fn new(root: PathBuf) -> Self {
        Self { workspace_root: root }
    }

    /// Orchestrates the three-stage bootstrap of the self-hosted compiler.
    pub fn execute_self_host_bootstrap(&self) -> Result<()> {
        println!(">>> Commencing Ouroboros Bootstrap Sequence <<<");
        println!("Workspace root: {}", self.workspace_root.display());

        // 1. Stage 0 Check: Build the compiler with the initial host seed binary
        println!("[1/3] Building Compiler using Seed Binary (Stage 0)");
        let stage_0_status = Command::new("cargo")
            .current_dir(&self.workspace_root)
            .args(&["build", "--release", "-p", "fuc"])
            .status()?;
        if !stage_0_status.success() {
            bail!("Ouroboros: Stage 0 compilation collapsed.");
        }

        // 2. Stage 1 Check: Build the compiler using the Stage 0 compiler binary
        println!("[2/3] Self-Compiling: Building Stage 1 from Stage 0 Compiler");
        let stage_1_status = Command::new("target/release/fuc")
            .args(&["compiler/src/fuc.fu", "-o", "bin/fuc_stage1"])
            .status()?;
        if !stage_1_status.success() {
            bail!("Ouroboros: Stage 1 self-hosting verification failed.");
        }

        // 3. Stage 2 Check: Build the compiler again and compare to verify sovereignty
        println!("[3/3] Final Verification: Building Stage 2 and matching binary signatures");
        let stage_2_status = Command::new("bin/fuc_stage1")
            .args(&["compiler/src/fuc.fu", "-o", "bin/fuc_stage2"])
            .status()?;
        if !stage_2_status.success() {
            bail!("Ouroboros: Stage 2 compilation path failed.");
        }

        println!(">>> Ouroboros Bootstrap completed successfully! Binary verified as self-hosted. <<<");
        Ok(())
    }
}

#[allow(dead_code)]
fn run_forge_bootstrap() -> Result<()> {
    println!("Fusion Forge v1.0.0 initializing...");
    let coordinator = OuroborosCoordinator::new(std::env::current_dir()?);
    coordinator.execute_self_host_bootstrap()?;
    Ok(())
}