use fusion_core::traits::Numeric;
/// Production Sparse Matrix Operators.
///
/// Implements Sparse Matrix multiplication using the CSR format.
use fusion_core::types::tensor::{Matrix, Tensor};
use fusion_core::FusionError;
use fusion_core::FusionResult;

pub struct CsrMatrix<T> {
    pub shape: (usize, usize),
    pub row_ptr: Vec<usize>,
    pub col_indices: Vec<usize>,
    pub values: Vec<T>,
}

impl<
        T: Numeric + Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + num_traits::Zero,
    > CsrMatrix<T>
{
    /// Sparse-Dense Matrix Multiplication: C = A_sparse * B_dense
    pub fn matmul_dense(&self, b_dense: &Matrix<T>) -> FusionResult<Matrix<T>> {
        let (rows_a, cols_a) = self.shape;
        let (rows_b, cols_b) = (b_dense.shape()[0], b_dense.shape()[1]);

        if cols_a != rows_b {
            return Err(FusionError::ShapeMismatch {
                op: "Sparse-Dense MatMul".into(),
                lhs: vec![self.shape.0, self.shape.1],
                rhs: b_dense.shape().to_vec(),
            });
        }

        let mut output = Tensor::zeros([rows_a, cols_b]);

        for r in 0..rows_a {
            let row_start = self.row_ptr[r];
            let row_end = self.row_ptr[r + 1];

            for ptr in row_start..row_end {
                let col_a = self.col_indices[ptr];
                let val_a = self.values[ptr];

                // C[r, c] += A[r, col_a] * B[col_a, c]
                for c in 0..cols_b {
                    let val_b = *b_dense
                        .get([col_a, c])
                        .ok_or(FusionError::IndexOutOfBounds(format!(
                            "Index [{}, {}]",
                            col_a, c
                        )))?;
                    let current_c =
                        *output
                            .get([r, c])
                            .ok_or(FusionError::IndexOutOfBounds(format!(
                                "Index [{}, {}]",
                                r, c
                            )))?;
                    output.set([r, c], current_c + (val_a * val_b))?;
                }
            }
        }

        Ok(output)
    }
}
