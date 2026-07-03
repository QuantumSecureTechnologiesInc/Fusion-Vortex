//! Build fingerprinting and caching for incremental compilation.
use std::path::Path;
use anyhow::Result;

pub fn is_dirty(_source_dir: &Path, _artifact: &Path) -> bool {
    // Always rebuild for now (bootstrap phase)
    true
}

pub fn save_hash(_source_dir: &Path, _artifact: &Path) -> Result<()> {
    Ok(())
}