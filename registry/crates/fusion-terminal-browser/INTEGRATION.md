# Fusion Terminal Browser - CLI Integration Example

This example shows how to integrate the terminal browser into the Fusion CLI.

## Adding to Fusion CLI

### Step 1: Update cmd/fusion/Cargo.toml

Add the dependency:

```toml
[dependencies]
fusion-terminal-browser = { path = "../../crates/fusion-terminal-browser" }
```text

### Step 2: Add Browser Commands

In `cmd/fusion/src/main.rs` or appropriate command module:

```rust
use clap::{Parser, Subcommand};
use fusion_terminal_browser::{Browser, BrowserConfig};
use fusion_terminal_browser::config::{RenderMode, ColorDepth};

#[derive(Subcommand)]

pub enum Commands {
    // ... existing commands ...

    /// Terminal browser with Blink engine and WebGPU
    #[command(subcommand)]
    Browser(BrowserCommands),
}

#[derive(Subcommand)]

pub enum BrowserCommands {
    /// Browse interactively in the terminal
    Browse {
        /// URL to navigate to
        url: Option<String>,

        /// Disable WebGPU acceleration
        #[arg(long)]
        no_webgpu: bool,

        /// Rendering mode
        #[arg(long, default_value = "truecolor")]
        render_mode: String,
    },

    /// Capture a screenshot
    Screenshot {
        /// URL to screenshot
        url: String,

        /// Output file path
        #[arg(short, long, default_value = "screenshot.png")]
        output: String,

        /// Window width
        #[arg(long, default_value = "1920")]
        width: u32,

        /// Window height
        #[arg(long, default_value = "1080")]
        height: u32,
    },

    /// Execute JavaScript on a page
    Exec {
        /// URL to load
        url: String,

        /// JavaScript code to execute
        script: String,

        /// Output format (json, pretty, value)
        #[arg(short, long, default_value = "pretty")]
        format: String,
    },

    /// Get HTML content
    Html {
        /// URL to load
        url: String,

        /// Save to file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Automate web interaction
    Automate {
        /// URL to start from
        url: String,

        /// Automation script file (YAML)
        #[arg(short, long)]
        script: String,
    },
}
```text

### Step 3: Implement Command Handler

```rust
pub fn handle_browser_command(cmd: BrowserCommands) -> anyhow::Result<()> {
    match cmd {
        BrowserCommands::Browse { url, no_webgpu, render_mode } => {
            browse_interactive(url, !no_webgpu, &render_mode)?;
        }
        BrowserCommands::Screenshot { url, output, width, height } => {
            capture_screenshot(&url, &output, width, height)?;
        }
        BrowserCommands::Exec { url, script, format } => {
            execute_script(&url, &script, &format)?;
        }
        BrowserCommands::Html { url, output } => {
            get_html(&url, output.as_deref())?;
        }
        BrowserCommands::Automate { url, script } => {
            run_automation(&url, &script)?;
        }
    }

    Ok(())
}

fn browse_interactive(url: Option<String>, enable_webgpu: bool, render_mode: &str) -> anyhow::Result<()> {
    let (width, height) = crossterm::terminal::size()?;

    let mut config = BrowserConfig::from_terminal_size(width, height);
    config.enable_webgpu = enable_webgpu;
    config.render_mode = parse_render_mode(render_mode);

    let mut browser = Browser::new(config)?;

    if let Some(url) = url {
        browser.navigate(&url)?;
    }

    browser.run()?;
    Ok(())
}

fn capture_screenshot(url: &str, output: &str, width: u32, height: u32) -> anyhow::Result<()> {
    let mut config = BrowserConfig::default();
    config.window_width = width;
    config.window_height = height;

    let mut browser = Browser::new(config)?;
    browser.navigate(url)?;

    // Wait for page load
    std::thread::sleep(std::time::Duration::from_secs(2));

    browser.screenshot_to_file(std::path::Path::new(output))?;
    println!("Screenshot saved to: {}", output);

    Ok(())
}

fn execute_script(url: &str, script: &str, format: &str) -> anyhow::Result<()> {
    let mut browser = Browser::new(BrowserConfig::default())?;
    browser.navigate(url)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    let result = browser.execute_script(script)?;

    match format {
        "json" => println!("{}", serde_json::to_string(&result)?),
        "pretty" => println!("{}", serde_json::to_string_pretty(&result)?),
        "value" => {
            if let Some(s) = result.as_str() {
                println!("{}", s);
            } else {
                println!("{}", result);
            }
        }
        _ => println!("{}", result),
    }

    Ok(())
}

fn get_html(url: &str, output: Option<&str>) -> anyhow::Result<()> {
    let mut browser = Browser::new(BrowserConfig::default())?;
    browser.navigate(url)?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    let html = browser.get_html()?;

    if let Some(path) = output {
        std::fs::write(path, html)?;
        println!("HTML saved to: {}", path);
    } else {
        println!("{}", html);
    }

    Ok(())
}

fn run_automation(url: &str, script_path: &str) -> anyhow::Result<()> {
    // This is a placeholder for automation functionality
    // In a real implementation, you would parse a YAML/JSON automation script

    let mut browser = Browser::new(BrowserConfig::default())?;
    browser.navigate(url)?;

    println!("Automation not yet implemented");
    println!("Script: {}", script_path);

    Ok(())
}

fn parse_render_mode(mode: &str) -> RenderMode {
    match mode.to_lowercase().as_str() {
        "ascii" => RenderMode::Ascii,
        "unicode-block" => RenderMode::UnicodeBlock,
        "unicode-full" => RenderMode::UnicodeFull,
        "truecolor" => RenderMode::TrueColor,
        "sixel" => RenderMode::Sixel,
        "kitty" => RenderMode::Kitty,
        _ => RenderMode::TrueColor,
    }
}
```text

## Usage Examples

Once integrated, you can use the browser through the Fusion CLI:

```bash

# Interactive browsing

fusion browser browse --url https://example.com

# Capture screenshot

fusion browser screenshot https://github.com --output github.png

# Execute JavaScript

fusion browser exec https://example.com "document.title"

# Get HTML

fusion browser html https://example.com > page.html

# With options

fusion browser browse --url https://example.com --no-webgpu --render-mode ascii
```text

## Advanced Integration: Automation Scripts

You could implement automation using YAML scripts:

```yaml

# automation.yml

name: "Login and Navigate"
steps:
  - navigate: "https://example.com/login"
  - type:
      selector: "#username"
      text: "myuser"
  - type:
      selector: "#password"
      text: "mypass"
  - click: "#login-button"
  - wait: 2000
  - screenshot: "after-login.png"
  - navigate: "https://example.com/dashboard"
  - execute:
      script: "document.querySelector('.user-info').textContent"
      save_to: "user_info.txt"
```text

Then run:

```bash
fusion browser automate https://example.com --script automation.yml
```text

## Benefits of Integration

1. **Unified Interface**: Single CLI for all Fusion tools
2. **Consistent Configuration**: Share configuration across tools
3. **Composability**: Combine with other Fusion features
4. **Automation**: Integrate web tasks into Fusion workflows
5. **Documentation**: Part of Fusion's comprehensive docs

## See Also

- [User Guide](../docs/guides/terminal_browser_user_guide.md)
- [Developer Guide](../docs/guides/terminal_browser_developer_guide.md)
- [Technical Sheet](../docs/guides/terminal_browser_technical_sheet.md)