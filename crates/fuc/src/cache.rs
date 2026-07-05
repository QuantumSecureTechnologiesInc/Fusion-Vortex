use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Checks if a package requires recompilation by hashing its source tree.
pub fn is_dirty(root: &Path, artifact: &Path) -> bool {
    if !artifact.exists() { return true; }
    let hash_file = artifact.with_extension("hash");
    if !hash_file.exists() { return true; }

    let old_hash = fs::read_to_string(hash_file).unwrap_or_default();
    let new_hash = calculate_tree_hash(root);
    old_hash != new_hash
}

fn calculate_tree_hash(root: &Path) -> String {
    let mut hasher = Sha256::new();
    let walker = WalkDir::new(root).sort_by_file_name().into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        // Skip hidden directories (like .git), target, and node_modules entirely
        !name.starts_with('.') && name != "target" && name != "node_modules"
    });
    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entry.path().extension().map_or(false, |e| e == "fu") {
            if let Ok(content) = fs::read(entry.path()) {
                hasher.update(content);
            }
        }
    }
    hex::encode(hasher.finalize())
}

pub fn update(root: &Path, artifact: &Path) -> anyhow::Result<()> {
    let hash = calculate_tree_hash(root);
    fs::write(artifact.with_extension("hash"), hash)?;
    Ok(())
}
