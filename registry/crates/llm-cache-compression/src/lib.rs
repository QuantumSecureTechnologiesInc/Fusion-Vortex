/// Production KV Cache Compression.
///
/// Quantizes Key/Value tensors to INT8 format to save VRAM and increase throughput.
/// Relies on fusion_llm_quantization for the core block-wise logic.
use fusion_core::types::tensor::Matrix;
use fusion_core::{FusionError, FusionResult};
use fusion_llm_quantization::int8::QuantizedMatrix;

pub struct KvCacheCompressor;

impl KvCacheCompressor {
    /// Quantizes a block of KV cache data to INT8 format.
    pub fn compress_block(
        key_block: &Matrix<f64>,
        value_block: &Matrix<f64>,
    ) -> FusionResult<(QuantizedMatrix, QuantizedMatrix)> {
        // Validation (Shapes must be compatible)
        if key_block.shape() != value_block.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "KV Compress".into(),
                lhs: key_block.shape().to_vec(),
                rhs: value_block.shape().to_vec(),
            });
        }

        // Use the production block-wise quantization logic
        let compressed_k = QuantizedMatrix::quantize(key_block)?;
        let compressed_v = QuantizedMatrix::quantize(value_block)?;

        Ok((compressed_k, compressed_v))
    }

    /// Dequantizes the block just before Attention calculation.
    pub fn decompress_block(
        k_comp: &QuantizedMatrix,
        v_comp: &QuantizedMatrix,
    ) -> FusionResult<(Matrix<f64>, Matrix<f64>)> {
        Ok((k_comp.dequantize()?, v_comp.dequantize()?))
    }
}
