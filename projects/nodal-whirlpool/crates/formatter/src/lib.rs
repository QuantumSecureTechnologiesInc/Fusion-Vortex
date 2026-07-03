use anyhow::Result;

pub struct FormatResult {
    pub needs_formatting: bool,
    pub formatted_count: usize,
}

pub fn format(_check: bool, _all: bool) -> Result<FormatResult> {
    println!("Formatting code...");
    Ok(FormatResult {
        needs_formatting: false,
        formatted_count: 0,
    })
}
