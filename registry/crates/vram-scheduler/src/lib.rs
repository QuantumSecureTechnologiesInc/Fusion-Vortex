/// Production GPU VRAM Scheduler.
/// 
/// Manages virtual and physical memory pages/blocks for efficient LLM serving.
/// Mimics the architecture of vLLM's scheduler.

use fusion_llm_inference_engine::kv_cache::PagedAttentionManager;
use fusion_core::{FusionResult, FusionError};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SequenceRequest {
    pub seq_id: u64,
    pub token_len: usize,
    pub max_tokens_to_generate: usize,
}

pub struct VramScheduler {
    // Shared reference to the cache manager
    cache_manager: Arc<Mutex<PagedAttentionManager>>,
    // Total VRAM pages/blocks available
    total_blocks: usize,
}

impl VramScheduler {
    pub fn new(total_blocks: usize, num_heads: usize, head_dim: usize) -> Self {
        Self {
            total_blocks,
            cache_manager: Arc::new(Mutex::new(PagedAttentionManager::new(
                total_blocks, num_heads, head_dim
            ))),
        }
    }

    /// Attempts to allocate blocks for a new sequence.
    /// Returns true if successful, false if insufficient memory.
    pub async fn try_schedule(&self, request: &SequenceRequest) -> FusionResult<bool> {
        let mut manager = self.cache_manager.lock().await;

        let required_blocks = (request.token_len + request.max_tokens_to_generate + PagedAttentionManager::BLOCK_SIZE - 1) 
            / PagedAttentionManager::BLOCK_SIZE;

        if manager.free_blocks.len() < required_blocks {
            // Log VRAM contention
            return Ok(false); 
        }

        // Allocate block table entries for the sequence
        for _ in 0..required_blocks {
            manager.allocate_block(request.seq_id)?; // Allocates from free list
        }

        Ok(true)
    }

    /// Frees all blocks associated with a completed sequence.
    pub async fn free_sequence(&self, seq_id: u64) {
        let mut manager = self.cache_manager.lock().await;
        if let Some(blocks) = manager.block_tables.remove(&seq_id) {
            manager.free_blocks.extend(blocks);
        }
    }
}