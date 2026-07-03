/// Production GQA Kernel.
///
/// Implements the Key/Value repetition logic for Grouped Query Attention.
/// Essential for Mistral/Llama efficiency.
use fusion_core::types::tensor::Matrix;
use fusion_core::FusionResult;

pub struct GqaKernel;

impl GqaKernel {
    /// Repeats Key/Value states to match the number of Query heads.
    /// K and V are smaller than Q heads due to sharing.
    /// Input K/V: [Batch, SeqLen, Num_KV_Heads, HeadDim] -> Simplified to [SeqLen, KV_Dim]
    /// Output K/V: [Batch, SeqLen, Num_Q_Heads, HeadDim] -> Simplified to [SeqLen, Q_Dim]
    pub fn repeat_kv(
        kv_tensor: &Matrix<f64>,
        num_q_heads: usize,
        num_kv_heads: usize,
    ) -> FusionResult<Matrix<f64>> {
        let (seq_len, kv_dim) = (kv_tensor.shape()[0], kv_tensor.shape()[1]);
        let head_dim = kv_dim / num_kv_heads;
        let repetition_factor = num_q_heads / num_kv_heads;

        if repetition_factor == 1 {
            return Ok(kv_tensor.clone()); // Multi-Query Attention (MQA) or MHA
        }

        let new_dim = num_q_heads * head_dim;
        let mut new_data = Vec::with_capacity(seq_len * new_dim);

        // Perform repetition loop
        for t in 0..seq_len {
            for kv_head in 0..num_kv_heads {
                // Determine the start/end indices for the current KV head
                let kv_head_start = kv_head * head_dim;
                let kv_head_end = kv_head_start + head_dim;

                // Repeat the data 'repetition_factor' times
                for _ in 0..repetition_factor {
                    for i in kv_head_start..kv_head_end {
                        let val = *kv_tensor.get(&[t, i] as &[usize]).ok_or(
                            fusion_core::FusionError::Generic("Index out of bounds".into()),
                        )?;
                        new_data.push(val);
                    }
                }
            }
        }

        Matrix::new(new_data, [seq_len, new_dim])
    }
}
