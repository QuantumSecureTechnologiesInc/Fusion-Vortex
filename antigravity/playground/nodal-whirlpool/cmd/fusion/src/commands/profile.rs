use anyhow::Result;
use fusion_profiler;

pub fn profile(mode: &str, output: &str) -> Result<()> {
    fusion_profiler::run_profile(mode, output)
}
