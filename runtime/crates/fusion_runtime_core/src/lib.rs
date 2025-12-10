pub use fusion_quantum_core::{
    QuantumCircuit, QuantumGate, QuantumRegistry, QuantumState, Qubit, QubitId,
};
pub use fusion_tensor_core::{Matrix, Scalar, Tensor, TensorOps, Vector};
pub use fusion_traits::{DataType, Numeric, Unitary};

/// The main Fusion runtime with interwoven components.
///
/// Components work together in an integrated fashion:
/// - Fiber Scheduler ↔ VLC ↔ Timer
/// - Event Poller ↔ Collective Comms ↔ QPU Sequencer  
/// - Shared Memory ↔ Device Memory ↔ Memory Manager
/// - Tensor Core ↔ Device Memory (zero-copy GPU tensors)
/// - Quantum Core ↔ QPU Sequencer (circuit batching)
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

    // === PHASE 2/3: TENSOR & QUANTUM CORES ===
    quantum_registry: Arc<parking_lot::RwLock<fusion_quantum_core::QuantumRegistry>>,

    // === UNIFIED INTERWOVEN CORE ===
    /// Unified core that interweaves traits, tensors, and quantum operations
    fusion_core: Arc<FusionCore>,

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

    // === Phase 2/3: Tensor & Quantum Core Accessors ===

    /// Access the quantum registry for qubit management
    pub fn quantum_registry(&self) -> &parking_lot::RwLock<fusion_quantum_core::QuantumRegistry> {
        &self.quantum_registry
    }

    /// Access the unified interwoven core
    /// This provides seamless integration of traits, tensors, and quantum operations
    pub fn fusion_core(&self) -> &FusionCore {
        &self.fusion_core
    }

    /// Create an interwoven workflow executor
    /// For quantum-classical-tensor hybrid computations
    pub fn create_workflow(&self) -> InterwovenWorkflow {
        InterwovenWorkflow::new(self.fusion_core.clone())
    }

    /// Submit a quantum circuit for execution
    pub fn submit_quantum_circuit(
        &self,
        circuit: fusion_quantum_core::QuantumCircuit,
    ) -> FusionResult<QpuJobId> {
        // Convert circuit to CircuitRequest for QPU sequencer
        let request = CircuitRequest {
            request_id: 0, // Will be assigned by sequencer
            num_qubits: circuit.num_qubits,
            operations: vec![], // Placeholder
            shots: 1000,
        };

        self.qpu_sequencer
            .submit(request)
            .map_err(|e| FusionError::RuntimeError(format!("QPU submission failed: {:?}", e)))
    }

    // === Core Component Accessors ===

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
        let vlc = Arc::new(VariationalLoopController::new());
        let metrics = Arc::new(RwLock::new(RuntimeMetrics::default()));

        info!("✅ All interwoven components initialized:");
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
