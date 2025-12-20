// Google Gemini adapter implementation for fusion-ai-core
use anyhow::{Context, Result};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

const GOOGLE_AI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta";
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleConfig {
    pub api_key: String,
    pub model: String,
    pub max_output_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<usize>,
    pub timeout: Duration,
}

impl Default for GoogleConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gemini-pro".to_string(),
            max_output_tokens: Some(2048),
            temperature: Some(0.9),
            top_p: Some(1.0),
            top_k: None,
            timeout: Duration::from_secs(120),
        }
    }
}

#[derive(Debug, Serialize)]
struct GenerateContentRequest {
    contents: Vec<GoogleContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    safety_settings: Option<Vec<SafetySetting>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleContent {
    pub role: String,
    pub parts: Vec<GooglePart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GooglePart {
    Text { text: String },
    InlineData { inline_data: InlineData },
    FunctionCall { function_call: FunctionCall },
    FunctionResponse { function_response: FunctionResponse },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineData {
    pub mime_type: String,
    pub data: String, // base64 encoded
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>, // not used currently
}

#[derive(Debug, Serialize, Deserialize)]
struct SafetySetting {
    category: String,
    threshold: String,
}

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: GoogleContent,
    #[allow(dead_code)]
    finish_reason: Option<String>,
    #[allow(dead_code)]
    safety_ratings: Option<Vec<SafetyRating>>, // ignored for now
}

#[derive(Debug, Deserialize)]
struct SafetyRating {
    #[allow(dead_code)]
    category: String,
    #[allow(dead_code)]
    probability: String,
}

#[derive(Debug, Deserialize)]
pub struct UsageMetadata {
    pub prompt_token_count: usize,
    pub candidates_token_count: usize,
    pub total_token_count: usize,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct StreamResponse {
    candidates: Option<Vec<Candidate>>, // streamed candidates
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    #[allow(dead_code)]
    code: i32,
    message: String,
    status: String,
}

pub struct GoogleAdapter {
    config: GoogleConfig,
    client: Client,
}

impl GoogleAdapter {
    pub fn new(config: GoogleConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            anyhow::bail!("Google AI API key is required");
        }
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { config, client })
    }

    pub async fn generate_content(
        &self,
        contents: Vec<GoogleContent>,
    ) -> Result<(GoogleContent, Option<UsageMetadata>)> {
        let generation_config = Some(GenerationConfig {
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            top_k: self.config.top_k,
            max_output_tokens: self.config.max_output_tokens,
            stop_sequences: None,
        });
        let request = GenerateContentRequest {
            contents,
            generation_config,
            safety_settings: Some(self.default_safety_settings()),
        };
        let response = self
            .make_request_with_retry(&request, false)
            .await
            .context("Generate content request failed")?;
        if response.candidates.is_empty() {
            anyhow::bail!("No candidates returned from API");
        }
        Ok((
            response.candidates[0].content.clone(),
            response.usage_metadata,
        ))
    }

    // Simplified streaming: just call generate_content and send the result via channel
    pub async fn generate_content_stream(
        &self,
        contents: Vec<GoogleContent>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let result = self.generate_content(contents).await;
        match result {
            Ok((content, _)) => {
                // Concatenate all text parts
                let mut full_text = String::new();
                for part in content.parts {
                    if let GooglePart::Text { text } = part {
                        full_text.push_str(&text);
                    }
                }
                let _ = tx.send(Ok(full_text)).await;
            }
            Err(e) => {
                let _ = tx.send(Err(e)).await;
            }
        }
        Ok(rx)
    }

    async fn make_request_with_retry(
        &self,
        request: &GenerateContentRequest,
        stream: bool,
    ) -> Result<GenerateContentResponse> {
        let mut last_error = None;
        let mut delay = INITIAL_RETRY_DELAY;
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                debug!("Retry attempt {} after {:?}", attempt, delay);
                sleep(delay).await;
                delay *= 2;
            }
            let endpoint = if stream {
                "streamGenerateContent"
            } else {
                "generateContent"
            };
            let url = format!(
                "{}/models/{}:{}?key={}",
                GOOGLE_AI_BASE, self.config.model, endpoint, self.config.api_key
            );
            match self.client.post(&url).json(request).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        return resp
                            .json::<GenerateContentResponse>()
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
                    if let Ok(err_resp) = resp.json::<ErrorResponse>().await {
                        let msg = format!(
                            "Google AI API error ({}): {}",
                            err_resp.error.status, err_resp.error.message
                        );
                        if should_retry && attempt < MAX_RETRIES - 1 {
                            warn!("{} - retrying", msg);
                            last_error = Some(anyhow::anyhow!(msg));
                            continue;
                        } else {
                            anyhow::bail!(msg);
                        }
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

    fn default_safety_settings(&self) -> Vec<SafetySetting> {
        vec![
            SafetySetting {
                category: "HARM_CATEGORY_HARASSMENT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
        ]
    }

    pub fn calculate_cost(&self, usage: &UsageMetadata) -> f64 {
        let (input_cost, output_cost) = match self.config.model.as_str() {
            "gemini-pro" => (0.00025, 0.0005),
            "gemini-pro-vision" => (0.00025, 0.0005),
            "gemini-ultra" => (0.00125, 0.00375),
            _ => (0.00025, 0.0005),
        };
        let input_total = (usage.prompt_token_count as f64 / 1000.0) * input_cost;
        let output_total = (usage.candidates_token_count as f64 / 1000.0) * output_cost;
        input_total + output_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let cfg = GoogleConfig::default();
        assert_eq!(cfg.model, "gemini-pro");
        assert_eq!(cfg.max_output_tokens, Some(2048));
    }

    #[test]
    fn test_safety_settings() {
        let adapter = GoogleAdapter::new(GoogleConfig {
            api_key: "test-key".to_string(),
            ..Default::default()
        })
        .unwrap();
        let settings = adapter.default_safety_settings();
        assert_eq!(settings.len(), 4);
    }

    #[test]
    fn test_cost_calculation() {
        let adapter = GoogleAdapter::new(GoogleConfig {
            api_key: "test-key".to_string(),
            ..Default::default()
        })
        .unwrap();
        let usage = UsageMetadata {
            prompt_token_count: 1000,
            candidates_token_count: 500,
            total_token_count: 1500,
        };
        let cost = adapter.calculate_cost(&usage);
        assert!(cost > 0.0);
        assert!(cost < 1.0);
    }
}
