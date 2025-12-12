use anyhow::Result;

pub struct LintResult {
    pub has_errors: bool,
    pub error_count: usize,
    pub warning_count: usize,
}

pub fn check(_all: bool) -> Result<()> {
    println!("Checking code...");
    Ok(())
}

pub fn lint(_fix: bool, _security: bool) -> Result<LintResult> {
    println!("Linting code...");
    Ok(LintResult {
        has_errors: false,
        error_count: 0,
        warning_count: 0,
    })
}
