// src/quantum/mod.rs - Quantum Computing Library for Fusion
#![allow(dead_code)]
// Provides quantum circuit building, simulation, and hardware integration

pub mod analysis;
pub mod circuit;
pub mod gates;
pub mod simulator;

/// Quantum error types
#[derive(Debug, Clone)]
pub enum QuantumError {
    /// Invalid qubit index
    InvalidQubit(usize),
    /// Circuit not initialized
    UninitializedCircuit,
    /// Measurement error
    MeasurementError(String),
    /// Backend error
    BackendError(String),
    /// Invalid gate operation
    InvalidGate(String),
}

impl std::fmt::Display for QuantumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumError::InvalidQubit(idx) => write!(f, "Invalid qubit index: {}", idx),
            QuantumError::UninitializedCircuit => write!(f, "Circuit not initialized"),
            QuantumError::MeasurementError(msg) => write!(f, "Measurement error: {}", msg),
            QuantumError::BackendError(msg) => write!(f, "Backend error: {}", msg),
            QuantumError::InvalidGate(msg) => write!(f, "Invalid gate: {}", msg),
        }
    }
}

impl std::error::Error for QuantumError {}

/// Complex number representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn i() -> Self {
        Self::new(0.0, 1.0)
    }

    /// Magnitude (absolute value)
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Phase angle
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Complex conjugate
    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imag)
    }

    /// Multiply by complex number
    pub fn mul(&self, other: &Complex) -> Self {
        Self::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }

    /// Add complex numbers
    pub fn add(&self, other: &Complex) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}

/// Quantum state vector
pub type StateVector = Vec<Complex>;

/// Create initial |0⟩ state
pub fn state_zero() -> Complex {
    Complex::one()
}

/// Create initial |1⟩ state  
pub fn state_one() -> Complex {
    Complex::one()
}

/// Quantum backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumBackend {
    /// Local simulator
    Simulator,
    /// IBM Quantum (real hardware)
    IBMQ,
    /// AWS Braket
    AWSBraket,
    /// Google Cirq
    GoogleCirq,
}

impl QuantumBackend {
    pub fn name(&self) -> &'static str {
        match self {
            QuantumBackend::Simulator => "Local Simulator",
            QuantumBackend::IBMQ => "IBM Quantum",
            QuantumBackend::AWSBraket => "AWS Braket",
            QuantumBackend::GoogleCirq => "Google Cirq",
        }
    }

    pub fn is_hardware(&self) -> bool {
        !matches!(self, QuantumBackend::Simulator)
    }
}

/// Quantum statistics
#[derive(Debug, Default)]
pub struct QuantumStats {
    pub circuits_created: usize,
    pub gates_applied: usize,
    pub measurements_taken: usize,
    pub simulations_run: usize,
}

impl QuantumStats {
    pub fn print_summary(&self) {
        println!("\n🔬 Quantum Computing Statistics:");
        println!("  Circuits created: {}", self.circuits_created);
        println!("  Gates applied: {}", self.gates_applied);
        println!("  Measurements: {}", self.measurements_taken);
        println!("  Simulations: {}", self.simulations_run);
    }
}

/// Quantum constants
pub mod constants {
    use std::f64::consts::PI as STD_PI;

    /// 1/sqrt(2) - common quantum constant
    pub const FRAC_1_SQRT_2: f64 = 0.7071067811865476;

    /// Pi constant
    pub const PI: f64 = STD_PI;

    /// Common rotation angles
    pub const PI_2: f64 = STD_PI / 2.0;
    pub const PI_4: f64 = STD_PI / 4.0;
    pub const PI_8: f64 = STD_PI / 8.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_multiply() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let result = a.mul(&b);

        assert_eq!(result.real, -5.0); // 1*3 - 2*4
        assert_eq!(result.imag, 10.0); // 1*4 + 2*3
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();

        assert_eq!(conj.real, 3.0);
        assert_eq!(conj.imag, -4.0);
    }

    #[test]
    fn test_backend_names() {
        assert_eq!(QuantumBackend::Simulator.name(), "Local Simulator");
        assert!(!QuantumBackend::Simulator.is_hardware());
        assert!(QuantumBackend::IBMQ.is_hardware());
    }
}
