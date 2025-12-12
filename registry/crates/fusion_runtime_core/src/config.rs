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

// Conversions to sub-component configs

impl From<&RuntimeConfig> for fusion_runtime_scheduler::RuntimeConfig {
    fn from(c: &RuntimeConfig) -> Self {
        fusion_runtime_scheduler::RuntimeConfig {
            worker_threads: c.worker_threads,
            stack_size: 2 * 1024 * 1024, // Default 2MB stack
        }
    }
}

impl From<&RuntimeConfig> for fusion_runtime_mem_mgr::RuntimeConfig {
    fn from(c: &RuntimeConfig) -> Self {
        fusion_runtime_mem_mgr::RuntimeConfig {
            memory_pool_size: c.memory_pool_size,
        }
    }
}

impl From<&RuntimeConfig> for fusion_runtime_hal::RuntimeConfig {
    fn from(c: &RuntimeConfig) -> Self {
        fusion_runtime_hal::RuntimeConfig {
            enable_gpu: c.enable_gpu,
            enable_qpu: c.enable_qpu,
            gpu_backend: c.gpu_backend,
        }
    }
}
