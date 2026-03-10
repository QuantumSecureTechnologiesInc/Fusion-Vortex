// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::sync::Arc;
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FVec<T> = Vec<T>;
// __FU_COMPAT_END__
use crate::protocol::CallToolResult;
use crate::tool::{McpHandler, McpToolFacet};
use anyhow::{Context, Result};
use fusion_vscode_runtime::LspClient;
use serde_json::Value;
use tokio::sync::Mutex;
/// Adapter to bridge MCP requests to an LSP server
pub struct LspAdapter {
    client: Arc<Mutex<LspClient>>,
    language_id: FString,
}
impl LspAdapter {
    /// Create a new LSP adapter
    pub async fn new(
        command: &str,
        args: &[&str],
        root_uri: &str,
        language_id: &str,
    ) -> Result<Self> {
        let client = LspClient::start(command, args, root_uri)
            .await
            .context("Failed to start LSP client")?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            language_id: language_id.to_string(),
        })
    }
    /// Generate MCP tool facets for this LSP server
    pub fn facets(&self) -> FVec<McpToolFacet> {
        let mut facets = Vec::new();
        facets
            .push(McpToolFacet {
                name: "diagnostics".to_string(),
                description: format!("Get diagnostics for {} files", self.language_id),
                handler: Box::new(LspDiagnosticsHandler {
                    client: self.client.clone(),
                    language_id: self.language_id.clone(),
                }),
            });
        facets
            .push(McpToolFacet {
                name: "symbols".to_string(),
                description: format!(
                    "Get document symbols for {} files", self.language_id
                ),
                handler: Box::new(LspSymbolsHandler {
                    client: self.client.clone(),
                    language_id: self.language_id.clone(),
                }),
            });
        facets
    }
}
/// Handler for LSP diagnostics
pub struct LspDiagnosticsHandler {
    client: Arc<Mutex<LspClient>>,
    language_id: FString,
}
#[async_trait::async_trait]
impl McpHandler for LspDiagnosticsHandler {
    async fn handle(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.unwrap_or(Value::Null);
        let uri = args
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("file:///virtual.fu");
        let text = args
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut client = self.client.lock().await;
        if !text.is_empty() {
            client
                .did_open(uri, &self.language_id, 1, text)
                .await
                .context("Failed to open virtual document for diagnostics")?;
        }
        let diagnostics = client
            .diagnostics(uri)
            .await
            .context("Diagnostics request failed")?;
        let text = serde_json::to_string_pretty(&diagnostics)?;
        Ok(CallToolResult {
            content: vec![crate::protocol::ToolContent::Text { text }],
            is_error: Some(false),
        })
    }
}
/// Handler for LSP symbols
pub struct LspSymbolsHandler {
    client: Arc<Mutex<LspClient>>,
    language_id: FString,
}
#[async_trait::async_trait]
impl McpHandler for LspSymbolsHandler {
    async fn handle(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.unwrap_or(Value::Null);
        let uri = args
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("file:///virtual.fu");
        let text = args
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut client = self.client.lock().await;
        if !text.is_empty() {
            client
                .did_open(uri, &self.language_id, 1, text)
                .await
                .context("Failed to open virtual document for symbol extraction")?;
        }
        let symbols = client
            .document_symbols(uri)
            .await
            .context("Symbol request failed")?;
        let text = serde_json::to_string_pretty(&symbols)?;
        Ok(CallToolResult {
            content: vec![crate::protocol::ToolContent::Text { text }],
            is_error: Some(false),
        })
    }
}
