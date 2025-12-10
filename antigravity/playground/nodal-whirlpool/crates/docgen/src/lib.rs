use anyhow::Result;
use std::path::PathBuf;

pub fn generate(_private: bool) -> Result<PathBuf> {
    println!("Generating documentation...");
    Ok(PathBuf::from("target/doc"))
}

pub fn open(_path: &PathBuf) -> Result<()> {
    println!("Opening documentation in browser...");
    Ok(())
}
