use anyhow::Result;
use fusion_profiler;

pub fn profile(mode: &str, output: &str) -> Result<()> {
    fusion_profiler::profile(mode, output).map(|_| ())
}
