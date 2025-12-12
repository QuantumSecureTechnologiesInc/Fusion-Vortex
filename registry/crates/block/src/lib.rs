/// Production Block-wise Quantization.
///
/// Quantizes weights in small blocks (e.g., 32 or 128) to maintain higher fidelity
/// than per-tensor scaling. This is the foundation for formats like GGUF/GPTQ.
use fusion_core::types::tensor::{Matrix, Tensor};
use fusion_core::{FusionError, FusionResult};
use std::sync::Arc;

pub const BLOCK_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct BlockQuantizedMatrix {
    pub quantized_data: Vec<i8>, // Flattened blocks
    pub scales: Vec<f64>,        // One scale per block
    pub zeros: Vec<f64>,         // One zero-point per block (optional, used in asymmetric)
    pub original_shape: [usize; 2],
}

impl BlockQuantizedMatrix {
    /// Quantize a Dense Matrix into blocks.
    pub fn quantize(_matrix: &Matrix<f64>) -> FusionResult<Self> {
        // Implementation disabled due to API mismatch
        Err(FusionError::Generic("Not implemented".to_string()))
    }

    fn quantize_block(block: &[f64]) -> (Vec<i8>, f64, f64) {
        let max_val = block.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_val = block.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        // Asymmetric Quantization: x_q = (x - z) / s
        // s = (max - min) / (2^bits - 1) -> for i8 [-128, 127], range is 255
        let scale = (max_val - min_val) / 255.0;
        let zero_point = min_val; // Simplified mapping 0 -> -128 offset logic handled in dequant

        // Avoid division by zero for constant blocks
        let scale = if scale == 0.0 { 1.0 } else { scale };

        let q_vals: Vec<i8> = block
            .iter()
            .map(|&x| {
                let scaled = (x - zero_point) / scale;
                // Map 0..255 to -128..127
                let clamped = scaled.round().clamp(0.0, 255.0);
                (clamped as i16 - 128) as i8
            })
            .collect();

        (q_vals, scale, zero_point)
    }

    pub fn dequantize(&self) -> FusionResult<Matrix<f64>> {
        let num_blocks = self.scales.len();
        let mut data = Vec::with_capacity(num_blocks * BLOCK_SIZE);

        for b in 0..num_blocks {
            let scale = self.scales[b];
            let zero = self.zeros[b];
            let start = b * BLOCK_SIZE;
            let end = start + BLOCK_SIZE;

            for i in start..end {
                let q = self.quantized_data[i];
                // x = (q + 128) * s + z
                let val = ((q as i16 + 128) as f64) * scale + zero;
                data.push(val);
            }
        }

        Tensor::new(data, self.original_shape)
    }
}
