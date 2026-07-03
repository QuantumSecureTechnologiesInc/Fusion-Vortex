//! Fusion linker driver.
use anyhow::Result;

pub fn link_bin(objects: &[String], output: &str) -> Result<()> {
    println!("Linking {} objects into {}", objects.len(), output);
    Ok(())
}