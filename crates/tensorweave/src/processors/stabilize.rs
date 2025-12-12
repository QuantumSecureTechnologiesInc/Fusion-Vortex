use anyhow::Result;
use tracing::{info, warn};

use crate::{core::TensorData, flow::FlowProcessor};

/// StabilizeProcessor removes numerical instabilities from tensor data
/// by replacing NaN and Inf values with safe defaults.
pub struct StabilizeProcessor;

impl FlowProcessor for StabilizeProcessor {
    fn name(&self) -> &str {
        "STABILIZE (Numerical Stability)"
    }

    fn process_sync(&self, mut tensor: TensorData) -> Result<TensorData> {
        info!("Stabilizing tensor {}", tensor.id);

        let mut array = tensor.to_ndarray()?;

        // Count problematic values before fixing
        let nan_count = array.iter().filter(|&&x| x.is_nan()).count();
        let inf_count = array.iter().filter(|&&x| x.is_infinite()).count();

        if nan_count > 0 || inf_count > 0 {
            warn!(
                "Tensor {} has {} NaN values and {} Inf values - stabilizing",
                tensor.id, nan_count, inf_count
            );
        }

        // Replace NaN values with 0.0
        // Replace +Inf with large positive value
        // Replace -Inf with large negative value
        array.mapv_inplace(|x| {
            if x.is_nan() {
                0.0
            } else if x.is_infinite() {
                if x.is_sign_positive() {
                    1e10 // Large positive value
                } else {
                    -1e10 // Large negative value
                }
            } else {
                x
            }
        });

        // Update tensor with stabilized data
        tensor.update_from_ndarray(array);

        // Add metadata about stabilization
        if nan_count > 0 || inf_count > 0 {
            tensor.metadata.insert(
                "stabilized".to_string(),
                format!("nan:{},inf:{}", nan_count, inf_count),
            );
        }

        info!("Tensor {} stabilized successfully", tensor.id);
        Ok(tensor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabilize_processor_clean_tensor() {
        let tensor = TensorData::new("test", vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
        let processor = StabilizeProcessor;
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let stabilized = result.unwrap();
        assert_eq!(stabilized.data, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_stabilize_processor_nan_values() {
        let tensor = TensorData::new("test", vec![3], vec![1.0, f64::NAN, 3.0]);
        let processor = StabilizeProcessor;
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let stabilized = result.unwrap();
        assert_eq!(stabilized.data, vec![1.0, 0.0, 3.0]);
        assert!(stabilized.metadata.contains_key("stabilized"));
    }

    #[test]
    fn test_stabilize_processor_inf_values() {
        let tensor = TensorData::new(
            "test",
            vec![4],
            vec![1.0, f64::INFINITY, -f64::INFINITY, 4.0],
        );
        let processor = StabilizeProcessor;
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let stabilized = result.unwrap();
        assert_eq!(stabilized.data[0], 1.0);
        assert_eq!(stabilized.data[1], 1e10);
        assert_eq!(stabilized.data[2], -1e10);
        assert_eq!(stabilized.data[3], 4.0);
    }

    #[test]
    fn test_stabilize_processor_mixed_issues() {
        let tensor = TensorData::new(
            "test",
            vec![5],
            vec![f64::NAN, f64::INFINITY, 0.0, -f64::INFINITY, f64::NAN],
        );
        let processor = StabilizeProcessor;
        let result = processor.process_sync(tensor);
        assert!(result.is_ok());
        let stabilized = result.unwrap();
        // All values should be finite
        assert!(stabilized.data.iter().all(|&x| x.is_finite()));
    }
}
