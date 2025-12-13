//! Session management for browser state persistence

use crate::{BrowserConfig, BrowserError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Browser session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Current URL
    pub current_url: Option<String>,

    /// Navigation history
    pub history: Vec<String>,

    /// Current position in history
    pub history_index: usize,

    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,

    /// Cookies
    pub cookies: Vec<Cookie>,

    /// Last access time
    pub last_access: chrono::DateTime<chrono::Utc>,
}

/// Bookmark entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

/// Cookie entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
    pub secure: bool,
    pub http_only: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            current_url: None,
            history: Vec::new(),
            history_index: 0,
            bookmarks: Vec::new(),
            cookies: Vec::new(),
            last_access: chrono::Utc::now(),
        }
    }
}

impl Session {
    /// Create a new session
    pub fn new() -> Self {
        Self::default()
    }

    /// Load session from file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            info!("Session file does not exist, creating new session");
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| BrowserError::Session(format!("Failed to read session file: {}", e)))?;

        let session: Session = serde_json::from_str(&content)
            .map_err(|e| BrowserError::Session(format!("Failed to parse session: {}", e)))?;

        info!("Session loaded from {:?}", path);
        Ok(session)
    }

    /// Save session to file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                BrowserError::Session(format!("Failed to create session directory: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BrowserError::Session(format!("Failed to serialise session: {}", e)))?;

        fs::write(path, content)
            .map_err(|e| BrowserError::Session(format!("Failed to write session file: {}", e)))?;

        debug!("Session saved to {:?}", path);
        Ok(())
    }

    /// Add URL to history
    pub fn add_to_history(&mut self, url: String) {
        // Remove any forward history when navigating to a new page
        self.history.truncate(self.history_index + 1);

        self.history.push(url.clone());
        self.history_index = self.history.len() - 1;
        self.current_url = Some(url);
        self.last_access = chrono::Utc::now();
    }

    /// Navigate back in history
    pub fn go_back(&mut self) -> Option<String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_url = self.history.get(self.history_index).cloned();
            self.last_access = chrono::Utc::now();
            self.current_url.clone()
        } else {
            None
        }
    }

    /// Navigate forward in history
    pub fn go_forward(&mut self) -> Option<String> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_url = self.history.get(self.history_index).cloned();
            self.last_access = chrono::Utc::now();
            self.current_url.clone()
        } else {
            None
        }
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, title: String, url: String) {
        let bookmark = Bookmark {
            title,
            url,
            created: chrono::Utc::now(),
        };
        self.bookmarks.push(bookmark);
    }

    /// Remove bookmark by URL
    pub fn remove_bookmark(&mut self, url: &str) {
        self.bookmarks.retain(|b| b.url != url);
    }

    /// Add or update cookie
    pub fn set_cookie(&mut self, cookie: Cookie) {
        // Remove existing cookie with same name/domain/path
        self.cookies.retain(|c| {
            !(c.name == cookie.name && c.domain == cookie.domain && c.path == cookie.path)
        });

        self.cookies.push(cookie);
    }

    /// Get cookies for a domain
    pub fn get_cookies(&self, domain: &str) -> Vec<&Cookie> {
        self.cookies
            .iter()
            .filter(|c| domain.ends_with(&c.domain))
            .collect()
    }

    /// Clear expired cookies
    pub fn clear_expired_cookies(&mut self) {
        let now = chrono::Utc::now();
        self.cookies
            .retain(|c| c.expires.map(|exp| exp > now).unwrap_or(true));
    }

    /// Clear all session data
    pub fn clear(&mut self) {
        self.current_url = None;
        self.history.clear();
        self.history_index = 0;
        self.cookies.clear();
    }
}

/// Session manager
pub struct SessionManager {
    session: Session,
    session_file: Option<std::path::PathBuf>,
    auto_save: bool,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(config: &BrowserConfig) -> Result<Self> {
        let session_file = config.session_file.clone();

        let session = if let Some(ref path) = session_file {
            Session::load_from_file(path)?
        } else {
            Session::new()
        };

        Ok(Self {
            session,
            session_file,
            auto_save: true,
        })
    }

    /// Get session reference
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Get mutable session reference
    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    /// Save session if auto-save is enabled
    pub fn auto_save(&self) -> Result<()> {
        if self.auto_save {
            if let Some(ref path) = self.session_file {
                self.session.save_to_file(path)?;
            }
        }
        Ok(())
    }

    /// Enable or disable auto-save
    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
    }
}

impl Drop for SessionManager {
    fn drop(&mut self) {
        let _ = self.auto_save();
    }
}
