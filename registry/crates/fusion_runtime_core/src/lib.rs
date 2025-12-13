//! # Fusion Runtime Core
//!
//! A custom, heterogeneous runtime engine designed specifically for Quantum/AI/Classical
//! hybrid workloads. Components are interwoven to work together seamlessly.

use parking_lot::RwLock;
use std::sync::Arc;
use tracing::info;

pub use fusion_runtime_hal::{GpuBackend, HardwareLayer};
pub use fusion_runtime_mem_mgr::{DeviceMemory, MemoryManager};
pub use fusion_runtime_scheduler::{Scheduler, TaskPriority, TaskQueue, VariationalLoopController};

mod config;
mod executor;
mod task;

// Low-level interwoven components
mod collective_comms;
mod device_memory;
mod event_poller;
mod fiber;
mod qpu_sequencer;
mod shared_memory;
mod timer;

pub use config::{QoSMode, RuntimeConfig};
pub use executor::Executor;
pub use task::{JoinError, Task, TaskHandle};

// Export interwoven components
pub use collective_comms::{CollectiveComms, CollectiveOp, CommBackend, CommHandle, ReduceOp};
pub use device_memory::{DeviceMemHandle, DeviceMemStats, DeviceMemoryAllocator, DeviceType};
pub use event_poller::{EventId, EventPoller, EventType, FusedIoReactor, IoEvent};
pub use fiber::{FiberContext, FiberScheduler, FiberState, FiberStats};
pub use qpu_sequencer::{CircuitRequest, QpuJobId, QpuJobResult, QpuJobSequencer, SequencerStats};
pub use shared_memory::{SharedMemoryManager, ShmId, ShmRegion};
pub use timer::{global_timer, Deadline, LowJitterTimer};

/// The main Fusion runtime with interwoven components.
///
/// Components work together in an integrated fashion:
/// - Fiber Scheduler ↔ VLC ↔ Timer
/// - Event Poller ↔ Collective Comms ↔ QPU Sequencer  
/// - Shared Memory ↔ Device Memory ↔ Memory Manager
/// - All components coordinate through Scheduler and HAL
pub struct Runtime {
    // === INTERWOVEN CONTROL & OPTIMIZATION ===
    fiber_scheduler: Arc<FiberScheduler>,
    timer: Arc<LowJitterTimer>,
    event_poller: Arc<FusedIoReactor>,
    vlc: Arc<VariationalLoopController>,

    // === INTERWOVEN RESOURCE MANAGEMENT ===
    shared_memory: Arc<SharedMemoryManager>,
    device_memory: Arc<DeviceMemoryAllocator>,
    memory_manager: Arc<MemoryManager>,

    // === INTERWOVEN COMMUNICATION ===
    collective_comms: Arc<CollectiveComms>,
    qpu_sequencer: Arc<QpuJobSequencer>,

    // === CORE COORDINATION ===
    scheduler: Arc<Scheduler>,
    hal: Arc<HardwareLayer>,
    config: RuntimeConfig,
    executor: Arc<Executor>,
    metrics: Arc<RwLock<RuntimeMetrics>>,
}

/// Runtime performance metrics
#[derive(Debug, Default, Clone)]
pub struct RuntimeMetrics {
    pub tasks_spawned: u64,
    pub tasks_completed: u64,
    pub gpu_kernel_launches: u64,
    pub qpu_submissions: u64,
    pub zero_copy_transfers: u64,
    pub total_latency_us: u64,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::default()
    }

    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.executor.block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> TaskHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.metrics.write().tasks_spawned += 1;
        self.executor.spawn(future, TaskPriority::Normal)
    }

    // === Accessor methods for all interwoven components ===

    pub fn fiber_scheduler(&self) -> &FiberScheduler {
        &self.fiber_scheduler
    }
    pub fn timer(&self) -> &LowJitterTimer {
        &self.timer
    }
    pub fn event_poller(&self) -> &FusedIoReactor {
        &self.event_poller
    }
    pub fn vlc(&self) -> &VariationalLoopController {
        &self.vlc
    }

    pub fn shared_memory(&self) -> &SharedMemoryManager {
        &self.shared_memory
    }
    pub fn device_memory(&self) -> &DeviceMemoryAllocator {
        &self.device_memory
    }
    pub fn memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }

    pub fn collective_comms(&self) -> &CollectiveComms {
        &self.collective_comms
    }
    pub fn qpu_sequencer(&self) -> &QpuJobSequencer {
        &self.qpu_sequencer
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }
    pub fn hal(&self) -> &HardwareLayer {
        &self.hal
    }
    pub fn executor(&self) -> &Executor {
        &self.executor
    }
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
    pub fn metrics(&self) -> RuntimeMetrics {
        self.metrics.read().clone()
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for configuring the Fusion runtime
#[derive(Default)]
pub struct RuntimeBuilder {
    enable_gpu: bool,
    enable_qpu: bool,
    qos_mode: Option<QoSMode>,
    gpu_backend: Option<GpuBackend>,
    worker_threads: Option<usize>,
    memory_pool_size: Option<usize>,
}

impl RuntimeBuilder {
    pub fn enable_gpu(mut self) -> Self {
        self.enable_gpu = true;
        self
    }

    pub fn enable_qpu(mut self) -> Self {
        self.enable_qpu = true;
        self
    }

    pub fn enable_qos(mut self, mode: QoSMode) -> Self {
        self.qos_mode = Some(mode);
        self
    }

    pub fn gpu_backend(mut self, backend: GpuBackend) -> Self {
        self.gpu_backend = Some(backend);
        self
    }

    pub fn worker_threads(mut self, threads: usize) -> Self {
        self.worker_threads = Some(threads);
        self
    }

    pub fn memory_pool_size(mut self, size: usize) -> Self {
        self.memory_pool_size = Some(size);
        self
    }

    pub fn build(self) -> Runtime {
        info!("Building Fusion Runtime with interwoven components");

        let config = RuntimeConfig {
            enable_gpu: self.enable_gpu,
            enable_qpu: self.enable_qpu,
            qos_mode: self.qos_mode.unwrap_or(QoSMode::Balanced),
            gpu_backend: self.gpu_backend.unwrap_or(GpuBackend::Auto),
            worker_threads: self.worker_threads.unwrap_or_else(num_cpus),
            memory_pool_size: self.memory_pool_size.unwrap_or(1024 * 1024 * 1024),
        };

        // Initialize all interwoven components
        let fiber_scheduler = Arc::new(FiberScheduler::new());
        let timer = Arc::new(LowJitterTimer::new());
        let event_poller = Arc::new(FusedIoReactor::new());
        let shared_memory = Arc::new(SharedMemoryManager::new());
        let device_memory = Arc::new(DeviceMemoryAllocator::new());
        let collective_comms = Arc::new(CollectiveComms::new(CommBackend::Nccl));
        let qpu_sequencer = Arc::new(QpuJobSequencer::default());

        // Core components
        // Core components
        let scheduler_config: fusion_runtime_scheduler::RuntimeConfig = (&config).into();
        let scheduler = Arc::new(Scheduler::new(&scheduler_config));

        let mem_config: fusion_runtime_mem_mgr::RuntimeConfig = (&config).into();
        let memory_manager = Arc::new(MemoryManager::new(&mem_config));

        let hal_config: fusion_runtime_hal::RuntimeConfig = (&config).into();
        let hal = Arc::new(HardwareLayer::new(&hal_config));

        // Executor doesn't take config in constructor, fixed to match signature
        // Assuming Executor::new(scheduler, &config) was intentional but if specific signature changed:
        // Checking executor::new signature: new(id: usize, scheduler: Arc<Scheduler>, shutdown: Arc<Mutex<bool>>) -> Self
        // It seems the Executor instantiation line was incorrect based on recent edits if Executor::new has changed.
        // Let's assume Executor::new takes (scheduler, &config) based on the code I read in lib.rs:230.
        // Wait, I saw "executor.rs:66:23 fn new(id: usize, scheduler: Arc<Scheduler>, shutdown: Arc<Mutex<bool>>) -> Self" in previous warning.
        // This implies Executor definition might not match usage!
        // But let's fix the config type references first which are definitely wrong.
        // And I'll fix Executor usage to... wait, if Executor::new is id, scheduler, shutdown, I need to provide those.
        // But maybe there is a pub fn new wrapper?
        // Let's stick to fixing the config references using into() as I implemented the From traits.
        // For Executor, I will assume signature in lib.rs 230 was intended for a different Executor or wrapper.
        // Given I cannot see executor.rs fully, I should look at "crates/fusion_runtime_core/src/executor.rs".
        // Use default Executor construction for now if possible or adapt.
        // Actually, let's look at the Executor type.

        let executor_config: fusion_runtime_scheduler::RuntimeConfig = (&config).into();
        // Assuming Executor needs scheduler and maybe config?
        // Line 230: let executor = Arc::new(Executor::new(scheduler.clone(), &config));
        // If Executor::new signature is different, this line is wrong.
        // I will trust the user intended signature and just fix the config type passed to it if it expects a specific config.
        // But wait, Executor is in the CURRENT crate. So &config (RuntimeConfig) IS the correct type for Executor::new if it takes &RuntimeConfig.
        // The error log didn't complain about Executor::new config type, only Scheduler, MemoryManager, Hal.
        let executor = Arc::new(Executor::new(scheduler.clone(), &config));
        // Ah, if I change 227-229, I should leave 230 alone if it wasn't erroring.

        // Re-reading error log carefully:
        // error[E0308]: mismatched types --> crates\fusion_runtime_core\src\lib.rs:227:49
        // error[E0308]: mismatched types --> crates\fusion_runtime_core\src\lib.rs:228:58
        // error[E0308]: mismatched types --> crates\fusion_runtime_core\src\lib.rs:229:47
        // That's it for lib.rs errors.

        let executor = Arc::new(Executor::new(scheduler.clone(), &config));
        let vlc = Arc::new(VariationalLoopController::new());
        let metrics = Arc::new(RwLock::new(RuntimeMetrics::default()));

        info!("✅ All interwoven components initialized:");
        info!("   Control: Fiber Scheduler, Timer, Event Poller");
        info!("   Optimization: VLC");
        info!("   Resources: Shared Memory, Device Memory, Memory Manager");
        info!("   Communication: Collective Comms, QPU Sequencer");
        info!("   Core: Scheduler, HAL, Executor");

        Runtime {
            fiber_scheduler,
            timer,
            event_poller,
            vlc,
            shared_memory,
            device_memory,
            memory_manager,
            collective_comms,
            qpu_sequencer,
            scheduler,
            hal,
            config,
            executor,
            metrics,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new();
        assert!(runtime.config().worker_threads > 0);
    }

    #[test]
    fn test_all_components_accessible() {
        let runtime = Runtime::builder().enable_gpu().enable_qpu().build();

        // Test all interwoven components are accessible
        let _ = runtime.fiber_scheduler();
        let _ = runtime.timer();
        let _ = runtime.event_poller();
        let _ = runtime.vlc();
        let _ = runtime.shared_memory();
        let _ = runtime.device_memory();
        let _ = runtime.collective_comms();
        let _ = runtime.qpu_sequencer();
        let _ = runtime.scheduler();
        let _ = runtime.hal();
    }
}
