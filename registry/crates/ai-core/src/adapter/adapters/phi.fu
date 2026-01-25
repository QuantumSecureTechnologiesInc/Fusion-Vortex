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
use crate::adapter::adapters::openai::OpenAIMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiConfig {
    pub base_url: FString,
    pub model: FString,
    pub max_tokens: Option<FSize>,
    pub temperature: Option<f32>,
    pub timeout: Duration,
}

impl Default for PhiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11436/v1".to_string(),
            model: "phi-3.5-mini".to_string(),
            max_tokens: Some(4096),
            temperature: Some(0.7),
            timeout: Duration::from_secs(120),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: FString,
    messages: FVec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<FSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<FBool>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: FVec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: FSize,
    pub completion_tokens: FSize,
    pub total_tokens: FSize,
}

#[derive(Clone)]
pub struct PhiAdapter {
    config: PhiConfig,
    client: Client,
}

impl PhiAdapter {
    pub fn new(config: PhiConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .context("Failed to create Phi HTTP client")?;
        Ok(Self { config, client })
    }

    pub async fn chat_completion(
        &self,
        messages: FVec<OpenAIMessage>,
    ) -> Result<(OpenAIMessage, Usage)> {
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: Some(false),
        };
        let url = format!("{}/chat/completions", self.config.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Phi chat completion request failed")?;
        let data: ChatCompletionResponse = response
            .json()
            .await
            .context("Invalid Phi response")?;
        if data.choices.is_empty() {
            anyhow::bail!("No choices returned from Phi");
        }
        Ok((data.choices[0].message.clone(), data.usage))
    }
}
