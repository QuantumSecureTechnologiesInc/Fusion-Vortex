//! Runtime configuration

use fusion_runtime_hal::GpuBackend;
use serde::{Deserialize, Serialize};

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Enable GPU acceleration
    pub enable_gpu: bool,

    /// Enable QPU support
    pub enable_qpu: bool,

    /// Quality of Service mode
    pub qos_mode: QoSMode,

    /// GPU backend to use
    pub gpu_backend: GpuBackend,

    /// Number of worker threads
    pub worker_threads: usize,

    /// Memory pool size in bytes
    pub memory_pool_size: usize,
}

/// Quality of Service modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QoSMode {
    /// Ultra-low latency (< 10μs) for HFT and quantum control
    UltraLowLatency,

    /// Low latency (< 100μs) for general financial applications
    LowLatency,

    /// Balanced mode for mixed workloads
    Balanced,

    /// High throughput for batch AI/ML workloads
    HighThroughput,
}

impl Default for QoSMode {
    fn default() -> Self {
        QoSMode::Balanced
    }
}
