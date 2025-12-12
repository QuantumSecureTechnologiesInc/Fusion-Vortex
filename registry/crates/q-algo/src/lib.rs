/// Production Quantum Fourier Transform (QFT).
/// 
/// Generates the QFT circuit for an arbitrary number of qubits.
/// Uses Phase Gates R_k = diag(1, e^{2pi i / 2^k}).

use fusion_core::types::quantum::{QuantumCircuit, QuantumGate};
use fusion_core::FusionResult;
use fusion_core::types::tensor::Matrix;
use num_complex::Complex64;
use std::f64::consts::PI;

pub struct QFTGenerator;

impl QFTGenerator {
   /// Generate QFT Circuit.
   pub fn generate(num_qubits: usize) -> FusionResult<QuantumCircuit> {
       if num_qubits == 0 {
           return Err(fusion_core::FusionError::InvalidDimension("Qubits must be > 0".into()));
       }

       let mut circuit = QuantumCircuit::new(num_qubits);

       for i in 0..num_qubits {
           // 1. Hadamard
           circuit.apply_gate(QuantumGate::hadamard(), vec![i])?;

           // 2. Controlled Phase Rotations
           // R_k acts on qubit 'i', controlled by qubit 'j'
           for j in (i + 1)..num_qubits {
               let k = j - i + 1; // distance
               let angle = 2.0 * PI / (1 << k) as f64; // 2^k
               
               let cp_gate = Self::controlled_phase(angle)?;
               // Standard QFT convention: Control j, Target i
               circuit.apply_gate(cp_gate, vec![j, i])?;
           }
       }

       // 3. Swap Qubits (Reverse order) to match endianness
       for i in 0..(num_qubits / 2) {
           let swap = fusion_core::ops::quantum_ops::swap(); // Assuming swap helper exists or we define it
           // If not existing, we build SWAP from 3 CNOTs in real impl.
           // Using placeholder logic for swap construction:
           circuit.apply_gate(Self::swap_gate()?, vec![i, num_qubits - 1 - i])?;
       }

       Ok(circuit)
   }

   fn controlled_phase(theta: f64) -> FusionResult<QuantumGate> {
       let one = Complex64::new(1.0, 0.0);
       let zero = Complex64::new(0.0, 0.0);
       let phase = Complex64::from_polar(1.0, theta);
       
       let data = vec![
           one, zero, zero, zero,
           zero, one, zero, zero,
           zero, zero, one, zero,
           zero, zero, zero, phase
       ];
       
       let matrix = Matrix::new(data, [4, 4])?;
       
       Ok(QuantumGate {
           name: format!("CP({:.2})", theta),
           matrix,
           num_qubits: 2,
       })
   }

   fn swap_gate() -> FusionResult<QuantumGate> {
       let one = Complex64::new(1.0, 0.0);
       let zero = Complex64::new(0.0, 0.0);
       
       // SWAP: |00>->|00>, |01>->|10>, |10>->|01>, |11>->|11>
       let data = vec![
           one, zero, zero, zero,
           zero, zero, one, zero,
           zero, one, zero, zero,
           zero, zero, zero, one
       ];
       let matrix = Matrix::new(data, [4, 4])?;
       
       Ok(QuantumGate {
           name: "SWAP".into(),
           matrix,
           num_qubits: 2,
       })
   }
}