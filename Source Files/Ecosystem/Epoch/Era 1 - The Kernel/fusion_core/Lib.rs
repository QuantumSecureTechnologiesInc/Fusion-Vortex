pub mod types;
pub mod traits;
pub mod ops;
pub mod compiler;
pub mod error; // Export Error module

pub use error::{FusionError, FusionResult};
// ... other exports ...