/// Attention Mask Generation.
///
/// Generates masks for Causal, Padding, and Sliding Window Attention (SWA).
use fusion_core::types::tensor::Matrix;
use fusion_core::FusionResult;

pub struct AttentionMaskGenerator;

impl AttentionMaskGenerator {
    /// Generate a Sliding Window Attention mask.
    /// Limits attention to a fixed window (W) around each token (Mistral/Longformer).
    pub fn generate_sliding_window_mask(
        seq_len: usize,
        window_size: usize,
    ) -> FusionResult<Matrix<f64>> {
        let mut mask = Matrix::new(vec![0.0; seq_len * seq_len], [seq_len, seq_len])?;

        for r in 0..seq_len {
            for c in 0..seq_len {
                // Apply Causal mask (r >= c) AND Sliding Window (c >= r - window_size)
                if c as i32 >= (r as i32 - window_size as i32).max(0) && c <= r {
                    // 0.0 means allow attention (no masking)
                    mask.set([r, c], 0.0)?;
                } else {
                    // -INF/1.0 means mask out
                    mask.set([r, c], 1.0)?;
                }
            }
        }
        Ok(mask)
    }
}
