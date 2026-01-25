// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::time::Duration;
#[allow(missing_docs, dead_code)] type FBool = bool;
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FSize = usize;
#[allow(missing_docs, dead_code)] type FVec<T> = Vec<T>;
// __FU_COMPAT_END__
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub base_url: FString,
    pub model: FString,
    pub timeout: Duration,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: "llama3".to_string(),
            timeout: Duration::from_secs(120),
        }
    }
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: FString,
    prompt: FString,
    stream: FBool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: FString,
}

#[derive(Clone)]
pub struct OllamaAdapter {
    config: OllamaConfig,
    client: Client,
}

impl OllamaAdapter {
    pub fn new(config: OllamaConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .context("Failed to create Ollama HTTP client")?;
        Ok(Self { config, client })
    }

    pub async fn generate(&self, prompt: &str) -> Result<FString> {
        let request = GenerateRequest {
            model: self.config.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };
        let url = format!("{}/api/generate", self.config.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Ollama generate request failed")?;
        let body: GenerateResponse = resp.json().await.context("Invalid Ollama response")?;
        Ok(body.response)
    }
}
