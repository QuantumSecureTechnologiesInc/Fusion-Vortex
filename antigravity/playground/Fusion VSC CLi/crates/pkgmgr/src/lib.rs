use anyhow::Result;

pub fn add(package: &str, version: Option<&str>) -> Result<()> {
    println!("Adding package {} (version: {:?})", package, version);
    Ok(())
}

pub fn remove(package: &str) -> Result<()> {
    println!("Removing package {}", package);
    Ok(())
}

pub fn update(all: bool) -> Result<usize> {
    println!("Updating packages (all: {})", all);
    Ok(0)
}

pub fn list() -> Result<()> {
    println!("Listing packages...");
    Ok(())
}

pub fn publish(verify: bool) -> Result<()> {
    println!("Publishing package (verify: {})", verify);
    Ok(())
}
