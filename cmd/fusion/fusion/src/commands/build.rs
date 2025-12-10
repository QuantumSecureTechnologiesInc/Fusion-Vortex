use anyhow::Result;
use fusion_toolchain;

pub fn build(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    fusion_toolchain::build(release, target, verbose)
}
