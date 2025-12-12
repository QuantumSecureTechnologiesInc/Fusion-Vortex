use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Specification for a Fusion Application
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "fusion-lang.org",
    version = "v1",
    kind = "FusionApp",
    namespaced
)]
#[kube(status = "FusionAppStatus")]
pub struct FusionAppSpec {
    /// Number of replicas to run
    pub replicas: i32,
    /// Container image for the application
    pub image: String,
    /// Quantum resources required
    pub quantum_resources: Option<QuantumResources>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct QuantumResources {
    pub qubits: i32,
    pub coherence_time_ms: Option<u64>,
}

/// Status of a Fusion Application
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct FusionAppStatus {
    /// Number of replicas currently available
    pub available_replicas: i32,
    /// Current phase of the application
    pub phase: String,
    /// Job ID associated with quantum resources (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantum_job_id: Option<String>,
}
