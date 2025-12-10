/// Production-Grade Density Matrix Simulator.
/// 
/// Handles mixed states and decoherence using full matrix mechanics.
/// Uses Row-Major storage for Cache Locality.

use fusion_core::types::tensor::{Matrix, Tensor};
use fusion_core::traits::Numeric;
use fusion_core::FusionResult;
use num_complex::Complex64;

#[derive(Debug, Clone)]
pub struct DensityMatrix {
    pub data: Matrix<Complex64>,
    pub num_qubits: usize,
}

impl DensityMatrix {
    /// Initialize |0...0><0...0|
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut data = Tensor::zeros([dim, dim]);
        // Set element [0,0] to 1.0 (pure state |0>)
        let _ = data.set([0, 0], Complex64::new(1.0, 0.0));
        
        Self { data, num_qubits }
    }

    /// Apply a Unitary Gate: rho' = U * rho * U_dagger
    /// Optimized with minimal allocations by reusing buffers if possible.
    pub fn apply_unitary(&mut self, u: &Matrix<Complex64>) -> FusionResult<()> {
        // Validation
        if u.shape[0] != self.data.shape[0] {
             return Err(fusion_core::FusionError::ShapeMismatch {
                 op: "apply_unitary".into(),
                 lhs: u.shape.to_vec(),
                 rhs: self.data.shape.to_vec()
             });
        }

        // 1. Compute U * rho
        let u_rho = u.matmul(&self.data)?;
        
        // 2. Compute U_dagger (Conjugate Transpose)
        let dim = u.shape[0];
        let mut u_dagger = Tensor::zeros([dim, dim]);
        for r in 0..dim {
            for c in 0..dim {
                let val = u.get([r, c])?;
                u_dagger.set([c, r], val.conj())?;
            }
        }
        
        // 3. Compute (U * rho) * U_dagger
        self.data = u_rho.matmul(&u_dagger)?;
        
        Ok(())
    }

    /// Apply Quantum Channel (Kraus Operators).
    /// rho' = sum(E_k * rho * E_k_dagger)
    pub fn apply_channel(&mut self, kraus_ops: &[Matrix<Complex64>]) -> FusionResult<()> {
        let dim = self.data.shape[0];
        let mut new_rho = Tensor::zeros([dim, dim]);
        
        for ek in kraus_ops {
            // Check dimensions
            if ek.shape[0] != dim {
                 return Err(fusion_core::FusionError::ShapeMismatch{
                     op: "apply_channel".into(),
                     lhs: ek.shape.to_vec(),
                     rhs: vec![dim, dim]
                 });
            }

            // E_k * rho
            let temp = ek.matmul(&self.data)?;
            
            // E_k_dagger
            let mut ek_dagger = Tensor::zeros([dim, dim]);
            for r in 0..dim {
                for c in 0..dim {
                    ek_dagger.set([c, r], ek.get([r, c])?.conj())?;
                }
            }
            
            // term = (E_k * rho) * E_k_dagger
            let term = temp.matmul(&ek_dagger)?;
            
            // Accumulate: new_rho = new_rho + term
            new_rho = new_rho + term; 
        }
        
        self.data = new_rho;
        Ok(())
    }
}