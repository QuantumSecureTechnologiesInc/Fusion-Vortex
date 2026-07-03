pub mod runner;

/// AI models version
pub const AI_MODELS_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use runner::LocalModelRunner;
