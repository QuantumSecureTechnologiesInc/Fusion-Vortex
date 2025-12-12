/// Production-Grade Paged KV Cache.
/// 
/// Manages memory in fixed-size blocks to eliminate fragmentation and allow dynamic sequence growth.
/// Thread-safe and ready for parallel inference.

use fusion_core::types::tensor::{Tensor, Matrix};
use fusion_core::FusionResult;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

const BLOCK_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct KVCacheBlock {
    pub key_block: Matrix<f64>,   // [num_heads * head_dim, block_size] optimized layout
    pub value_block: Matrix<f64>, 
    pub num_filled: usize,
}

impl KVCacheBlock {
    pub fn new(num_heads: usize, head_dim: usize) -> Self {
        // Pre-allocate memory
        // Using f64 for core compatibility, though f16/f32 is common in prod LLMs
        // Flattened dimension: num_heads * head_dim
        let dim = num_heads * head_dim;
        Self {
            key_block: Tensor::zeros([dim, BLOCK_SIZE]), 
            value_block: Tensor::zeros([dim, BLOCK_SIZE]),
            num_filled: 0,
        }
    }
}

pub struct PagedAttentionManager {
    // Pool of physical blocks
    physical_blocks: Vec<Arc<RwLock<KVCacheBlock>>>,
    // Mapping: Sequence ID -> List of Physical Block Indices
    block_tables: HashMap<u64, Vec<usize>>,
    free_blocks: Vec<usize>,
    
    num_heads: usize,
    head_dim: usize,
}

impl PagedAttentionManager {
    pub fn new(num_blocks: usize, num_heads: usize, head_dim: usize) -> Self {
        let mut physical_blocks = Vec::with_capacity(num_blocks);
        let mut free_blocks = Vec::with_capacity(num_blocks);
        
        for i in 0..num_blocks {
            physical_blocks.push(Arc::new(RwLock::new(KVCacheBlock::new(num_heads, head_dim))));
            free_blocks.push(i);
        }

        Self {
            physical_blocks,
            block_tables: HashMap::new(),
            free_blocks,
            num_heads,
            head_dim,
        }
    }

    /// Allocate a new block for a sequence.
    pub fn allocate_block(&mut self, seq_id: u64) -> FusionResult<()> {
        if let Some(block_idx) = self.free_blocks.pop() {
            self.block_tables.entry(seq_id).or_default().push(block_idx);
            
            // Reset the block state
            let block = &self.physical_blocks[block_idx];
            let mut writer = block.write().map_err(|_| fusion_core::FusionError::CompilationError("Lock poisoned".into()))?;
            writer.num_filled = 0;
            
            Ok(())
        } else {
            Err(fusion_core::FusionError::CompilationError("OOM: No free KV blocks".into()))
        }
    }

    /// Append a token's KV states to the cache.
    pub fn append_token(&mut self, seq_id: u64, key: Matrix<f64>, value: Matrix<f64>) -> FusionResult<()> {
        let table = self.block_tables.get_mut(&seq_id)
            .ok_or(fusion_core::FusionError::UnknownVariable(format!("Seq {}", seq_id)))?;
            
        // Check if last block is full or needs allocation
        let last_block_idx_opt = table.last().copied();
        
        let needs_new_block = match last_block_idx_opt {
            Some(idx) => {
                let block = self.physical_blocks[idx].read().unwrap();
                block.num_filled == BLOCK_SIZE
            },
            None => true,
        };

        if needs_new_block {
            self.allocate_block(seq_id)?;
        }

        // Get the (possibly new) last block
        let table = self.block_tables.get(&seq_id).unwrap();
        let block_idx = *table.last().unwrap();
        let block_ref = &self.physical_blocks[block_idx];
        
        let mut writer = block_ref.write().unwrap();
        let pos = writer.num_filled;
        
        // Copy data into block at position 'pos'
        // Since we are using Matrix<f64>, we copy column-wise or row-wise depending on layout.
        // Assuming [features, block_size], we copy into column `pos`.
        
        let features = self.num_heads * self.head_dim;
        if key.shape[0] * key.shape[1] != features {
             return Err(fusion_core::FusionError::ShapeMismatch { 
                 op: "KV Append".into(), 
                 lhs: key.shape.to_vec(), 
                 rhs: vec![features] 
             });
        }

        // Manual copy since tensor slice ops might not be fully exposed in Phase 2 core
        // Flattened copy
        for i in 0..features {
            let val = key.data[i]; 
            // Index in block: row=i, col=pos -> index = i * BLOCK_SIZE + pos (assuming RowMajor)
            // But we defined shape as [dim, BLOCK_SIZE], so strides[0] = BLOCK_SIZE
            writer.key_block.set([i, pos], val)?;
            
            let v_val = value.data[i];
            writer.value_block.set([i, pos], v_val)?;
        }
        
        writer.num_filled += 1;
        Ok(())
    }
}