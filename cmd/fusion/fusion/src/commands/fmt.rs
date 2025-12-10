use anyhow::Result;
use fusion_formatter;

pub fn fmt(check: bool, all: bool) -> Result<()> {
    fusion_formatter::format(check, all)?;
    Ok(())
}
