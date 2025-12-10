use anyhow::Result;
use fusion_audit;

pub fn audit(report: bool, _deny: bool) -> Result<()> {
    fusion_audit::audit(report)?;
    Ok(())
}
