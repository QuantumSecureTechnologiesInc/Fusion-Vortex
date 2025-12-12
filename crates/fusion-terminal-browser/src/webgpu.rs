//! WebGPU integration for GPU-accelerated rendering

use crate::{BrowserError, Result};
use tracing::{debug, info};
use wgpu::{Adapter, Device, Instance, Queue};

/// WebGPU renderer for hardware-accelerated graphics
pub struct WebGpuRenderer {
    instance: Instance,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    enabled: bool,
}

impl WebGpuRenderer {
    /// Create a new WebGPU renderer
    pub fn new(enabled: bool) -> Self {
        info!("Initialising WebGPU renderer (enabled: {})", enabled);

        let instance = Instance::default();

        Self {
            instance,
            adapter: None,
            device: None,
            queue: None,
            enabled,
        }
    }

    /// Initialize WebGPU (async)
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.enabled {
            debug!("WebGPU is disabled, skipping initialisation");
            return Ok(());
        }

        info!("Requesting WebGPU adapter");

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| {
                BrowserError::WebGpu("Failed to find suitable GPU adapter".to_string())
            })?;

        info!("WebGPU adapter: {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Fusion Terminal Browser Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .map_err(|e| BrowserError::WebGpu(e.to_string()))?;

        info!("WebGPU device and queue acquired");

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);

        Ok(())
    }

    /// Check if WebGPU is available and initialised
    pub fn is_available(&self) -> bool {
        self.enabled && self.device.is_some()
    }

    /// Get adapter information
    pub fn adapter_info(&self) -> Option<wgpu::AdapterInfo> {
        self.adapter.as_ref().map(|a| a.get_info())
    }

    /// Process image with GPU acceleration
    pub fn process_image(&self, image_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        if !self.is_available() {
            return Err(BrowserError::WebGpu("WebGPU not initialised".to_string()));
        }

        // This is a placeholder for actual GPU-accelerated image processing
        // In a full implementation, this would use compute shaders for:
        // - Image scaling
        // - Color conversion
        // - Dithering
        // - Edge detection for better ASCII/Unicode mapping

        debug!("Processing image with WebGPU ({}x{})", width, height);

        // For now, return the input data as-is
        Ok(image_data.to_vec())
    }

    /// Get device reference
    pub fn device(&self) -> Option<&Device> {
        self.device.as_ref()
    }

    /// Get queue reference
    pub fn queue(&self) -> Option<&Queue> {
        self.queue.as_ref()
    }
}

/// Initialize WebGPU renderer (blocking)
pub fn init_webgpu(enabled: bool) -> Result<WebGpuRenderer> {
    let mut renderer = WebGpuRenderer::new(enabled);

    if enabled {
        pollster::block_on(renderer.initialize())?;
    }

    Ok(renderer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webgpu_disabled() {
        let renderer = WebGpuRenderer::new(false);
        assert!(!renderer.is_available());
    }

    #[test]
    fn test_webgpu_creation() {
        let renderer = WebGpuRenderer::new(true);
        assert!(renderer.enabled);
    }
}
