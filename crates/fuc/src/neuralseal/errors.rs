use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityViolation {
    #[error("Malicious script injection payload detected during character verification pass.")]
    載InjectionAttemptDetected,
    
    #[error("Inbound communication buffer exceeds hard infrastructure volume limits.")]
    PayloadTooLarge,
    
    #[error("Execution path denied. Assigned token does not hold relevant structural clearance.")]
    AccessDenied,
    
    #[error("Internal system integrity state vector has degraded or tripped active locks.")]
    ProcessingError,
}