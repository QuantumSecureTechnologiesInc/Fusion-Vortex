# Fusion Core Type System - Implementation Plan

**Version**: 1.0  
**Date**: December 7, 2025  
**Status**: Implementation Ready  
**Target**: v0.2.0

---

## Executive Summary

This document outlines the **step-by-step implementation plan** for the Fusion Core Type System, as specified in `Core_Type_System_Design.md`. The implementation will be delivered in phases over 2-3 months.

---

## Phase 1: Foundation (Weeks 1-2)

### Week 1: Project Setup

**Deliverables**:
- ✅ Create `fusion_core` crate
- ✅ Set up module structure
- ✅ Define basic type traits
- ✅ Establish testing framework

**Files to Create**:
```
fusion_core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types/
│   │   ├── mod.rs
│   │   ├── classical.rs
│   │   ├── tensor.rs
│   │   └── quantum.rs
│   └── traits.rs
└── tests/
    └── type_system_tests.rs
```

**Code Skeleton**:
```rust
// fusion_core/src/lib.rs
pub mod types;
pub mod traits;
pub mod ops;
pub mod runtime;

pub use types::{ClassicalType, TensorType, QuantumType, FusionType};
pub use traits::{Numeric, Measurable, Unitary};
```

### Week 2: Classical Type System

**Tasks**:
1. Implement primitive types (int, float, bool, string)
2. Implement compound types (struct, enum, tuple)
3. Add collection types (Vector, HashMap, HashSet wrapper)
4. Write unit tests

**Implementation**:
```rust
// fusion_core/src/types/classical.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ClassicalType {
    // Primitives
    Int(IntType),
    Float(FloatType),
    Bool,
    Char,
    String,
    
    // Compound
    Struct(StructType),
    Enum(EnumType),
    Tuple(Vec<ClassicalType>),
    
    // Collections
    Vector(Box<ClassicalType>),
    HashMap(Box<ClassicalType>, Box<ClassicalType>),
    HashSet(Box<ClassicalType>),
    
    // References
    Reference(Box<ClassicalType>),
    MutReference(Box<ClassicalType>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntType {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatType {
    F32, F64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, ClassicalType)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<ClassicalType>,
}
```

**Tests**:
```rust
#[test]
fn test_classical_type_creation() {
    let int_type = ClassicalType::Int(IntType::I64);
    let vec_type = ClassicalType::Vector(Box::new(int_type.clone()));
    assert_eq!(vec_type, ClassicalType::Vector(Box::new(ClassicalType::Int(IntType::I64))));
}
```

---

## Phase 2: Tensor Type System (Weeks 3-4)

### Week 3: Tensor Types

**Tasks**:
1. Define Tensor<T, RANK> type
2. Implement const generics for rank
3. Add DataType enum
4. Implement basic shape operations

**Implementation**:
```rust
// fusion_core/src/types/tensor.rs

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Tensor<T: Numeric, const RANK: usize> {
    data: Vec<T>,
    shape: [usize; RANK],
    strides: [usize; RANK],
    dtype: DataType,
    _phantom: PhantomData<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    Complex64, Complex128,
    Bool,
}

// Type aliases for common ranks
pub type Scalar<T> = Tensor<T, 0>;
pub type Vector1D<T> = Tensor<T, 1>;
pub type Matrix<T> = Tensor<T, 2>;
pub type Tensor3D<T> = Tensor<T, 3>;

impl<T: Numeric, const RANK: usize> Tensor<T, RANK> {
    pub fn zeros(shape: [usize; RANK]) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![T::zero(); size];
        let strides = Self::compute_strides(&shape);
        
        Tensor {
            data,
            shape,
            strides,
            dtype: T::data_type(),
            _phantom: PhantomData,
        }
    }
    
    pub fn ones(shape: [usize; RANK]) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![T::one(); size];
        let strides = Self::compute_strides(&shape);
        
        Tensor {
            data,
            shape,
            strides,
            dtype: T::data_type(),
            _phantom: PhantomData,
        }
    }
    
    fn compute_strides(shape: &[usize; RANK]) -> [usize; RANK] {
        let mut strides = [1; RANK];
        for i in (0..RANK-1).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }
    
    pub fn get(&self, indices: [usize; RANK]) -> T {
        let index = self.compute_index(&indices);
        self.data[index]
    }
    
    pub fn set(&mut self, indices: [usize; RANK], value: T) {
        let index = self.compute_index(&indices);
        self.data[index] = value;
    }
    
    fn compute_index(&self, indices: &[usize; RANK]) -> usize {
        indices.iter()
            .zip(self.strides.iter())
            .map(|(i, s)| i * s)
            .sum()
    }
}
```

### Week 4: Tensor Operations

**Tasks**:
1. Implement arithmetic operations (+, -, *, /)
2. Add reduction operations (sum, mean, max, min)
3. Implement reshape, transpose
4. Write comprehensive tests

**Implementation**:
```rust
// fusion_core/src/ops/tensor_ops.rs

use crate::types::tensor::{Tensor, Matrix};
use crate::traits::Numeric;

// Element-wise operations
impl<T: Numeric, const RANK: usize> std::ops::Add for Tensor<T, RANK> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.shape, other.shape, "Shape mismatch in addition");
        
        let data: Vec<T> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.add(*b))
            .collect();
        
        Tensor {
            data,
            shape: self.shape,
            strides: self.strides,
            dtype: self.dtype,
            _phantom: PhantomData,
        }
    }
}

// Matrix multiplication
impl<T: Numeric> Matrix<T> {
    pub fn matmul(&self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.shape[1], other.shape[0], 
                   "Matrix multiplication shape mismatch");
        
        let m = self.shape[0];
        let k = self.shape[1];
        let n = other.shape[1];
        
        let mut result = Matrix::zeros([m, n]);
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = T::zero();
                for p in 0..k {
                    let a = self.get([i, p]);
                    let b = other.get([p, j]);
                    sum = sum.add(a.mul(b));
                }
                result.set([i, j], sum);
            }
        }
        
        result
    }
}

// Reductions
impl<T: Numeric, const RANK: usize> Tensor<T, RANK> {
    pub fn sum(&self) -> T {
        self.data.iter().fold(T::zero(), |acc, x| acc.add(*x))
    }
    
    pub fn mean(&self) -> T {
        let total = self.sum();
        let count = T::from_usize(self.data.len());
        total.div(count)
    }
}
```

---

## Phase 3: Quantum Type System (Weeks 5-6)

### Week 5: Quantum Types

**Tasks**:
1. Define Qubit, QubitRegister types
2. Implement QuantumGate structure
3. Add quantum state representation
4. Enforce no-cloning theorem

**Implementation**:
```rust
// fusion_core/src/types/quantum.rs

use num_complex::Complex64;
use std::sync::Arc;

/// Quantum bit - cannot be cloned (no-cloning theorem)
pub struct Qubit {
    id: QubitId,
    state: Arc<QuantumState>,  // Shared state for simulator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitId(usize);

/// Quantum state (for simulation)
#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitudes: Vec<Complex64>,
    pub num_qubits: usize,
}

impl QuantumState {
    pub fn zeros(num_qubits: usize) -> Self {
        let num_states = 1 << num_qubits;  // 2^n
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); num_states];
        amplitudes[0] = Complex64::new(1.0, 0.0);  // |00...0⟩
        
        QuantumState {
            amplitudes,
            num_qubits,
        }
    }
    
    pub fn superposition(num_qubits: usize) -> Self {
        let num_states = 1 << num_qubits;
        let amplitude = Complex64::new(1.0 / (num_states as f64).sqrt(), 0.0);
        let amplitudes = vec![amplitude; num_states];
        
        QuantumState {
            amplitudes,
            num_qubits,
        }
    }
    
    pub fn is_normalized(&self) -> bool {
        let total: f64 = self.amplitudes.iter()
            .map(|a| a.norm_sqr())
            .sum();
        (total - 1.0).abs() < 1e-10
    }
    
    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.iter()
            .map(|a| a.norm_sqr())
            .sum::<f64>()
            .sqrt();
        
        for amp in &mut self.amplitudes {
            *amp /= norm;
        }
    }
}

// Qubit CANNOT be cloned (quantum no-cloning theorem)
// Note: Clone trait is NOT implemented

impl Qubit {
    pub fn new() -> Self {
        static mut NEXT_ID: usize = 0;
        let id = unsafe {
            let current = NEXT_ID;
            NEXT_ID += 1;
            QubitId(current)
        };
        
        Qubit {
            id,
            state: Arc::new(QuantumState::zeros(1)),
        }
    }
    
    /// Measure qubit (consumes it, returns classical bit)
    pub fn measure(self) -> bool {
        // Measurement collapses the state
        let prob_zero = self.state.amplitudes[0].norm_sqr();
        
        // Random measurement outcome based on probability
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < prob_zero
    }
}

/// Qubit register
pub struct QubitRegister {
    qubits: Vec<Qubit>,
    state: Arc<QuantumState>,
}

impl QubitRegister {
    pub fn new(n: usize) -> Self {
        let state = Arc::new(QuantumState::zeros(n));
        let qubits = (0..n).map(|_| {
            Qubit {
                id: QubitId(0),  // Temporary
                state: state.clone(),
            }
        }).collect();
        
        QubitRegister { qubits, state }
    }
    
    pub fn measure_all(self) -> Vec<bool> {
        // Measure all qubits
        self.qubits.into_iter().map(|q| q.measure()).collect()
    }
}
```

### Week 6: Quantum Gates & Circuits

**Tasks**:
1. Implement standard quantum gates
2. Build quantum circuit framework
3. Add gate application logic
4. Verify unitarity

**Implementation**:
```rust
// fusion_core/src/types/quantum.rs (continued)

use crate::types::tensor::Matrix;

/// Quantum gate (unitary operation)
#[derive(Debug, Clone)]
pub struct QuantumGate {
    pub name: String,
    pub matrix: Matrix<Complex64>,
    pub num_qubits: usize,
}

impl QuantumGate {
    // Single-qubit gates
    pub fn hadamard() -> Self {
        let matrix = Matrix::from_vec(
            vec![
                Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
                Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
                Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0),
                Complex64::new(-1.0 / 2.0_f64.sqrt(), 0.0),
            ],
            [2, 2]
        );
        
        QuantumGate {
            name: "H".to_string(),
            matrix,
            num_qubits: 1,
        }
    }
    
    pub fn pauli_x() -> Self {
        let matrix = Matrix::from_vec(
            vec![
                Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
            ],
            [2, 2]
        );
        
        QuantumGate {
            name: "X".to_string(),
            matrix,
            num_qubits: 1,
        }
    }
    
    pub fn pauli_z() -> Self {
        let matrix = Matrix::from_vec(
            vec![
                Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0),
            ],
            [2, 2]
        );
        
        QuantumGate {
            name: "Z".to_string(),
            matrix,
            num_qubits: 1,
        }
    }
    
    // Two-qubit gate
    pub fn cnot() -> Self {
        let matrix = Matrix::from_vec(
            vec![
                Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), 
                Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0),
                
                Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), 
                Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0),
                
                Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), 
                Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0),
                
                Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), 
                Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
            ],
            [4, 4]
        );
        
        QuantumGate {
            name: "CNOT".to_string(),
            matrix,
            num_qubits: 2,
        }
    }
    
    /// Verify gate is unitary (U†U = I)
    pub fn is_unitary(&self) -> bool {
        let conjugate_transpose = self.matrix.conjugate_transpose();
        let product = conjugate_transpose.matmul(&self.matrix);
        product.is_identity(1e-10)
    }
}

/// Quantum circuit
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<GateApplication>,
}

#[derive(Debug, Clone)]
pub struct GateApplication {
    pub gate: QuantumGate,
    pub targets: Vec<usize>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        QuantumCircuit {
            num_qubits,
            gates: Vec::new(),
        }
    }
    
    pub fn apply(&mut self, gate: QuantumGate, targets: Vec<usize>) {
        assert_eq!(gate.num_qubits, targets.len(), 
                   "Gate requires {} qubits, got {}", gate.num_qubits, targets.len());
        
        for &target in &targets {
            assert!(target < self.num_qubits, 
                    "Target qubit {} out of range", target);
        }
        
        self.gates.push(GateApplication { gate, targets });
    }
    
    pub fn simulate(&self) -> QuantumState {
        let mut state = QuantumState::zeros(self.num_qubits);
        
        for gate_app in &self.gates {
            apply_gate_to_state(&mut state, &gate_app.gate, &gate_app.targets);
        }
        
        state
    }
}

fn apply_gate_to_state(state: &mut QuantumState, gate: &QuantumGate, targets: &[usize]) {
    // Apply gate to quantum state
    // This is complex - full implementation would expand the gate to full Hilbert space
    // For now, simplified version for single-qubit gates
    
    if gate.num_qubits == 1 {
        let target = targets[0];
        apply_single_qubit_gate(state, gate, target);
    }
    // Multi-qubit gates require tensor product expansion
}

fn apply_single_qubit_gate(state: &mut QuantumState, gate: &QuantumGate, target: usize) {
    let num_states = state.amplitudes.len();
    let mut new_amplitudes = vec![Complex64::new(0.0, 0.0); num_states];
    
    for i in 0..num_states {
        let bit = (i >> target) & 1;  // Get target bit
        let flipped = i ^ (1 << target);  // Flip target bit
        
        if bit == 0 {
            new_amplitudes[i] = gate.matrix.get([0, 0]) * state.amplitudes[i] 
                              + gate.matrix.get([0, 1]) * state.amplitudes[flipped];
        } else {
            new_amplitudes[i] = gate.matrix.get([1, 0]) * state.amplitudes[flipped] 
                              + gate.matrix.get([1, 1]) * state.amplitudes[i];
        }
    }
    
    state.amplitudes = new_amplitudes;
}
```

---

## Phase 4: Type Integration (Weeks 7-8)

### Week 7: Hybrid Type System

**Tasks**:
1. Create unified FusionType enum
2. Implement type conversions
3. Add type checking utilities
4. Write conversion tests

**Implementation**:
```rust
// fusion_core/src/types/mod.rs

pub mod classical;
pub mod tensor;
pub mod quantum;
pub mod hybrid;

pub use classical::ClassicalType;
pub use tensor::TensorType;
pub use quantum::QuantumType;

#[derive(Debug, Clone, PartialEq)]
pub enum FusionType {
    Classical(ClassicalType),
    Tensor(TensorType),
    Quantum(QuantumType),
}

impl FusionType {
    pub fn is_classical(&self) -> bool {
        matches!(self, FusionType::Classical(_))
    }
    
    pub fn is_tensor(&self) -> bool {
        matches!(self, FusionType::Tensor(_))
    }
    
    pub fn is_quantum(&self) -> bool {
        matches!(self, FusionType::Quantum(_))
    }
    
    pub fn can_convert_to(&self, other: &FusionType) -> bool {
        match (self, other) {
            // Classical ↔ Tensor
            (FusionType::Classical(_), FusionType::Tensor(_)) => true,
            (FusionType::Tensor(_), FusionType::Classical(_)) => true,
            
            // Tensor ↔ Quantum (state vectors)
            (FusionType::Tensor(_), FusionType::Quantum(_)) => true,
            
            // Quantum → Classical (measurement only)
            (FusionType::Quantum(_), FusionType::Classical(_)) => true,
            
            // Same type
            _ if self == other => true,
            
            _ => false,
        }
    }
}
```

### Week 8: Compiler Integration

**Tasks**:
1. Update semantic analyzer with FusionType
2. Add type checking for hybrid expressions
3. Implement type inference
4. Integration tests

**Implementation**:
```rust
// src/semantic_analyzer/type_checker.rs

use fusion_core::types::FusionType;
use fusion_core::types::classical::ClassicalType;
use fusion_core::types::tensor::TensorType;
use fusion_core::types::quantum::QuantumType;

pub struct TypeChecker {
    type_env: HashMap<String, FusionType>,
}

impl TypeChecker {
    pub fn check_expression(&mut self, expr: &Expression) 
        -> Result<FusionType, TypeError> {
        match expr {
            // Classical
            Expression::IntLiteral(_) => {
                Ok(FusionType::Classical(ClassicalType::Int(IntType::I64)))
            },
            
            // Tensor
            Expression::TensorCreation { shape, dtype } => {
                Ok(FusionType::Tensor(TensorType {
                    rank: shape.len(),
                    dtype: *dtype,
                    shape: Some(shape.clone()),
                }))
            },
            
            Expression::MatMul(a, b) => {
                self.check_matmul(a, b)
            },
            
            // Quantum
            Expression::QubitAlloc(n) => {
                Ok(FusionType::Quantum(QuantumType::Register(*n)))
            },
            
            Expression::GateApplication { gate, qubits } => {
                self.check_gate_application(gate, qubits)
            },
            
            Expression::Measurement(qubit) => {
                // Quantum → Classical
                let qubit_type = self.check_expression(qubit)?;
                if !matches!(qubit_type, FusionType::Quantum(_)) {
                    return Err(TypeError::TypeMismatch {
                        expected: FusionType::Quantum(QuantumType::Qubit),
                        found: qubit_type,
                    });
                }
                Ok(FusionType::Classical(ClassicalType::Bool))
            },
            
            _ => Err(TypeError::UnsupportedExpression),
        }
    }
}
```

---

## Phase 5: Testing & Documentation (Weeks 9-10)

### Week 9: Comprehensive Testing

**Test Categories**:
1. Unit tests for each type
2. Integration tests for type conversions
3. Property-based tests for quantum no-cloning
4. Performance benchmarks

**Example Tests**:
```rust
#[test]
fn test_quantum_no_cloning() {
    let q = Qubit::new();
    // let q_copy = q.clone();  // ❌ Compile error!
    let _ = q.measure();  // OK: measurement consumes
}

#[test]
fn test_bell_state_simulation() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.apply(QuantumGate::hadamard(), vec![0]);
    circuit.apply(QuantumGate::cnot(), vec![0, 1]);
    
    let state = circuit.simulate();
    
    // Expected: (|00⟩ + |11⟩) / √2
    assert!((state.amplitudes[0b00].norm_sqr() - 0.5).abs() < 1e-10);
    assert!((state.amplitudes[0b11].norm_sqr() - 0.5).abs() < 1e-10);
}

#[test]
fn test_matmul_correctness() {
    let a = Matrix::from_vec(vec![1.0, 2.0, 3.0, 4.0], [2, 2]);
    let b = Matrix::from_vec(vec![5.0, 6.0, 7.0, 8.0], [2, 2]);
    let c = a.matmul(&b);
    
    // Expected: [[19, 22], [43, 50]]
    assert_eq!(c.get([0, 0]), 19.0);
    assert_eq!(c.get([0, 1]), 22.0);
    assert_eq!(c.get([1, 0]), 43.0);
    assert_eq!(c.get([1, 1]), 50.0);
}
```

### Week 10: Documentation

**Documentation Tasks**:
1. API reference documentation
2. User guide for each type system
3. Hybrid programming examples
4. Performance optimization guide

---

## Deliverables Summary

### Code Deliverables

| Component       | Files  | Lines     | Tests   |
| --------------- | ------ | --------- | ------- |
| Classical Types | 2      | 500       | 20      |
| Tensor Types    | 3      | 1,500     | 30      |
| Quantum Types   | 3      | 2,000     | 40      |
| Hybrid System   | 2      | 800       | 20      |
| **Total**       | **10** | **4,800** | **110** |

### Documentation Deliverables

1. ✅ Core Type System Design (this document)
2. ✅ Implementation Plan (this document)
3. API Reference (rustdoc)
4. User Guide
5. Hybrid Programming Guide
6. Performance Guide

---

## Success Criteria

**Phase 1 Complete**:
- ✅ All classical types implemented
- ✅ Basic tensor operations working
- ✅ 50+ tests passing

**Phase 2 Complete**:
- ✅ Full tensor library functional
- ✅ GPU backend integrated
- ✅ 80+ tests passing

**Phase 3 Complete**:
- ✅ Quantum types implemented
- ✅ Quantum simulator working
- ✅ Bell state example runs

**Phase 4 Complete**:
- ✅ Hybrid type system integrated
- ✅ Compiler type checking works
- ✅ 110+ tests passing
- ✅ Documentation complete

**v0.2.0 Release Ready**:
- ✅ All tests passing
- ✅ Performance acceptable
- ✅ API stable
- ✅ Examples working
- ✅ Documentation published

---

## Risk Mitigation

### Risk 1: Quantum Simulator Performance

**Mitigation**:
- Use sparse state representation
- Implement GPU acceleration
- Add circuit optimization passes

### Risk 2: Type System Complexity

**Mitigation**:
- Incremental implementation
- Extensive testing at each step
- Clear documentation

### Risk 3: Hardware Integration

**Mitigation**:
- Design abstraction layer
- Support multiple backends
- Simulator-first development

---

## Next Steps

1. **Immediate** (Week 1):
   - Create `fusion_core` crate
   - Set up CI/CD
   - Begin classical type implementation

2. **Short-term** (Weeks 2-4):
   - Complete classical + tensor types
   - Integration with existing compiler

3. **Medium-term** (Weeks 5-8):
   - Quantum types implementation
   - Hybrid type system

4. **Long-term** (Weeks 9-10):
   - Testing and hardening
   - Documentation
   - v0.2.0 release

---

**Status**: ✅ Ready for Implementation  
**Target**: v0.2.0 Release  
**Estimated Timeline**: 10 weeks  
**Team Size**: 1-2 developers  

---

End of Implementation Plan
