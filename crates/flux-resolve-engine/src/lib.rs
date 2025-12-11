use anyhow::{anyhow, Result};
use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;

/// Resolve dependencies from a TOML manifest.
///
/// The manifest should contain a table `packages` where each key is a package name
/// and the value is a table with an optional `deps` array of strings.
/// Example:
/// ```toml
/// [packages]
/// pkg_a = { deps = ["pkg_b", "pkg_c"] }
/// pkg_b = { deps = [] }
/// pkg_c = { deps = ["pkg_b"] }
/// ```
/// The function returns a topologically sorted list of package names.
pub fn resolve(manifest: &str) -> Result<Vec<String>> {
    // Parse the TOML string into a generic map.
    let value: toml::Value =
        toml::from_str(manifest).map_err(|e| anyhow!("Failed to parse manifest TOML: {}", e))?;
    let packages = value
        .get("packages")
        .ok_or_else(|| anyhow!("Manifest missing [packages] table"))?
        .as_table()
        .ok_or_else(|| anyhow!("[packages] should be a table"))?;

    // Build a directed graph where edges go from a package to its dependencies.
    let mut graph = DiGraphMap::<&str, ()>::new();
    for (pkg, data) in packages.iter() {
        graph.add_node(pkg.as_str());
        let deps = data
            .get("deps")
            .and_then(|v| v.as_array())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        for dep in deps {
            let dep_str = dep
                .as_str()
                .ok_or_else(|| anyhow!("Dependency should be a string"))?;
            graph.add_node(dep_str);
            // Edge from pkg -> dep (pkg depends on dep)
            graph.add_edge(pkg.as_str(), dep_str, ());
        }
    }

    // Perform topological sort. petgraph returns nodes in reverse order (dependencies first).
    let sorted = toposort(&graph, None).map_err(|cycle| {
        let node = cycle.node_id();
        anyhow!("Dependency cycle detected involving package '{}'", node)
    })?;

    // Convert to owned Strings in the order returned (dependencies first).
    let result: Vec<String> = sorted.into_iter().map(|s| s.to_string()).collect();
    Ok(result)
}
