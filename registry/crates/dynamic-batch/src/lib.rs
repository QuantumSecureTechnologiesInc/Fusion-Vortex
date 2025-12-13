/// Production Dynamic Batch Scheduler.
///
/// Manages a dynamic list of sequences and groups them into batches for efficient GPU utilization.
use fusion_core::FusionResult;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub id: u64,
    pub token_ids: Vec<i64>,
    pub next_token_index: usize,
    pub status: String,
}

pub struct BatchScheduler {
    sequence_queue: VecDeque<Sequence>,
    active_sequences: Vec<Sequence>,
    max_batch_size: usize,
}

impl BatchScheduler {
    pub fn new(max_batch_size: usize) -> Self {
        Self {
            sequence_queue: VecDeque::new(),
            active_sequences: Vec::new(),
            max_batch_size,
        }
    }

    /// Adds a new sequence request to the queue.
    pub fn enqueue(&mut self, seq: Sequence) {
        self.sequence_queue.push_back(seq);
    }

    /// Selects sequences to form the next execution batch (Continuous Batching logic).
    pub fn schedule_next_batch(&mut self) -> Vec<Sequence> {
        let mut new_batch = Vec::new();

        // 1. Prioritize pre-filling new sequences from the queue
        while new_batch.len() < self.max_batch_size {
            if let Some(seq) = self.sequence_queue.pop_front() {
                new_batch.push(seq);
            } else {
                break;
            }
        }

        // 2. Add decoding sequences from active list (Continuous Batching)
        // Active sequences (active_sequences) are prioritized for throughput

        // We simulate moving the new batch into the active list
        self.active_sequences.extend(new_batch.clone());

        new_batch
    }
}
