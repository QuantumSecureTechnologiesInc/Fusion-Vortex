#![allow(dead_code)]
use fusion_ai_core::{Layer, Linear, Tensor};
use fusion_core::types::classical::ClassicalType;
use fusion_core::types::hybrid::HybridValue;

/// Fusion AI Model Server.
///
/// This application demonstrates the "Interwoven" strategy in action.
/// It uses the Tensor engine (Core) inside a Web Server (Ext) to serve AI predictions (Pillar).
use fusion_http::{Request, Response, Server};
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
        use ndarray::IxDyn;
        // Convert Vec<f64> -> Tensor (Matrix 1x2)
        let mut input_tensor = Tensor::from_slice(&input);

        // Reshape to [1, 2] using ndarray API exposed on .data
        input_tensor.data = input_tensor.data.into_shape(IxDyn(&[1, 2])).unwrap();

        // Forward pass
        let output_var = self.layer.forward(&input_tensor);

        // Extract result
        output_var.item::<f64>()
    }
}

// --- 2. Define Request/Response DTOs ---

#[derive(serde::Deserialize)]
struct InferenceRequest {
    features: Vec<f64>,
}

// --- 3. Main Application Entry Point ---

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Initializing Fusion Model Server...");

    // Load Model (Thread-safe reference)
    let model = Arc::new(Mutex::new(MyModel::new()));

    // Define Request Handler
    let handler = move |req: Request<Vec<u8>>| -> Response<Vec<u8>> {
        match (req.method().as_str(), req.uri().path()) {
            ("POST", "/predict") => {
                // Parse Body
                let body_str = String::from_utf8_lossy(req.body());

                // Use Serde directly for DTO parsing (Standard Rust pattern)
                let inference_req: InferenceRequest = match serde_json::from_str(&body_str) {
                    Ok(r) => r,
                    Err(_) => {
                        return Response::builder()
                            .status(400)
                            .body(b"Invalid JSON".to_vec())
                            .unwrap()
                    }
                };

                if inference_req.features.len() != 2 {
                    return Response::builder()
                        .status(400)
                        .body(b"Model expects 2 features".to_vec())
                        .unwrap();
                }

                // Run Inference
                let prediction = {
                    let m = model.blocking_lock();
                    m.predict(inference_req.features)
                };

                // Serialize Response using Fusion JSON (Simulated with Serde)
                let response_val = HybridValue::Classical(ClassicalType::Float(prediction));
                let json_body = serde_json::to_string(&response_val).unwrap_or_default();

                Response::builder()
                    .status(200)
                    .body(json_body.into_bytes())
                    .unwrap()
            }

            ("GET", "/health") => Response::builder()
                .status(200)
                .body(b"{\"status\": \"healthy\", \"framework\": \"Fusion AI\"}".to_vec())
                .unwrap(),

            _ => Response::builder().status(404).body(Vec::new()).unwrap(),
        }
    };

    // Start Server
    let server = Server::new(handler);
    server.listen("127.0.0.1:3000").await?;

    Ok(())
}
