use anyhow::Result;

pub fn handle_prompt(prompt: &str, _offline: bool) -> Result<()> {
    println!("Processing: {}", prompt);
    println!("Response: This is a mock AI response.");
    Ok(())
}
