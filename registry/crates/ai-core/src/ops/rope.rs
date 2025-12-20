/// Production Rotary Position Embedding (RoPE) Implementation
///
/// RoPE encodes absolute position information with rotation matrices and naturally
/// incorporates relative position dependency in self-attention formulation.
///
/// Reference: "RoFormer: Enhanced Transformer with Rotary Position Embedding" (Su et al., 2021)
use fusion_core::{FusionError, FusionResult};
use ndarray::{Array2, ArrayD};
use std::f64::consts::PI;

pub struct RotaryEmbedding {
    dim: usize,
    max_seq_len: usize,
    /// Precomputed inverse frequencies for efficiency
    inv_freq: Vec<f64>,
    /// Cached cos and sin values for positions
    cos_cached: Option<Array2<f64>>,
    sin_cached: Option<Array2<f64>>,
}

impl RotaryEmbedding {
    /// Create a new RoPE instance
    ///
    /// # Arguments
    /// * `dim` - Dimension of the embedding (must be even)
    /// * `max_seq_len` - Maximum sequence length to precompute
    /// * `base` - Base for the geometric progression (default: 10000.0)
    pub fn new(dim: usize, max_seq_len: usize) -> FusionResult<Self> {
        Self::with_base(dim, max_seq_len, 10000.0)
    }

    pub fn with_base(dim: usize, max_seq_len: usize, base: f64) -> FusionResult<Self> {
        if dim % 2 != 0 {
            return Err(FusionError::InvalidDimension(format!(
                "RoPE dimension must be even, got {}",
                dim
            )));
        }

        // Compute inverse frequencies: 1 / (base^(2i/dim)) for i in 0..dim/2
        let inv_freq: Vec<f64> = (0..dim / 2)
            .map(|i| {
                let exponent = (2 * i) as f64 / dim as f64;
                1.0 / base.powf(exponent)
            })
            .collect();

        let mut rope = Self {
            dim,
            max_seq_len,
            inv_freq,
            cos_cached: None,
            sin_cached: None,
        };

        // Precompute cos and sin for all positions
        rope.precompute_freqs_cis(max_seq_len)?;

        Ok(rope)
    }

    /// Precompute cos and sin values for all positions up to seq_len
    fn precompute_freqs_cis(&mut self, seq_len: usize) -> FusionResult<()> {
        let half_dim = self.dim / 2;

        // Create position indices: [0, 1, 2, ..., seq_len-1]
        let positions: Vec<f64> = (0..seq_len).map(|i| i as f64).collect();

        // Compute outer product of positions and inverse frequencies
        // Shape: [seq_len, half_dim]
        let mut freqs = Array2::zeros((seq_len, half_dim));
        for (pos_idx, &pos) in positions.iter().enumerate() {
            for (freq_idx, &inv_f) in self.inv_freq.iter().enumerate() {
                freqs[[pos_idx, freq_idx]] = pos * inv_f;
            }
        }

        // Compute cos and sin
        let cos_vals = freqs.mapv(|x| x.cos());
        let sin_vals = freqs.mapv(|x| x.sin());

        self.cos_cached = Some(cos_vals);
        self.sin_cached = Some(sin_vals);

        Ok(())
    }

    /// Apply rotary embedding to query or key tensors
    ///
    /// # Arguments
    /// * `x` - Input tensor of shape [seq_len, dim] or [batch, seq_len, dim]
    /// * `position_offset` - Starting position (for incremental decoding)
    ///
    /// # Returns
    /// Rotated tensor with the same shape as input
    pub fn apply_rotation(
        &self,
        x: &ArrayD<f64>,
        position_offset: usize,
    ) -> FusionResult<ArrayD<f64>> {
        let shape = x.shape();

        // Handle both [seq_len, dim] and [batch, seq_len, dim]
        let (seq_len, dim) = if shape.len() == 2 {
            (shape[0], shape[1])
        } else if shape.len() == 3 {
            (shape[1], shape[2])
        } else {
            return Err(FusionError::ShapeError(format!(
                "Expected 2D or 3D tensor, got shape {:?}",
                shape
            )));
        };

        if dim != self.dim {
            return Err(FusionError::ShapeMismatch {
                op: "RoPE".to_string(),
                lhs: vec![dim],
                rhs: vec![self.dim],
            });
        }

        // Extend cache if needed
        if position_offset + seq_len > self.max_seq_len {
            return Err(FusionError::Generic(format!(
                "Position {} + seq_len {} exceeds max_seq_len {}",
                position_offset, seq_len, self.max_seq_len
            )));
        }

        let cos_cached = self
            .cos_cached
            .as_ref()
            .ok_or_else(|| FusionError::Generic("RoPE cache not initialized".to_string()))?;
        let sin_cached = self
            .sin_cached
            .as_ref()
            .ok_or_else(|| FusionError::Generic("RoPE cache not initialized".to_string()))?;

        // Extract relevant cos/sin values for this sequence
        let cos_vals =
            cos_cached.slice(ndarray::s![position_offset..position_offset + seq_len, ..]);
        let sin_vals =
            sin_cached.slice(ndarray::s![position_offset..position_offset + seq_len, ..]);

        // Apply rotation
        let mut result = x.clone();
        let half_dim = self.dim / 2;

        // For 2D input [seq_len, dim]
        if shape.len() == 2 {
            for i in 0..seq_len {
                for j in 0..half_dim {
                    let x1 = x[[i, j]];
                    let x2 = x[[i, j + half_dim]];
                    let cos = cos_vals[[i, j]];
                    let sin = sin_vals[[i, j]];

                    // Rotation: [x1, x2] -> [x1*cos - x2*sin, x1*sin + x2*cos]
                    result[[i, j]] = x1 * cos - x2 * sin;
                    result[[i, j + half_dim]] = x1 * sin + x2 * cos;
                }
            }
        }
        // For 3D input [batch, seq_len, dim]
        else if shape.len() == 3 {
            let batch_size = shape[0];
            for b in 0..batch_size {
                for i in 0..seq_len {
                    for j in 0..half_dim {
                        let x1 = x[[b, i, j]];
                        let x2 = x[[b, i, j + half_dim]];
                        let cos = cos_vals[[i, j]];
                        let sin = sin_vals[[i, j]];

                        result[[b, i, j]] = x1 * cos - x2 * sin;
                        result[[b, i, j + half_dim]] = x1 * sin + x2 * cos;
                    }
                }
            }
        }

        Ok(result)
    }

    /// Get the dimension of this RoPE instance
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Get the maximum sequence length
    pub fn max_seq_len(&self) -> usize {
        self.max_seq_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;

    #[test]
    fn test_rope_creation() {
        let rope = RotaryEmbedding::new(64, 2048).unwrap();
        assert_eq!(rope.dim(), 64);
        assert_eq!(rope.max_seq_len(), 2048);
    }

    #[test]
    fn test_rope_odd_dimension_fails() {
        let result = RotaryEmbedding::new(63, 2048);
        assert!(result.is_err());
    }

    #[test]
    fn test_rope_rotation_2d() {
        let rope = RotaryEmbedding::new(4, 10).unwrap();
        let input = Array::from_shape_vec((2, 4), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .unwrap()
            .into_dyn();

        let result = rope.apply_rotation(&input, 0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().shape(), &[2, 4]);
    }

    #[test]
    fn test_rope_rotation_3d() {
        let rope = RotaryEmbedding::new(4, 10).unwrap();
        let input = Array::from_shape_vec((2, 3, 4), (0..24).map(|x| x as f64).collect())
            .unwrap()
            .into_dyn();

        let result = rope.apply_rotation(&input, 0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().shape(), &[2, 3, 4]);
    }
}
