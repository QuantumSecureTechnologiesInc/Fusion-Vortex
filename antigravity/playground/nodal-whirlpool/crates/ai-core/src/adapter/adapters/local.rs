use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};
use super::openai::{Message, Usage};

const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    pub base_url: String,
    pub model: String,
    pub api_key: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub timeout: Duration,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434/v1".to_string(),
            model: "llama2".to_string(), // Common default
            api_key: None,
            max_tokens: Some(2048),
            temperature: Some(0.7),
            top_p: Some(1.0),
            timeout: Duration::from_secs(120),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>, // usage might be missing in some local implementations
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: serde_json::Value,
}

pub struct LocalAdapter {
    config: LocalConfig,
    client: Client,
}

impl LocalAdapter {
    pub fn new(config: LocalConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &config.api_key {
            headers.insert(
                "Authorization",
                format!("Bearer {}", key).parse().context("Invalid API key")?,
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { config, client })
    }

    pub async fn chat_completion(&self, messages: Vec<Message>) -> Result<(Message, Usage)> {
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            stream: Some(false),
        };

        let response = self
            .make_request_with_retry(&request)
            .await
            .context("Local chat completion request failed")?;

        if response.choices.is_empty() {
            anyhow::bail!("No choices returned from local API");
        }

        let usage = response.usage.unwrap_or(Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        });

        Ok((response.choices[0].message.clone(), usage))
    }

    pub async fn chat_completion_stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        
        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            stream: Some(false), // Streaming not implemented for local yet to keep it simple
        };

        // We simulate streaming by buffering and sending at once, similar to current OpenAI workaround
        // Ideally we would implement real streaming parsing
        let client = self.client.clone();
        let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
        
        tokio::spawn(async move {
             match client.post(&url).json(&request).send().await {
                Ok(resp) => {
                    match resp.json::<ChatCompletionResponse>().await {
                        Ok(chat_resp) => {
                            if let Some(choice) = chat_resp.choices.first() {
                                let _ = tx.send(Ok(choice.message.content.clone())).await;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(Err(anyhow::anyhow!("Failed to parse response: {}", e))).await;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(anyhow::anyhow!("Request failed: {}", e))).await;
                }
            }
        });

        Ok(rx)
    }

    async fn make_request_with_retry(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let mut last_error = None;
        let mut delay = INITIAL_RETRY_DELAY;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                debug!("Retry attempt {} after {:?}", attempt, delay);
                sleep(delay).await;
                delay *= 2;
            }

            let url = format!("{}/chat/completions", self.config.base_url.trim_end_matches('/'));
            
            match self.client.post(&url).json(request).send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return response
                            .json::<ChatCompletionResponse>()
                            .await
                            .context("Failed to parse response");
                    }

                    let should_retry = matches!(
                        status,
                        StatusCode::TOO_MANY_REQUESTS
                            | StatusCode::INTERNAL_SERVER_ERROR
                            | StatusCode::BAD_GATEWAY
                            | StatusCode::SERVICE_UNAVAILABLE
                            | StatusCode::GATEWAY_TIMEOUT
                    );

                    if let Ok(error_response) = response.json::<ErrorResponse>().await {
                        let error_msg = format!("Local API error: {:?}", error_response.error);
                        if should_retry && attempt < MAX_RETRIES - 1 {
                            warn!("{} - retrying", error_msg);
                            last_error = Some(anyhow::anyhow!(error_msg));
                            continue;
                        } else {
                            anyhow::bail!(error_msg);
                        }
                    } else {
                         // Fallback for non-JSON error
                         let text = response.text().await.unwrap_or_default();
                         let msg = format!("Local API error ({}): {}", status, text);
                         anyhow::bail!(msg);
                    }
                }
                Err(e) => {
                    if (e.is_timeout() || e.is_connect()) && attempt < MAX_RETRIES - 1 {
                        warn!("Request failed: {} - retrying", e);
                        last_error = Some(anyhow::anyhow!(e));
                        continue;
                    }
                    anyhow::bail!("Request failed: {}", e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retries exhausted")))
    }

    pub fn calculate_cost(&self, _usage: &Usage) -> f64 {
        0.0 // Local inference is usually free regarding API cost
    }
}
