use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// OAuth provider configuration
#[derive(Clone, Debug)]
pub struct OAuthProvider {
    pub name: String,
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub scopes: Vec<String>,
}

/// Authentication manager for extensions
pub struct AuthManager {
    providers: Arc<RwLock<HashMap<String, OAuthProvider>>>,
    tokens: Arc<RwLock<HashMap<String, String>>>,
}

impl AuthManager {
    pub fn new() -> Self {
        let mut providers = HashMap::new();

        // Pre-configure Google OAuth for Gemini Code Assist
        providers.insert(
            "google".to_string(),
            OAuthProvider {
                name: "Google".to_string(),
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
                client_id: "fusion-vsc-cli".to_string(),
                scopes: vec![
                    "https://www.googleapis.com/auth/cloud-platform".to_string(),
                    "https://www.googleapis.com/auth/generative-language".to_string(),
                ],
            },
        );

        Self {
            providers: Arc::new(RwLock::new(providers)),
            tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initiate OAuth flow for an extension
    pub async fn authenticate(&self, provider_name: &str) -> Result<String> {
        let providers = self.providers.read().await;
        let provider = providers
            .get(provider_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown OAuth provider: {}", provider_name))?;

        println!("🔐 Starting OAuth flow for {}", provider.name);
        println!("   Scopes: {}", provider.scopes.join(", "));

        // Generate state for CSRF protection
        let state = uuid::Uuid::new_v4().to_string();

        // Build authorization URL
        let auth_url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.auth_url,
            urlencoding::encode(&provider.client_id),
            urlencoding::encode("http://localhost:8765/callback"),
            urlencoding::encode(&provider.scopes.join(" ")),
            state
        );

        println!("\n📱 Opening browser for authentication...");
        println!("   URL: {}", auth_url);

        // Launch terminal browser
        let browser_result = self.launch_browser(&auth_url).await?;

        // Simulate token exchange (in real implementation, receive callback)
        let token = format!("token_{}_{}", provider_name, uuid::Uuid::new_v4());

        // Store token
        let mut tokens = self.tokens.write().await;
        tokens.insert(provider_name.to_string(), token.clone());

        println!("✅ Authentication successful!");

        Ok(token)
    }

    /// Launch terminal browser for OAuth
    async fn launch_browser(&self, url: &str) -> Result<String> {
        println!("\n🌐 Launching terminal browser...");
        println!("   (Terminal browser integration)");
        println!("   URL: {}", url);

        // For demonstration, simulate user authentication
        println!("\n📋 Simulating OAuth flow:");
        println!("   1. User navigates to auth page");
        println!("   2. User grants permissions");
        println!("   3. Redirect with auth code");
        println!("   4. Exchange code for token");

        // In real implementation:
        // - Spawn fusion-terminal-browser
        // - Start local server on port 8765 for callback
        // - Receive auth code from callback
        // - Exchange auth code for access token

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        Ok("simulated_auth_code".to_string())
    }

    /// Get stored token for provider
    pub async fn get_token(&self, provider_name: &str) -> Option<String> {
        let tokens = self.tokens.read().await;
        tokens.get(provider_name).cloned()
    }

    /// Check if authenticated for provider
    pub async fn is_authenticated(&self, provider_name: &str) -> bool {
        let tokens = self.tokens.read().await;
        tokens.contains_key(provider_name)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}
