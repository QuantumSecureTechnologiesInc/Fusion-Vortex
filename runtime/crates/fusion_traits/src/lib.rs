//! # Fusion Traits
//!
//! Foundational traits for the Fusion type system.
//! Integrated from fusion_core.

mod conversions;
mod numeric;
mod unitary;

pub use conversions::{ToClassical, ToQuantumState, ToTensor};
pub use numeric::{DataType, Numeric};
pub use unitary::Unitary;
