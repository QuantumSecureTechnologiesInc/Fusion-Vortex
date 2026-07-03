//! Blink/Chromium browser engine wrapper using chromiumoxide
//!
//! This uses chromiumoxide which provides superior Chromium DevTools Protocol (CDP)
//! support compared to headless_chrome, with better WebGPU capabilities.

use crate::{BrowserConfig, Result};
use chromiumoxide::browser::{Browser as ChromeBrowser, BrowserConfig as ChromeConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::js::Evaluation;
use chromiumoxide::page::Page;
use fusion_runtime_core::Runtime;
use futures::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info};

/// Browser engine wrapper around Chromium via chromiumoxide
pub struct Engine {
    browser: Arc<ChromeBrowser>,
    runtime: Arc<Runtime>,
    current_page: Option<Arc<Page>>,
    _config: BrowserConfig,
}

impl Engine {
    /// Create a new browser engine instance
    pub fn new(config: BrowserConfig, runtime: Arc<Runtime>) -> Result<Self> {
        info!("Initialising Blink browser engine with chromiumoxide");

        // Build chromiumoxide config
        let mut chrome_args = vec![
            "--headless=new".to_string(),
            "--no-sandbox".to_string(),
            "--enable-unsafe-webgpu".to_string(),
            "--disable-gpu-sandbox".to_string(),
            "--disable-infobars".to_string(),
            "--disable-blink-features=AutomationControlled".to_string(),
            format!(
                "--window-size={},{}",
                config.window_width, config.window_height
            ),
        ];

        // Add user data dir for persistent sessions
        if let Some(ref cache_dir) = config.cache_dir {
            std::fs::create_dir_all(cache_dir)?;
            chrome_args.push(format!("--user-data-dir={}", cache_dir.to_string_lossy()));
        }

        // Platform-specific args
        #[cfg(target_os = "linux")]
        chrome_args.push("--use-angle=vulkan".to_string());

        // Merge with config args
        chrome_args.extend(config.chrome_args.clone());

        let mut chrome_config = ChromeConfig::builder()
            .args(chrome_args)
            .window_size(config.window_width, config.window_height);

        // Set chrome path if specified
        if let Some(ref path) = config.chrome_path {
            chrome_config = chrome_config.chrome_executable(path.clone());
        } else if let Some(auto_path) = find_chrome_executable() {
            chrome_config = chrome_config.chrome_executable(auto_path);
        }

        let chrome_config = chrome_config
            .build()
            .map_err(|e| crate::BrowserError::Engine(e.to_string()))?;

        // Launch browser using fusion runtime
        let (browser, mut handler) = runtime
            .block_on(async { ChromeBrowser::launch(chrome_config).await })
            .map_err(|e| crate::BrowserError::Engine(format!("Failed to launch browser: {}", e)))?;

        let browser = Arc::new(browser);

        // Spawn handler task using Fusion runtime
        runtime.spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        info!("Browser engine initialised successfully");

        Ok(Self {
            browser,
            runtime,
            current_page: None,
            _config: config,
        })
    }

    /// Navigate to a URL
    pub fn navigate(&mut self, url: &str) -> Result<()> {
        debug!("Navigating to: {}", url);

        let browser = self.browser.clone();
        let url = url.to_string();
        let runtime = self.runtime.clone();

        // Create page and wait for navigation in single async block
        let page = runtime.block_on(async move {
            let page = browser.new_page(&url).await?;
            // wait_for_navigation returns &Page, just await it
            let _ = page.wait_for_navigation().await;
            Ok::<Page, chromiumoxide::error::CdpError>(page)
        })?;

        self.current_page = Some(Arc::new(page));

        debug!("Page loaded successfully");
        Ok(())
    }

    /// Get the current page screenshot
    pub fn capture_screenshot(&self) -> Result<Vec<u8>> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        debug!("Capturing screenshot");

        let page = page.clone();
        let screenshot = self.runtime.block_on(async move {
            page.screenshot(
                chromiumoxide::page::ScreenshotParams::builder()
                    .format(CaptureScreenshotFormat::Png)
                    .build(),
            )
            .await
        })?;

        Ok(screenshot)
    }

    /// Execute JavaScript in the current page
    pub fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        debug!("Executing JavaScript");

        let page = page.clone();
        let script = script.to_string();

        let result = self
            .runtime
            .block_on(async move { page.evaluate(script).await })?;

        Ok(result.into_value().unwrap_or(serde_json::Value::Null))
    }

    /// Get the current page title
    pub fn get_title(&self) -> Result<String> {
        let script = "document.title";
        let result = self.execute_script(script)?;

        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Get the current page URL
    pub fn get_url(&self) -> Result<String> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        let page = page.clone();
        let url = self.runtime.block_on(async move { page.url().await })?;

        Ok(url.unwrap_or_default())
    }

    /// Get page HTML content
    pub fn get_content(&self) -> Result<String> {
        let script = "document.documentElement.outerHTML";
        let result = self.execute_script(script)?;

        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Click at specific coordinates using JavaScript (CDP mouse events are complex in chromiumoxide)
    pub fn click_at(&self, x: f64, y: f64) -> Result<()> {
        let script = format!("document.elementFromPoint({}, {})?.click();", x, y);
        self.execute_script(&script)?;
        Ok(())
    }

    /// Click an element by selector
    pub fn click(&self, selector: &str) -> Result<()> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        let page = page.clone();
        let selector = selector.to_string();

        self.runtime.block_on(async move {
            if let Ok(element) = page.find_element(&selector).await {
                element.click().await?;
            }
            Ok::<(), chromiumoxide::error::CdpError>(())
        })?;

        Ok(())
    }

    /// Type text into an element
    pub fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        let page = page.clone();
        let selector = selector.to_string();
        let text = text.to_string();

        self.runtime.block_on(async move {
            if let Ok(element) = page.find_element(&selector).await {
                element.click().await?;
                element.type_str(&text).await?;
            }
            Ok::<(), chromiumoxide::error::CdpError>(())
        })?;

        Ok(())
    }

    /// Scroll the page
    pub fn scroll(&self, delta_y: i32) -> Result<()> {
        let script = format!("window.scrollBy(0, {});", delta_y);
        self.execute_script(&script)?;
        Ok(())
    }

    /// Go back in history
    pub fn go_back(&self) -> Result<()> {
        let script = "window.history.back()";
        self.execute_script(script)?;
        Ok(())
    }

    /// Go forward in history
    pub fn go_forward(&self) -> Result<()> {
        let script = "window.history.forward()";
        self.execute_script(script)?;
        Ok(())
    }

    /// Reload the current page
    pub fn reload(&self) -> Result<()> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        let page = page.clone();

        self.runtime.block_on(async move {
            page.reload().await?;
            Ok::<(), chromiumoxide::error::CdpError>(())
        })?;

        Ok(())
    }

    /// Wait for an element to appear (simplified version)
    pub fn wait_for_element(&self, selector: &str, _timeout_ms: u64) -> Result<bool> {
        let page = self
            .current_page
            .as_ref()
            .ok_or_else(|| crate::BrowserError::Engine("No active page".to_string()))?;

        let page = page.clone();
        let selector = selector.to_string();

        let result = self.runtime.block_on(async move {
            match page.find_element(&selector).await {
                Ok(_) => Ok::<bool, chromiumoxide::error::CdpError>(true),
                Err(_) => Ok::<bool, chromiumoxide::error::CdpError>(false),
            }
        })?;

        Ok(result)
    }

    /// Set viewport size
    pub fn set_viewport_size(&self, _width: u32, _height: u32) -> Result<()> {
        // Viewport is set at browser launch
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        debug!("Shutting down browser engine");
    }
}

/// Find Chrome/Chromium executable
fn find_chrome_executable() -> Option<PathBuf> {
    // Check environment variable
    if let Ok(p) = std::env::var("CHROME_PATH") {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }

    // Check local bin directory
    let bin = if cfg!(target_os = "macos") {
        Path::new("./bin/chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing")
    } else if cfg!(target_os = "windows") {
        Path::new("./bin/chrome-win64/chrome.exe")
    } else {
        Path::new("./bin/chrome-linux64/chrome")
    };

    if bin.exists() {
        std::fs::canonicalize(bin).ok()
    } else {
        None
    }
}
