//! # QPU Job Sequencer
//!
//! Batching QPU Submissions - logic to serialize multiple user requests into
//! one optimized QPU job (batching) and map the probabilistic results back to
//! the individual Fusion Actors.
//!
//! Sits between fusion_q_cloud_agent and the hardware drivers.

use parking_lot::Mutex;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// QPU job ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QpuJobId(pub u64);

/// Individual circuit request
#[derive(Debug, Clone)]
pub struct CircuitRequest {
    /// Request ID (from user)
    pub request_id: u64,

    /// Number of qubits
    pub num_qubits: usize,

    /// Circuit operations (simplified)
    pub operations: Vec<String>,

    /// Number of shots (measurements)
    pub shots: usize,
}

/// Batched QPU job
#[derive(Debug, Clone)]
pub struct BatchedQpuJob {
    /// Job ID
    pub job_id: QpuJobId,

    /// Individual circuits in this batch
    pub circuits: Vec<CircuitRequest>,

    /// Total shots across all circuits
    pub total_shots: usize,
}

/// QPU job result
#[derive(Debug, Clone)]
pub struct QpuJobResult {
    /// Job ID
    pub job_id: QpuJobId,

    /// Results per circuit (indexed by request_id)
    pub results: HashMap<u64, Vec<u8>>, // Measurement outcomes
}

/// QPU Job Sequencer
///
/// Batches multiple circuit requests into optimized QPU jobs
pub struct QpuJobSequencer {
    /// Pending circuit requests
    pending_requests: Arc<Mutex<VecDeque<CircuitRequest>>>,

    /// Active jobs (submitted to QPU)
    active_jobs: Arc<Mutex<HashMap<QpuJobId, BatchedQpuJob>>>,

    /// Completed jobs
    completed_jobs: Arc<Mutex<HashMap<QpuJobId, QpuJobResult>>>,

    /// Next job ID
    next_job_id: Arc<Mutex<u64>>,

    /// Batching configuration
    max_batch_size: usize,
    max_wait_ms: u64,
}

impl QpuJobSequencer {
    /// Create a new QPU job sequencer
    ///
    /// # Arguments
    ///
    /// * `max_batch_size` - Maximum circuits per batch
    /// * `max_wait_ms` - Maximum time to wait for batch to fill
    pub fn new(max_batch_size: usize, max_wait_ms: u64) -> Self {
        Self {
            pending_requests: Arc::new(Mutex::new(VecDeque::new())),
            active_jobs: Arc::new(Mutex::new(HashMap::new())),
            completed_jobs: Arc::new(Mutex::new(HashMap::new())),
            next_job_id: Arc::new(Mutex::new(1)),
            max_batch_size,
            max_wait_ms,
        }
    }

    /// Submit a circuit request
    ///
    /// # Arguments
    ///
    /// * `request` - Circuit request from user
    ///
    /// # Returns
    ///
    /// Request ID for tracking
    pub fn submit_circuit(&self, request: CircuitRequest) -> u64 {
        let request_id = request.request_id;
        self.pending_requests.lock().push_back(request);

        tracing::debug!("Submitted circuit request: id={}", request_id);

        request_id
    }

    /// Try to create a batch from pending requests
    ///
    /// # Returns
    ///
    /// Optional batched job if enough circuits are pending
    pub fn try_create_batch(&self) -> Option<BatchedQpuJob> {
        let mut pending = self.pending_requests.lock();

        if pending.is_empty() {
            return None;
        }

        // Collect circuits for batch
        let batch_size = pending.len().min(self.max_batch_size);
        let circuits: Vec<CircuitRequest> = pending.drain(..batch_size).collect();

        let total_shots: usize = circuits.iter().map(|c| c.shots).sum();

        // Generate job ID
        let mut next_job_id = self.next_job_id.lock();
        let job_id = QpuJobId(*next_job_id);
        *next_job_id += 1;
        drop(next_job_id);

        let job = BatchedQpuJob {
            job_id,
            circuits,
            total_shots,
        };

        tracing::info!(
            "Created QPU batch: job_id={:?}, circuits={}, total_shots={}",
            job_id,
            batch_size,
            total_shots
        );

        Some(job)
    }

    /// Submit batch to QPU
    ///
    /// # Arguments
    ///
    /// * `job` - Batched job to submit
    ///
    /// # Returns
    ///
    /// Job ID for tracking
    pub fn submit_batch(&self, job: BatchedQpuJob) -> QpuJobId {
        let job_id = job.job_id;
        self.active_jobs.lock().insert(job_id, job);

        // In real implementation, would submit to QPU:
        // - IBM Quantum: qiskit.execute()
        // - AWS Braket: braket_client.create_quantum_task()
        // - Google Quantum Engine: quantum_engine.run_circuit()

        tracing::info!("Submitted batch to QPU: job_id={:?}", job_id);

        job_id
    }

    /// Poll for job completion
    ///
    /// # Arguments
    ///
    /// * `job_id` - Job to check
    ///
    /// # Returns
    ///
    /// Optional result if job is complete
    pub fn poll_job(&self, job_id: QpuJobId) -> Option<QpuJobResult> {
        // Check if already completed
        if let Some(result) = self.completed_jobs.lock().get(&job_id) {
            return Some(result.clone());
        }

        // In real implementation, would poll QPU:
        // - IBM: job.status()
        // - AWS: braket_client.get_quantum_task()

        None
    }

    /// Simulate job completion (for testing)
    pub fn simulate_completion(&self, job_id: QpuJobId) {
        let mut active_jobs = self.active_jobs.lock();

        if let Some(job) = active_jobs.remove(&job_id) {
            // Generate fake results
            let mut results = HashMap::new();
            for circuit in &job.circuits {
                // Fake measurement outcomes
                let measurements = vec![0u8; circuit.shots];
                results.insert(circuit.request_id, measurements);
            }

            let result = QpuJobResult { job_id, results };

            self.completed_jobs.lock().insert(job_id, result);

            tracing::info!("Simulated QPU job completion: job_id={:?}", job_id);
        }
    }

    /// Get result for specific circuit request
    ///
    /// # Arguments
    ///
    /// * `request_id` - Original circuit request ID
    ///
    /// # Returns
    ///
    /// Optional measurement results
    pub fn get_circuit_result(&self, request_id: u64) -> Option<Vec<u8>> {
        let completed = self.completed_jobs.lock();

        for result in completed.values() {
            if let Some(measurements) = result.results.get(&request_id) {
                return Some(measurements.clone());
            }
        }

        None
    }

    /// Get sequencer statistics
    pub fn stats(&self) -> SequencerStats {
        SequencerStats {
            pending_requests: self.pending_requests.lock().len(),
            active_jobs: self.active_jobs.lock().len(),
            completed_jobs: self.completed_jobs.lock().len(),
        }
    }
}

impl Default for QpuJobSequencer {
    fn default() -> Self {
        Self::new(10, 100) // 10 circuits per batch, 100ms max wait
    }
}

/// Sequencer statistics
#[derive(Debug, Clone)]
pub struct SequencerStats {
    pub pending_requests: usize,
    pub active_jobs: usize,
    pub completed_jobs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_and_batch() {
        let sequencer = QpuJobSequencer::new(5, 100);

        // Submit 3 circuits
        for i in 0..3 {
            let request = CircuitRequest {
                request_id: i,
                num_qubits: 4,
                operations: vec!["H".to_string(), "CNOT".to_string()],
                shots: 1000,
            };
            sequencer.submit_circuit(request);
        }

        let stats = sequencer.stats();
        assert_eq!(stats.pending_requests, 3);

        // Create batch
        let batch = sequencer.try_create_batch().unwrap();
        assert_eq!(batch.circuits.len(), 3);
        assert_eq!(batch.total_shots, 3000);
    }

    #[test]
    fn test_job_completion() {
        let sequencer = QpuJobSequencer::new(5, 100);

        let request = CircuitRequest {
            request_id: 100,
            num_qubits: 4,
            operations: vec!["H".to_string()],
            shots: 500,
        };
        sequencer.submit_circuit(request);

        let batch = sequencer.try_create_batch().unwrap();
        let job_id = sequencer.submit_batch(batch);

        // Simulate completion
        sequencer.simulate_completion(job_id);

        // Get result
        let result = sequencer.get_circuit_result(100);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 500);
    }
}
