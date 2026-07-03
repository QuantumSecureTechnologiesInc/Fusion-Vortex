use std::path::PathBuf;
use std::env;
use anyhow::{Result, bail};

pub struct Sysroot {
    pub std_path: PathBuf,
    pub runtime_obj: PathBuf,
}

impl Sysroot {
    /// Locates the Fusion installation relative to the executable path.
    pub fn discover() -> Result<Self> {
        let exe = env::current_exe()?;
        let root = exe.parent().and_then(|p| p.parent())
            .ok_or_else(|| anyhow::anyhow!("Failed to derive sysroot"))?;

        let std = root.join("lib/fusion/std");
        let runtime = root.join("lib/fusion/runtime.o");

        if !std.exists() || !runtime.exists() {
            bail!("Incomplete Fusion toolchain detected at {}", root.display());
        }

        Ok(Self { std_path: std, runtime_obj: runtime })
    }
}