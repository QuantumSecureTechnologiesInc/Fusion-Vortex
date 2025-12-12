use crate::core::TensorData;
use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;
use tracing::{error, info};

pub trait FlowProcessor: Send + Sync {
    fn name(&self) -> &str;
    fn process_sync(&self, tensor: TensorData) -> Result<TensorData>;
}

pub struct TensorWeaveEngine {
    processors: Arc<Vec<Box<dyn FlowProcessor>>>,
}

impl TensorWeaveEngine {
    pub fn new(_timeout_sec: u64) -> Self {
        Self {
            processors: Arc::new(Vec::new()),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn FlowProcessor>) {
        if let Some(procs) = Arc::get_mut(&mut self.processors) {
            procs.push(processor);
        }
    }

    pub fn run_batch_sync(&self, batch: Vec<TensorData>) -> Vec<Result<TensorData>> {
        info!("Dispatching batch processing with Rayon parallelism");

        let shared_engine = self.processors.clone();

        batch
            .into_par_iter()
            .map(|data| {
                let mut current_data = data;
                for p in shared_engine.iter() {
                    match p.process_sync(current_data) {
                        Ok(processed) => current_data = processed,
                        Err(e) => {
                            error!("Tensor processing failed: {}", e);
                            return Err(e);
                        }
                    }
                }
                Ok(current_data)
            })
            .collect()
    }
}
