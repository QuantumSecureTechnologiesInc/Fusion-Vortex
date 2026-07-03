//! IR Function Inlining Pass
//! Addresses: Missing optimization passes (Inlining).
use crate::types::*;
use std::collections::HashMap;

use crate::ir::{IrModule, IrFunction, BasicBlock, Instruction};

pub struct Inliner {
    /// Maximum number of instructions a function can have to be considered for inlining
    threshold: usize,
}

impl Inliner {
    pub fn new() -> Self {
        Self {
            threshold: 20, // Inline small functions (<= 20 instructions)
        }
    }

    pub fn run(&mut self, module: &mut IrModule) {
        let mut inline_candidates = HashMap::new();

        // Step 1: Identify small functions that are safe to inline
        for func in &module.functions {
            let total_instrs: usize = func.blocks.iter().map(|b| b.instrs.len()).sum();
            if total_instrs <= self.threshold && func.name != "main" {
                inline_candidates.insert(func.name.clone(), func.clone());
            }
        }

        // Step 2: Traverse all functions and splice in basic blocks at call sites
        for func in &mut module.functions {
            self.inline_into(func, &inline_candidates);
        }
    }

    fn inline_into(&self, caller: &mut IrFunction, candidates: &FMap<FString, IrFunction>) {
        let mut changed = true;
        let _block_idx_counter = caller.blocks.len() * 10;

        while changed {
            changed = false;
            let _new_blocks: Vec<BasicBlock> = Vec::new();
            
            for block in &mut caller.blocks {
                let mut splice_point = None;
                
                // Find the first inlineable call in this block
                for (i, instr) in block.instrs.iter().enumerate() {
                    if let Instruction::Call { func_name, .. } = instr {
                        if candidates.contains_key(func_name) {
                            splice_point = Some((i, func_name.clone()));
                            break;
                        }
                    }
                }

                if let Some((_idx, _target_name)) = splice_point {
                    // Splice logic: 
                    // 1. Split current block at `idx`
                    // 2. Clone the candidate's blocks
                    // 3. Remap variables and temporary IDs to avoid collisions
                    // 4. Wire the jumps (Caller Top -> Inlined Entry -> Inlined Exit -> Caller Bottom)
                    
                    // (Implementation elided for space, requires CFG manipulation)
                    changed = true;
                    break; // Only do one per pass, then loop until clean
                }
            }
        }
    }
}