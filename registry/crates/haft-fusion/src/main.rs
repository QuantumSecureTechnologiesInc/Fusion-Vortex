use clap::Parser;
use haft_fusion::FluxTensor;
use std::sync::Arc;
use tokio::signal;

/// Simple production binary for HAFT.
#[derive(Parser, Debug)]
#[command(
    name = "haft-fusion",
    version,
    about = "Production ready HAFT tensor manager"
)]
struct Args {
    /// Shape of the tensor, e.g. "2,3,4"
    #[arg(short, long, value_delimiter = ',')]
    shape: Vec<usize>,

    /// Hot memory limit before compression (number of elements)
    #[arg(short = 'l', long, default_value_t = 1_000_000)]
    hot_limit: usize,

    /// Variance threshold for researcher warnings
    #[arg(short = 't', long, default_value_t = 1.0)]
    variance_threshold: f64,
}

#[tokio::main]
async fn main() {
    // Initialise logger
    env_logger::init();

    let args = Args::parse();
    let tensor = Arc::new(FluxTensor::new(args.shape));

    // Spawn agents with provided configuration
    // Researcher uses threshold, Builder uses hot_limit, Optimizer default
    // We'll create custom agents here to pass parameters.
    use haft_fusion::{Agent, Builder, Optimizer, Researcher};
    let researcher = Researcher::new(tensor.clone(), args.variance_threshold);
    let builder = Builder::new(tensor.clone(), args.hot_limit);
    let optimizer = Optimizer::new(tensor.clone());

    tokio::spawn(async move { researcher.run().await });
    tokio::spawn(async move { builder.run().await });
    tokio::spawn(async move { optimizer.run().await });

    // Wait for termination signal (Ctrl+C)
    println!("HAFT tensor running. Press Ctrl+C to exit.");
    signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
    println!("Shutting down HAFT agents.");
}
