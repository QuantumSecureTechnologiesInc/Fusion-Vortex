// Fusion Build Driver: Orchestration Layer
use std::process::Command;
use std::path::Path;
use anyhow::{Result, bail};
use crate::{graph, fingerprint, linker};

pub fn build_project(release: bool) -> Result<()> {
    let root = std::env::current_dir()?;
    let build_graph = graph::resolve_dependencies(&root)?;
    let order = build_graph.topological_sort()?;

    let mut object_files = Vec::new();

    for pkg in order {
        let artifact = format!("target/deps/{}.o", pkg.name);
        
        // Caching Layer: Hash source files to determine if re-build is necessary
        if fingerprint::is_dirty(&pkg.path, Path::new(&artifact)) {
            let mut cmd = Command::new("fuc");
            cmd.arg(pkg.path.join("src/main.fu"));
            cmd.arg("-o").arg(&artifact);
            if release { cmd.arg("-O").arg("3"); }
            if !cmd.status()?.success() { bail!("Vortex Engine halted build for package {}", pkg.name); }
            fingerprint::save_hash(&pkg.path, Path::new(&artifact))?;
        }
        object_files.push(artifact);
    }

    // Final Link Step
    linker::link_bin(&object_files, &format!("target/{}", build_graph.root_name))?;
    Ok(())
}