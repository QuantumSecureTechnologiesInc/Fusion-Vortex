use anyhow::Result;

pub async fn handle_request(_request: serde_json::Value) -> Result<serde_json::Value> {
    // Placeholder request handler
    Ok(serde_json::json!({
        "status": "ok",
        "response": "Mock AI daemon response"
    }))
}
