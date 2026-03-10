// __FU_COMPAT_START__
#![allow(missing_docs)]
#[allow(missing_docs, dead_code)] type FBool = bool;
// __FU_COMPAT_END__
use anyhow::Result;
fn build_local_response(prompt: &str) -> String {
    let words = prompt
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .count();
    let chars = prompt.chars().count();
    format!(
        "Local response (no remote model configured):\n- words: {words}\n- characters: {chars}\n- prompt: {prompt}"
    )
}

pub fn handle_prompt(prompt: &str, offline: FBool) -> Result<()> {
    println!("Processing: {}", prompt);
    match crate::get_adapter(offline) {
        Ok(adapter) => {
            let rt = tokio::runtime::Runtime::new()?;
            let (response, _) = rt.block_on(adapter.predict(prompt))?;
            println!("Response: {}", response);
        }
        Err(err) => {
            eprintln!("AI adapter unavailable: {err}");
            println!("Response: {}", build_local_response(prompt));
        }
    }
    Ok(())
}
