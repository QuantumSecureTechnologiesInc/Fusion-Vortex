//! Browser configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Terminal width (cells)
    pub terminal_width: u16,

    /// Terminal height (cells)
    pub terminal_height: u16,

    /// Enable WebGPU acceleration
    pub enable_webgpu: bool,

    /// Enable JavaScript execution
    pub enable_javascript: bool,

    /// Enable WebSocket support
    pub enable_websocket: bool,

    /// Enable image rendering
    pub enable_images: bool,

    /// Image quality (1-10, higher is better but slower)
    pub image_quality: u8,

    /// User agent string
    pub user_agent: Option<String>,

    /// Cache directory
    pub cache_dir: Option<PathBuf>,

    /// Session file path
    pub session_file: Option<PathBuf>,

    /// Page load timeout (milliseconds)
    pub page_load_timeout: u64,

    /// Chrome/Chromium executable path (optional, auto-detected otherwise)
    pub chrome_path: Option<PathBuf>,

    /// Chrome launch arguments
    pub chrome_args: Vec<String>,

    /// Window width for rendering (pixels)
    pub window_width: u32,

    /// Window height for rendering (pixels)
    pub window_height: u32,

    /// Headless mode
    pub headless: bool,

    /// Enable GPU rendering in headless Chrome
    pub enable_gpu: bool,

    /// Rendering mode
    pub render_mode: RenderMode,

    /// Color depth
    pub color_depth: ColorDepth,
}

/// Rendering mode for terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderMode {
    /// ASCII-only characters
    Ascii,

    /// Unicode block characters
    UnicodeBlock,

    /// Unicode with full character set
    UnicodeFull,

    /// True color with Unicode
    TrueColor,

    /// Sixel graphics (if terminal supports)
    Sixel,

    /// Kitty graphics protocol (if terminal supports)
    Kitty,
}

/// Terminal color depth
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorDepth {
    /// No colors (monochrome)
    Monochrome,

    /// 16 colors (ANSI)
    Ansi16,

    /// 256 colors
    Ansi256,

    /// True color (24-bit RGB)
    TrueColor,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            terminal_width: 120,
            terminal_height: 40,
            enable_webgpu: true,
            enable_javascript: true,
            enable_websocket: true,
            enable_images: true,
            image_quality: 7,
            user_agent: Some(crate::USER_AGENT.to_string()),
            cache_dir: dirs::cache_dir().map(|d| d.join("fusion-browser")),
            session_file: dirs::data_dir().map(|d| d.join("fusion-browser").join("session.json")),
            page_load_timeout: 30000,
            chrome_path: None,
            chrome_args: vec![
                "--disable-gpu".to_string(),
                "--no-sandbox".to_string(),
                "--disable-dev-shm-usage".to_string(),
            ],
            window_width: 1920,
            window_height: 1080,
            headless: true,
            enable_gpu: false,
            render_mode: RenderMode::TrueColor,
            color_depth: ColorDepth::TrueColor,
        }
    }
}

impl BrowserConfig {
    /// Create a new configuration from terminal dimensions
    pub fn from_terminal_size(width: u16, height: u16) -> Self {
        let mut config = Self::default();
        config.terminal_width = width;
        config.terminal_height = height;
        config
    }

    /// Load configuration from file
    pub fn load_from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::BrowserError::Config(e.to_string()))?;
        toml::from_str(&content).map_err(|e| crate::BrowserError::Config(e.to_string()))
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let content =
            toml::to_string_pretty(self).map_err(|e| crate::BrowserError::Config(e.to_string()))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Update terminal dimensions
    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_width = width;
        self.terminal_height = height;

        // Update window dimensions proportionally
        let char_width = 8; // Approximate character width in pixels
        let char_height = 16; // Approximate character height in pixels
        self.window_width = (width as u32) * char_width;
        self.window_height = (height as u32) * char_height;
    }
}
