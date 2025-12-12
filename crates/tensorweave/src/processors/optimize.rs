use crate::core::TensorData;
use crate::flow::FlowProcessor;
use anyhow::Result;
use ndarray::Array2;
use ndarray_linalg::SVD;
use tracing::{info, warn};

pub struct SvdOptimizeProcessor {
    pub keep_ratio: f64, // Percentage of energy (singular values) to keep (e.g., 0.9)
}

impl SvdOptimizeProcessor {
    pub fn new(keep_ratio: f64) -> Self {
        Self { keep_ratio }
    }
}

impl FlowProcessor for SvdOptimizeProcessor {
    fn name(&self) -> &str {
        "OPTIMIZE (SVD Compression)"
    }

    fn process_sync(&self, mut tensor: TensorData) -> Result<TensorData> {
        // Only optimize if it's a 2D matrix (SVD requirement)
        if tensor.shape.len() != 2 {
            info!("Skipping SVD: Tensor {} is not 2D.", tensor.id);
            return Ok(tensor);
        }

        let matrix = tensor.to_array2()?;
        let (rows, cols) = (matrix.nrows(), matrix.ncols());

        info!(
            "Computing SVD for tensor {} ({}x{})...",
            tensor.id, rows, cols
        );

        // 1. Compute SVD: A = U * Sigma * Vt
        match matrix.svd(true, true) {
            Ok((Some(u), sigma, Some(vt))) => {
                // 2. Determine truncation threshold
                // Calculate total energy (sum of singular values)
                let total_energy: f64 = sigma.iter().sum();
                let mut current_energy = 0.0;
                let mut k = 0; // Number of singular values to keep

                for &s in sigma.iter() {
                    current_energy += s;
                    k += 1;
                    if current_energy / total_energy >= self.keep_ratio {
                        break;
                    }
                }

                info!(
                    "Compression: Keeping top {}/{} singular values (Ratio: {:.2})",
                    k,
                    sigma.len(),
                    self.keep_ratio
                );

                // 3. Reconstruct approximated matrix
                // A_approx = U[:, :k] * Sigma[:k] * Vt[:k, :]

                // Construct diagonal Sigma matrix for top k
                let mut sigma_k = Array2::<f64>::zeros((k, k));
                for i in 0..k {
                    sigma_k[[i, i]] = sigma[i];
                }

                // Slice U and Vt using ndarray slicing
                let u_k = u.slice(ndarray::s![.., ..k]);
                let vt_k = vt.slice(ndarray::s![..k, ..]);

                // Perform matrix multiplication: (U_k * Sigma_k) * Vt_k
                let u_sigma = u_k.dot(&sigma_k);
                let approx_matrix = u_sigma.dot(&vt_k);

                // 4. Update tensor data
                tensor.update_from_ndarray(approx_matrix.into_dyn());
                tensor
                    .metadata
                    .insert("optimization".to_string(), "SVD_Truncated".to_string());
                tensor
                    .metadata
                    .insert("kept_singular_values".to_string(), k.to_string());
            }
            Err(e) => {
                warn!(
                    "SVD Computation failed for {}: {}. Passing through.",
                    tensor.id, e
                );
            }
            _ => {
                warn!("SVD failed to produce U or Vt components.");
            }
        }

        Ok(tensor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svd_optimizer_2d_tensor() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let tensor = TensorData::new("test", vec![2, 3], data);
        let processor = SvdOptimizeProcessor::new(0.9);
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert!(optimized.metadata.contains_key("optimization"));
    }

    #[test]
    fn test_svd_optimizer_skip_1d() {
        let tensor = TensorData::new("test", vec![6], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let processor = SvdOptimizeProcessor::new(0.9);
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let result_tensor = result.unwrap();
        // Should not have optimization metadata since it was skipped
        assert!(!result_tensor.metadata.contains_key("optimization"));
    }
}
