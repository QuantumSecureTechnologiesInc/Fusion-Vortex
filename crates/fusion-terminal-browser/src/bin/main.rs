//! Fusion Terminal Browser CLI

use clap::{Parser, Subcommand};
use fusion_terminal_browser::{Browser, BrowserConfig, Result};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "fusion-browser")]
#[command(about = "Terminal-based browser with Blink engine and WebGPU", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// URL to navigate to
    #[arg(short, long)]
    url: Option<String>,

    /// Disable WebGPU acceleration
    #[arg(long)]
    no_webgpu: bool,

    /// Disable JavaScript
    #[arg(long)]
    no_js: bool,

    /// Disable images
    #[arg(long)]
    no_images: bool,

    /// Rendering mode (ascii, unicode-block, unicode-full, truecolor, sixel, kitty)
    #[arg(short, long, default_value = "truecolor")]
    render_mode: String,

    /// Color depth (monochrome, ansi16, ansi256, truecolor)
    #[arg(short, long, default_value = "truecolor")]
    color_depth: String,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive browser session
    Browse {
        /// Initial URL to load
        url: Option<String>,
    },

    /// Capture a screenshot of a URL
    Screenshot {
        /// URL to screenshot
        url: String,

        /// Output file path
        #[arg(short, long, default_value = "screenshot.png")]
        output: String,
    },

    /// Execute JavaScript on a page
    Exec {
        /// URL to load
        url: String,

        /// JavaScript code to execute
        #[arg(short, long)]
        script: String,
    },

    /// Get HTML content of a page
    Html {
        /// URL to load
        url: String,
    },

    /// Show browser configuration
    Config {
        /// Show configuration and exit
        #[arg(short, long)]
        show: bool,

        /// Save current configuration to file
        #[arg(short, long)]
        save: Option<String>,
    },
}

fn parse_render_mode(mode: &str) -> fusion_terminal_browser::config::RenderMode {
    use fusion_terminal_browser::config::RenderMode;
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

fn parse_color_depth(depth: &str) -> fusion_terminal_browser::config::ColorDepth {
    use fusion_terminal_browser::config::ColorDepth;
    match depth.to_lowercase().as_str() {
        "monochrome" => ColorDepth::Monochrome,
        "ansi16" => ColorDepth::Ansi16,
        "ansi256" => ColorDepth::Ansi256,
        "truecolor" => ColorDepth::TrueColor,
        _ => ColorDepth::TrueColor,
    }
}

fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("fusion_terminal_browser=debug,headless_chrome=info")
    } else {
        EnvFilter::new("fusion_terminal_browser=info,headless_chrome=warn")
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose);

    // Load or create configuration
    let mut config = if let Some(ref config_path) = cli.config {
        BrowserConfig::load_from_file(std::path::Path::new(config_path))?
    } else {
        // Get terminal size
        let (width, height) = crossterm::terminal::size().unwrap_or((120, 40));

        BrowserConfig::from_terminal_size(width, height)
    };

    // Apply CLI overrides
    if cli.no_webgpu {
        config.enable_webgpu = false;
    }

    if cli.no_js {
        config.enable_javascript = false;
    }

    if cli.no_images {
        config.enable_images = false;
    }

    config.render_mode = parse_render_mode(&cli.render_mode);
    config.color_depth = parse_color_depth(&cli.color_depth);

    // Execute command
    match cli.command {
        Some(Commands::Browse { url }) => {
            let mut browser = Browser::new(config)?;

            if let Some(ref url) = url {
                browser.navigate(url)?;
            } else if let Some(ref url) = cli.url {
                browser.navigate(url)?;
            }

            browser.run()?;
        }

        Some(Commands::Screenshot { url, output }) => {
            let mut browser = Browser::new(config)?;
            browser.navigate(&url)?;

            // Wait for page to load
            std::thread::sleep(std::time::Duration::from_secs(2));

            browser.screenshot_to_file(std::path::Path::new(&output))?;
            println!("Screenshot saved to: {}", output);
        }

        Some(Commands::Exec { url, script }) => {
            let mut browser = Browser::new(config)?;
            browser.navigate(&url)?;

            // Wait for page to load
            std::thread::sleep(std::time::Duration::from_secs(2));

            let result = browser.execute_script(&script)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        Some(Commands::Html { url }) => {
            let mut browser = Browser::new(config)?;
            browser.navigate(&url)?;

            // Wait for page to load
            std::thread::sleep(std::time::Duration::from_secs(2));

            let html = browser.get_html()?;
            println!("{}", html);
        }

        Some(Commands::Config { show, save }) => {
            if show {
                let toml = toml::to_string_pretty(&config)
                    .map_err(|e| fusion_terminal_browser::BrowserError::Config(e.to_string()))?;
                println!("{}", toml);
            }

            if let Some(path) = save {
                config.save_to_file(std::path::Path::new(&path))?;
                println!("Configuration saved to: {}", path);
            }
        }

        None => {
            // Default: interactive mode
            if let Some(ref url) = cli.url {
                let mut browser = Browser::new(config)?;
                browser.navigate(url)?;
                browser.run()?;
            } else {
                eprintln!("No URL provided. Use --url <URL> or see --help for usage.");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
