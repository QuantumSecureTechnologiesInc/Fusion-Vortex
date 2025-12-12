use anyhow::Result;
use std::path::Path;

pub mod builder;
pub mod project;
pub mod runner;

/// Create a new Fusion project
pub fn new_project(name: &str, template: &str, path: &Path) -> Result<()> {
    project::create(name, template, path)
}

/// Build a Fusion project
pub fn build(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    builder::build(release, target, verbose)
}

/// Run a Fusion project
pub fn run(release: bool, args: &[String]) -> Result<()> {
    runner::run(release, args)
}
