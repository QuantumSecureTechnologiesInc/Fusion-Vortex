mod ai;
mod quantum;

use fusion_web::{extract::Json, Router, Server};
use serde::{Deserialize, Serialize};
use ai::ReturnPredictor;
use quantum::PortfolioOptimizer;
use fusion_ai_core::Tensor;

#[derive(Deserialize)]
struct PortfolioRequest {
    tickers: Vec<String>,
    history: Vec<f64>, // Flattened list of prices
}

#[derive(Serialize)]
struct PortfolioResponse {
    selected_assets: Vec<String>,
    predicted_return: f64,
}

async fn handle_optimization(Json(payload): Json<PortfolioRequest>) -> Json<PortfolioResponse> {
    println!("Received request for {} assets", payload.tickers.len());

    // 1. AI Step: Predict returns
    let predictor = ReturnPredictor::new();
    let history_tensor = Tensor::from_slice(&payload.history);
    let expected_returns = predictor.predict(&history_tensor);
    
    // 2. Quantum Step: Optimize selection
    let optimizer = PortfolioOptimizer::new(payload.tickers.len() as i32);
    let risk_matrix = Tensor::eye(payload.tickers.len()); // Dummy identity risk
    let selection_mask = optimizer.optimize(&expected_returns, &risk_matrix);
    
    // 3. Classical Aggregation
    let mut selected_assets = Vec::new();
    let mut total_return = 0.0;
    
    for (i, is_selected) in selection_mask.iter().enumerate() {
        if *is_selected {
            selected_assets.push(payload.tickers[i].clone());
            total_return += expected_returns[i];
        }
    }

    Json(PortfolioResponse {
        selected_assets,
        predicted_return: total_return,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/optimize", fusion_web::post(handle_optimization));

    println!("Tri-brid Server running on http://localhost:3000");
    
    // In a real impl, bind would return a Result
    Server::bind("0.0.0.0:3000")
        .serve(app.into_make_service())
        .await
        .unwrap();
}
