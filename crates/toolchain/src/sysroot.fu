// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::path::PathBuf;
// __FU_COMPAT_END__
use anyhow::{bail, Result};
pub struct Sysroot {
    pub root: PathBuf,
    pub std_source: PathBuf,
    pub runtime_obj: PathBuf,
}
impl Sysroot {
    pub fn discover() -> Result<Sysroot> {
        let exe_path = std::env::current_exe()?;
        let install_root = exe_path
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow::anyhow!("Could not determine install root"))?;
        let lib_root = install_root.join("lib").join("fusion");
        let std_source = lib_root.join("std");
        let runtime_obj = lib_root.join("runtime.o");
        if !std_source.exists() || !runtime_obj.exists() {
            bail!(
                "Broken toolchain installation: missing std or runtime.o at {}",
                lib_root.display()
            );
        }
        Ok(Sysroot {
            root: install_root.to_path_buf(),
            std_source,
            runtime_obj,
        })
    }
}
