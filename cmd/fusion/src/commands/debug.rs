use anyhow::Result;
use fusion_debugger;

pub fn debug(target: Option<&str>) -> Result<()> {
    fusion_debugger::start(target)
}
