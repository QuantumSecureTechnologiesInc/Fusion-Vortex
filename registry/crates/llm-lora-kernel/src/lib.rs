use fusion_core::types::tensor::Matrix;
use fusion_core::FusionResult;

/// Optimized LoRA Kernel.
///
/// Merges base model weights with Low-Rank Adapters on the fly.
/// Critical for efficient fine-tuning and inference.
///
/// LoRA (Low-Rank Adaptation) formula:
/// W' = W + B @ A * scaling
/// where W is the base weight, A and B are low-rank matrices (r << d)

pub fn apply_lora_kernel(
    base_weight: &Matrix<f32>,
    lora_a: &Matrix<f32>,
    lora_b: &Matrix<f32>,
    scaling: f64,
) -> FusionResult<Matrix<f32>> {
    // Get shapes
    let base_shape = base_weight.shape();
    let a_shape = lora_a.shape();
    let b_shape = lora_b.shape();

    // Validate dimensions: B @ A should match base_weight dimensions
    // A: [r, k], B: [d, r] => B @ A: [d, k]
    if b_shape[1] != a_shape[0] {
        return Err(fusion_core::FusionError::CompilationError(format!(
            "LoRA dimension mismatch: B shape {:?} incompatible with A shape {:?}",
            b_shape, a_shape
        )));
    }

    if base_shape[0] != b_shape[0] || base_shape[1] != a_shape[1] {
        return Err(fusion_core::FusionError::CompilationError(format!(
            "LoRA output shape {:?} doesn't match base weight shape {:?}",
            [b_shape[0], a_shape[1]],
            base_shape
        )));
    }

    // Compute B @ A
    let ba_product = matmul_f32(lora_b, lora_a)?;

    // Scale the product
    let scaled_ba = scale_matrix(&ba_product, scaling as f32)?;

    // Add to base weight: W' = W + scaled(B @ A)
    add_matrices(base_weight, &scaled_ba)
}

/// Matrix multiplication for f32 matrices
fn matmul_f32(a: &Matrix<f32>, b: &Matrix<f32>) -> FusionResult<Matrix<f32>> {
    let a_shape = a.shape();
    let b_shape = b.shape();

    if a_shape[1] != b_shape[0] {
        return Err(fusion_core::FusionError::CompilationError(format!(
            "Matrix multiplication dimension mismatch: {:?} @ {:?}",
            a_shape, b_shape
        )));
    }

    let m = a_shape[0];
    let n = b_shape[1];
    let k = a_shape[1];

    let a_data = a.data();
    let b_data = b.data();
    let mut result = vec![0.0f32; m * n];

    // Standard matrix multiplication
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0f32;
            for p in 0..k {
                sum += a_data[i * k + p] * b_data[p * n + j];
            }
            result[i * n + j] = sum;
        }
    }

    Matrix::from_slice(&result, [m, n])
}

/// Scale matrix by a scalar
fn scale_matrix(mat: &Matrix<f32>, scalar: f32) -> FusionResult<Matrix<f32>> {
    let data = mat.data();
    let scaled: Vec<f32> = data.iter().map(|&x| x * scalar).collect();
    Matrix::from_slice(&scaled, mat.shape())
}

/// Add two matrices element-wise
fn add_matrices(a: &Matrix<f32>, b: &Matrix<f32>) -> FusionResult<Matrix<f32>> {
    let a_shape = a.shape();
    let b_shape = b.shape();

    if a_shape != b_shape {
        return Err(fusion_core::FusionError::CompilationError(format!(
            "Matrix addition shape mismatch: {:?} + {:?}",
            a_shape, b_shape
        )));
    }

    let a_data = a.data();
    let b_data = b.data();
    let result: Vec<f32> = a_data
        .iter()
        .zip(b_data.iter())
        .map(|(&x, &y)| x + y)
        .collect();

    Matrix::from_slice(&result, a_shape)
}
