pub mod config;
pub mod core;
pub mod flow;
pub mod processors;

// Re-export commonly used types
pub use config::Settings;
pub use core::TensorData;
pub use flow::{FlowProcessor, TensorWeaveEngine};
