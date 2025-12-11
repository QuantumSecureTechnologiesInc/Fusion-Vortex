use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{info, warn};

mod server;
mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🤖 Fusion AI Daemon starting...");
    
    let addr = "127.0.0.1:9876";
    info!("Listening on {}", addr);
    
    // Start server
    server::start(addr).await?;
    
    Ok(())
}
