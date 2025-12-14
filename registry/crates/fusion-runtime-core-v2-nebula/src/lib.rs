// Library exports for Fusion Runtime Core v2.0 Nebula

pub mod engine;

// Re-export the generated proto types
pub mod fusion_proto {
    tonic::include_proto!("fusion_core_v2");
}

pub use engine::WasmEngine;
