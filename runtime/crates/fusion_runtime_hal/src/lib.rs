//! # Fusion Runtime Hardware Abstraction Layer (HAL)
//!
//! Direct hardware bindings for maximum performance, bypassing high-level abstractions.
//!
//! ## Supported Backends
//!
//! - **GPU**: CUDA (NVIDIA), Metal (Apple), Vulkan (Cross-platform)
//! - **Network**: Standard sockets, DPDK (ultra-low latency)
//! - **QPU**: IBM Quantum, Rigetti, IonQ
//!
//! ## Architecture
//!
//! The HAL provides three main interfaces:
//!
//! 1. **GPU Kernel Executor**: Direct kernel launch with precise timing
//! 2. **Network Device Interface**: Bypass kernel for packet processing
//! 3. **QPU Interface**: Async job submission to quantum hardware

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};

mod gpu;
mod network;
mod qpu;

pub use gpu::{GpuBackend, GpuKernelExecutor};
pub use network::NetworkInterface;
pub use qpu::{QpuInterface, QpuProvider};

/// Hardware abstraction layer coordinating all devices
pub struct HardwareLayer {
    /// GPU executor
    gpu_exec: Option<Arc<GpuKernelExecutor>>,

    /// Network interface
    network: Arc<NetworkInterface>,

    /// QPU interface
    qpu: Option<Arc<QpuInterface>>,

    /// Configuration
    config: HalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalConfig {
    pub enable_gpu: bool,
    pub gpu_backend: GpuBackend,
    pub enable_qpu: bool,
    pub qpu_provider: Option<QpuProvider>,
    pub enable_dpdk: bool,
}

impl HardwareLayer {
    pub fn new(config: &RuntimeConfig) -> Self {
        info!("Initialising Hardware Abstraction Layer");

        let hal_config = HalConfig {
            enable_gpu: config.enable_gpu,
            gpu_backend: config.gpu_backend,
            enable_qpu: config.enable_qpu,
            qpu_provider: if config.enable_qpu {
                Some(QpuProvider::IbmQuantum)
            } else {
                None
            },
            enable_dpdk: false, // Requires root privileges
        };

        // Initialize GPU if enabled
        let gpu_exec = if hal_config.enable_gpu {
            debug!(
                "Initialising GPU executor with backend: {:?}",
                hal_config.gpu_backend
            );
            Some(Arc::new(GpuKernelExecutor::new(hal_config.gpu_backend)))
        } else {
            None
        };

        // Initialize network interface
        let network = Arc::new(NetworkInterface::new(hal_config.enable_dpdk));

        // Initialize QPU if enabled
        let qpu = if hal_config.enable_qpu {
            if let Some(provider) = hal_config.qpu_provider {
                debug!("Initialising QPU interface with provider: {:?}", provider);
                Some(Arc::new(QpuInterface::new(provider)))
            } else {
                None
            }
        } else {
            None
        };

        info!("HAL initialisation complete");

        Self {
            gpu_exec,
            network,
            qpu,
            config: hal_config,
        }
    }

    /// Get GPU executor (if available)
    pub fn gpu(&self) -> Option<&GpuKernelExecutor> {
        self.gpu_exec.as_deref()
    }

    /// Get network interface
    pub fn network(&self) -> &NetworkInterface {
        &self.network
    }

    /// Get QPU interface (if available)
    pub fn qpu(&self) -> Option<&QpuInterface> {
        self.qpu.as_deref()
    }
}

// Stub RuntimeConfig
pub struct RuntimeConfig {
    pub enable_gpu: bool,
    pub gpu_backend: GpuBackend,
    pub enable_qpu: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hal_creation() {
        let config = RuntimeConfig {
            enable_gpu: false,
            gpu_backend: GpuBackend::Auto,
            enable_qpu: false,
        };

        let hal = HardwareLayer::new(&config);
        assert!(hal.gpu().is_none());
        assert!(hal.qpu().is_none());
    }
}
