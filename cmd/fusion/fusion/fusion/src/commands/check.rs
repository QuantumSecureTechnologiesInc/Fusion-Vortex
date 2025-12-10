use anyhow::Result;
use fusion_analyzer;

pub fn check(all: bool) -> Result<()> {
    fusion_analyzer::check(all)
}
