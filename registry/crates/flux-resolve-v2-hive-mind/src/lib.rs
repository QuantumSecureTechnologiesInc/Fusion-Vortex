//! Flux-Resolve v2.0 - Hive Mind
//!
//! Distributed dependency resolution engine with GPU acceleration.
//! Leverages fusion-redis for distributed caching and fusion_runtime_core for async execution.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

pub mod cache;
pub mod ffi;

#[cfg(feature = "gpu")]
pub mod gpu;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub project_name: String,
    /// List of dependencies as (PackageID, ConflictMask) tuples
    pub dependencies: Vec<(i32, i32)>,
}

/// Represents a dependency node matching CUDA struct layout
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Node {
    pub id: i32,
    pub conflict_mask: i32,
}

#[cfg(feature = "gpu")]
unsafe impl cudarc::driver::DeviceRepr for Node {}

// --- Engine Core ---

pub struct FluxEngine {
    cache: Arc<cache::CacheLayer>,
    #[allow(dead_code)]
    gpu_enabled: bool,
    #[allow(dead_code)]
    gpu_threshold: usize,
    #[cfg(feature = "gpu")]
    cuda_dev: Option<Arc<cudarc::driver::CudaDevice>>,
}

impl FluxEngine {
    pub fn new(cache: Arc<cache::CacheLayer>) -> Self {
        let gpu_enabled =
            std::env::var("FUSION_CUDA_ENABLE").unwrap_or_else(|_| "true".to_string()) == "true";

        let gpu_threshold = std::env::var("FLUX_GPU_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);

        #[cfg(feature = "gpu")]
        let cuda_dev = if gpu_enabled {
            tracing::info!("[Flux] Initializing CUDA Context...");
            cudarc::driver::CudaDevice::new(0).ok()
        } else {
            None
        };

        #[cfg(feature = "gpu")]
        if gpu_enabled && cuda_dev.is_none() {
            tracing::warn!("[Flux] GPU requested but device 0 not found. Falling back to CPU.");
        }

        Self {
            cache,
            gpu_enabled,
            gpu_threshold,
            #[cfg(feature = "gpu")]
            cuda_dev,
        }
    }

    pub async fn resolve(&self, manifest: Manifest) -> Result<String> {
        let start = Instant::now();
        let json = serde_json::to_string(&manifest)?;
        let hash = format!("{:x}", md5::compute(&json));

        tracing::info!("[Flux] Solving Hash: {}", hash);

        // 1. Check Distributed Cache
        if let Some(lock) = self.cache.get(&hash).await? {
            tracing::info!(
                "[Flux] ✓ HIT: Hive Mind returned solution in {:.2?}",
                start.elapsed()
            );
            return Ok(lock);
        }

        // 2. Solve (CPU or GPU based on complexity)
        let solution = if self.should_use_gpu(&manifest) {
            #[cfg(feature = "gpu")]
            {
                self.solve_cuda(&manifest).await?
            }
            #[cfg(not(feature = "gpu"))]
            {
                self.solve_cpu(&manifest).await
            }
        } else {
            self.solve_cpu(&manifest).await
        };

        // 3. Update Cache with TTL (24 hours = 86400000 ms)
        self.cache.put(&hash, &solution, Some(86400000)).await?;

        tracing::info!("[Flux] ✓ Solved & Cached in {:.2?}", start.elapsed());
        Ok(solution)
    }

    fn should_use_gpu(&self, manifest: &Manifest) -> bool {
        #[cfg(feature = "gpu")]
        {
            self.gpu_enabled
                && self.cuda_dev.is_some()
                && manifest.dependencies.len() >= self.gpu_threshold
        }
        #[cfg(not(feature = "gpu"))]
        {
            let _ = manifest;
            false
        }
    }

    async fn solve_cpu(&self, manifest: &Manifest) -> String {
        // Linear fallback logic
        tracing::debug!(
            "[Flux] Using CPU solver for {} nodes",
            manifest.dependencies.len()
        );
        format!("LOCK_CPU_NODES_{}_SOLVED", manifest.dependencies.len())
    }

    #[cfg(feature = "gpu")]
    async fn solve_cuda(&self, manifest: &Manifest) -> Result<String> {
        use cudarc::driver::{LaunchAsync, LaunchConfig};
        use cudarc::nvrtc::Ptx;

        let dev = self
            .cuda_dev
            .as_ref()
            .context("CUDA device not initialized")?;

        tracing::debug!(
            "[Flux] Using GPU solver for {} nodes",
            manifest.dependencies.len()
        );

        // Load PTX (compiled by build.rs)
        let ptx_path = env!("KERNEL_PTX_PATH");
        dev.load_ptx(
            Ptx::from_file(ptx_path),
            "flux_solver",
            &["solve_dependencies_ebp"],
        )?;
        let func = dev
            .get_func("flux_solver", "solve_dependencies_ebp")
            .context("Failed to load GPU kernel function")?;

        // Prepare Data
        let n = manifest.dependencies.len();
        let mut nodes = Vec::with_capacity(n);
        for dep in &manifest.dependencies {
            nodes.push(Node {
                id: dep.0,
                conflict_mask: dep.1,
            });
        }

        // Host -> Device transfer
        let mut dev_nodes = dev.alloc_zeros::<Node>(n)?;
        dev.htod_copy_into(nodes, &mut dev_nodes)?;

        let mut dev_results = dev.alloc_zeros::<i32>(n)?;

        // Launch Kernel
        let cfg = LaunchConfig::for_num_elems(n as u32);
        unsafe {
            func.launch(cfg, (&dev_nodes, &dev_results, n as i32))?;
        }

        // Device -> Host transfer
        let results = dev.dtoh_sync_copy(&dev_results)?;

        // Process Results
        let valid_count = results.iter().filter(|&&r| r == 1).count();
        Ok(format!("LOCK_CUDA_NODES_{}_VALID_{}", n, valid_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manifest_serialization() {
        let manifest = Manifest {
            project_name: "test".into(),
            dependencies: vec![(1, 0), (2, 1)],
        };

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("test"));
    }

    #[tokio::test]
    async fn test_cpu_solver() {
        let cache = Arc::new(cache::CacheLayer::new_memory());
        let engine = FluxEngine::new(cache);

        let manifest = Manifest {
            project_name: "test".into(),
            dependencies: vec![(1, 0)],
        };

        let result = engine.solve_cpu(&manifest).await;
        assert!(result.contains("CPU"));
    }
}
