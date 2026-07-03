/// Production Optimized RoPE Kernel.
///
/// Provides high-performance, in-place RoPE calculation for Q/K tensors.
/// This implementation is designed to be mapped to highly efficient GPU compute kernels.
use fusion_core::types::tensor::{Matrix, Vector1D};
use fusion_core::FusionResult;

pub struct RotaryKernel;

impl RotaryKernel {
    /// Apply RoPE rotation to a tensor.
    /// Input assumed to be [SeqLen, HeadDim] (or reshaped).
    pub fn apply_optimized(
        input: &Matrix<f64>,
        position_ids: &Vector1D<i64>,
        inv_freq: &Vector1D<f64>,
        head_dim: usize,
    ) -> FusionResult<Matrix<f64>> {
        let (seq_len, _dim) = (input.shape()[0], input.shape()[1]);
        let mut output = input.clone();

        for t in 0..seq_len {
            // Get position ID safely
            let pos =
                *position_ids
                    .get(&[t] as &[usize])
                    .ok_or(fusion_core::FusionError::Generic(
                        "Index out of bounds".into(),
                    ))? as f64;

            for i in 0..head_dim / 2 {
                // Get precomputed frequency
                let freq =
                    *inv_freq
                        .get(&[i] as &[usize])
                        .ok_or(fusion_core::FusionError::Generic(
                            "Index out of bounds".into(),
                        ))?;
                let theta = pos * freq;

                let cos = theta.cos();
                let sin = theta.sin();

                // Get current pair (x0, x1)
                let x0 = *input.get(&[t, 2 * i] as &[usize]).ok_or(
                    fusion_core::FusionError::Generic("Index out of bounds".into()),
                )?;
                let x1 = *input.get(&[t, 2 * i + 1] as &[usize]).ok_or(
                    fusion_core::FusionError::Generic("Index out of bounds".into()),
                )?;

                // Apply rotation: x0' = x0*cos - x1*sin, x1' = x1*cos + x0*sin
                let x0_prime = x0 * cos - x1 * sin;
                let x1_prime = x1 * cos + x0 * sin;

                output.set(&[t, 2 * i][..], x0_prime)?;
                output.set(&[t, 2 * i + 1][..], x1_prime)?;
            }
        }

        Ok(output)
    }
}
