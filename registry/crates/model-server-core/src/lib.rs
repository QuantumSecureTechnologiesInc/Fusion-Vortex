use fusion_ai_core::{Layer, Linear, Variable};
use fusion_core::types::hybrid::HybridValue;
use fusion_core::types::tensor::{Matrix, Tensor};
/// Fusion AI Model Server.
///
/// This application demonstrates the "Interwoven" strategy in action.
/// It uses the Tensor engine (Core) inside a Web Server (Ext) to serve AI predictions (Pillar).
use fusion_http::{Request, Response, Server};
use fusion_json;
use fusion_std::error::StdResult;
use std::sync::Arc;
use tokio::sync::Mutex;

// --- 1. Define the AI Model ---

struct MyModel {
    layer: Linear,
}

impl MyModel {
    fn new() -> Self {
        // A simple linear regression model: y = Wx + b
        // Input features: 2, Output classes: 1
        Self {
            layer: Linear::new(2, 1),
        }
    }

    fn predict(&self, input: Vec<f64>) -> f64 {
        // Convert Vec<f64> -> Tensor (Matrix 1x2)
        let input_tensor = Matrix::from_vec(input, [1, 2]);
        let input_var = Variable::new(input_tensor);

        // Forward pass
        let output_var = self.layer.forward(input_var);

        // Extract result (Tensor -> f64)
        // Simplified: getting index [0,0]
        output_var.data.get([0, 0])
    }
}

// --- 2. Define Request/Response DTOs ---

#[derive(serde::Deserialize)]
struct InferenceRequest {
    features: Vec<f64>,
}

// --- 3. Main Application Entry Point ---

#[tokio::main]
async fn main() -> StdResult<()> {
    println!("Initializing Fusion Model Server...");

    // Load Model (Thread-safe reference)
    let model = Arc::new(Mutex::new(MyModel::new()));

    // Define Request Handler
    let handler = move |req: Request<Vec<u8>>| -> Response<Vec<u8>> {
        match (req.method.as_str(), req.path.as_str()) {
            ("POST", "/predict") => {
                // Parse Body
                let body_str = String::from_utf8_lossy(req.body());

                // Use Serde directly for DTO parsing (Standard Rust pattern)
                // In a pure Fusion app, we might use fusion_json::parse -> HybridValue -> Struct
                let inference_req: InferenceRequest = match serde_json::from_str(&body_str) {
                    Ok(r) => r,
                    Err(_) => return Response::new(400).body(b"Invalid JSON"),
                };

                if inference_req.features.len() != 2 {
                    return Response::new(400).body(b"Model expects 2 features");
                }

                // Run Inference (Locking model for thread safety if it had mutable state)
                // Here MyModel is read-only logic effectively, but we lock to simulate shared access
                let prediction = {
                    let m = model.blocking_lock(); // Use sync lock for demo math
                    m.predict(inference_req.features)
                };

                // Serialize Response using Fusion JSON
                let response_val = HybridValue::Float(prediction);
                let json_body = fusion_json::to_string(&response_val).unwrap_or_default();

                Response::ok().body(json_body)
            }

            ("GET", "/health") => {
                Response::ok().body(b"{\"status\": \"healthy\", \"framework\": \"Fusion AI\"}")
            }

            _ => Response::not_found(),
        }
    };

    // Start Server
    let server = Server::new(handler);
    server.listen("127.0.0.1:3000").await?;

    Ok(())
}
