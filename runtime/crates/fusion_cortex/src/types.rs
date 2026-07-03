//! # Core Types for the Cortex Engine
//!
//! This module defines the fundamental types used by the Cortex Engine
//! for task scheduling and device management.

use serde::{Deserialize, Serialize};

/// Represents the hardware backbone where code is executed.
///
/// The Fusion Runtime treats all device types as first-class citizens,
/// allowing seamless execution across heterogeneous hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Device {
    /// Classical CPU execution
    Cpu,
    /// GPU execution (index identifies the GPU device)
    Gpu(u32),
    /// Quantum Processing Unit (index identifies the QPU device)
    Qpu(u32),
}

impl Device {
    /// Check if this is a CPU device
    pub fn is_cpu(&self) -> bool {
        matches!(self, Device::Cpu)
    }

    /// Check if this is a GPU device
    pub fn is_gpu(&self) -> bool {
        matches!(self, Device::Gpu(_))
    }

    /// Check if this is a QPU device
    pub fn is_qpu(&self) -> bool {
        matches!(self, Device::Qpu(_))
    }

    /// Get the device index (if applicable)
    pub fn device_index(&self) -> Option<u32> {
        match self {
            Device::Cpu => None,
            Device::Gpu(idx) | Device::Qpu(idx) => Some(*idx),
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::Cpu => write!(f, "CPU"),
            Device::Gpu(idx) => write!(f, "GPU:{}", idx),
            Device::Qpu(idx) => write!(f, "QPU:{}", idx),
        }
    }
}

/// The Intent tells the Cortex what the user values most for this task.
///
/// ## Intent Categories
///
/// - **Critical**: For HFT/Algo-trading where minimal jitter (<10μs) is essential
/// - **HighThroughput**: For AI/ML workloads where maximum FLOPS is desired
/// - **Precision**: For scientific computing where accuracy is paramount
/// - **Background**: For logging, cleanup, and non-urgent tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Intent {
    /// HFT: Minimal Jitter - Always runs on CPU
    Critical = 0,
    /// AI: Maximum FLOPS - Prefers GPU
    HighThroughput = 1,
    /// Science: Accuracy over speed
    Precision = 2,
    /// Logging/Cleanup: Low priority
    Background = 3,
}

impl Intent {
    /// Get the priority level (higher = more urgent)
    pub fn priority(&self) -> u8 {
        match self {
            Intent::Critical => 3,
            Intent::HighThroughput => 2,
            Intent::Precision => 2,
            Intent::Background => 0,
        }
    }

    /// Check if this intent requires CPU-only execution
    pub fn requires_cpu(&self) -> bool {
        matches!(self, Intent::Critical)
    }
}

impl std::fmt::Display for Intent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Intent::Critical => write!(f, "Critical (HFT)"),
            Intent::HighThroughput => write!(f, "HighThroughput (AI)"),
            Intent::Precision => write!(f, "Precision (Science)"),
            Intent::Background => write!(f, "Background"),
        }
    }
}

/// Metadata required for the AI Scheduler to make a prediction.
///
/// The TaskProfile captures the essential characteristics of a task
/// that the Cortex Engine needs to make scheduling decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProfile {
    /// Estimated number of operations (FLOPS)
    pub estimated_ops: u64,
    /// Memory footprint in bytes
    pub memory_footprint_bytes: usize,
    /// The user's intent for this task
    pub intent: Intent,
    /// Number of dependencies this task has
    pub dependencies: usize,
}

impl TaskProfile {
    /// Create a new TaskProfile with default values and the specified intent.
    pub fn new(intent: Intent) -> Self {
        Self {
            estimated_ops: 0,
            memory_footprint_bytes: 0,
            intent,
            dependencies: 0,
        }
    }

    /// Create a TaskProfile with all fields specified.
    pub fn with_details(
        estimated_ops: u64,
        memory_footprint_bytes: usize,
        intent: Intent,
        dependencies: usize,
    ) -> Self {
        Self {
            estimated_ops,
            memory_footprint_bytes,
            intent,
            dependencies,
        }
    }

    /// Update the estimated operations count.
    pub fn with_ops(mut self, ops: u64) -> Self {
        self.estimated_ops = ops;
        self
    }

    /// Update the memory footprint.
    pub fn with_memory(mut self, bytes: usize) -> Self {
        self.memory_footprint_bytes = bytes;
        self
    }

    /// Update the dependency count.
    pub fn with_dependencies(mut self, deps: usize) -> Self {
        self.dependencies = deps;
        self
    }

    /// Calculate a complexity score for this task.
    ///
    /// Higher scores indicate more complex tasks that may benefit
    /// from accelerator offloading.
    pub fn complexity_score(&self) -> f64 {
        let ops_component = (self.estimated_ops as f64).log10().max(0.0);
        let memory_component = (self.memory_footprint_bytes as f64 / 1024.0)
            .log10()
            .max(0.0);
        let dep_component = (self.dependencies as f64).sqrt();

        ops_component * 0.5 + memory_component * 0.3 + dep_component * 0.2
    }
}

impl Default for TaskProfile {
    fn default() -> Self {
        Self::new(Intent::Background)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_display() {
        assert_eq!(format!("{}", Device::Cpu), "CPU");
        assert_eq!(format!("{}", Device::Gpu(0)), "GPU:0");
        assert_eq!(format!("{}", Device::Qpu(1)), "QPU:1");
    }

    #[test]
    fn test_device_type_checks() {
        assert!(Device::Cpu.is_cpu());
        assert!(!Device::Cpu.is_gpu());
        assert!(Device::Gpu(0).is_gpu());
        assert!(Device::Qpu(0).is_qpu());
    }

    #[test]
    fn test_intent_priority() {
        assert!(Intent::Critical.priority() > Intent::Background.priority());
        assert_eq!(
            Intent::HighThroughput.priority(),
            Intent::Precision.priority()
        );
    }

    #[test]
    fn test_task_profile_builder() {
        let profile = TaskProfile::new(Intent::HighThroughput)
            .with_ops(1_000_000)
            .with_memory(1024 * 1024)
            .with_dependencies(2);

        assert_eq!(profile.estimated_ops, 1_000_000);
        assert_eq!(profile.memory_footprint_bytes, 1024 * 1024);
        assert_eq!(profile.dependencies, 2);
    }

    #[test]
    fn test_complexity_score() {
        let simple = TaskProfile::new(Intent::Background);
        let complex = TaskProfile::with_details(
            1_000_000_000,
            1024 * 1024 * 1024,
            Intent::HighThroughput,
            10,
        );

        assert!(complex.complexity_score() > simple.complexity_score());
    }
}
