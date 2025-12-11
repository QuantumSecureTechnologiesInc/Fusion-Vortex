use anyhow::Result;
use std::path::PathBuf;

pub struct ProfileResult {
    pub output_path: PathBuf,
}

pub fn profile(mode: &str, output: &str) -> Result<ProfileResult> {
    println!("Profiling (mode: {}, output: {})", mode, output);
    Ok(ProfileResult {
        output_path: PathBuf::from(format!("target/profile.{}", output)),
    })
}
