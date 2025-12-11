use anyhow::Result;
use fusion_analyzer;

pub fn lint(fix: bool, security: bool) -> Result<()> {
    fusion_analyzer::lint(fix, security).map(|_| ())
}
