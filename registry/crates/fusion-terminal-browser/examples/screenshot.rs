//! Screenshot capture example

use fusion_terminal_browser::{Browser, BrowserConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create browser configuration
    let mut config = BrowserConfig::default();
    config.window_width = 1920;
    config.window_height = 1080;

    // Create browser
    let mut browser = Browser::new(config)?;

    // URLs to screenshot
    let urls = vec![
        ("https://example.com", "example_screenshot.png"),
        ("https://rust-lang.org", "rust_screenshot.png"),
        ("https://github.com", "github_screenshot.png"),
    ];

    for (url, filename) in urls {
        println!("Capturing screenshot of {}...", url);

        // Navigate to URL
        browser.navigate(url)?;

        // Wait for page to fully load
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Save screenshot
        browser.screenshot_to_file(Path::new(filename))?;

        println!("  Saved to {}", filename);
    }

    println!("\nAll screenshots captured successfully!");

    Ok(())
}
