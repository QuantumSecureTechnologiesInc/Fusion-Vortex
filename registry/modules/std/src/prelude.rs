//! Fusion prelude.
//! Importing this module shadows common types with Fusion equivalents.
pub use crate::collections::FVec as Vec;
pub use crate::core::{Environment, Narrative, RuntimeContext, Verse};
pub use crate::cycle::{ComputeNode, CycleFailure, MarketBid, ServiceAgreement};
pub use crate::ai::{
    inference_engine, load_bpe_runtime, session_from_env, LlmConfig, LlmRuntime,
};
pub use crate::flow::{spawn_narrative, wait_for, NarrativeTask};
pub use crate::primitives::{FBool, F32, FInt, FString as String};
pub use crate::runtime::{fusion_channel, spawn_secure, FusionReceiver, FusionSender};
pub use crate::security::{clear_context, set_context, FusionContext};
pub use crate::seal::{NeuralGuard, NeuralSanitiser, SecurityViolation};
pub use crate::stdlib;
pub use crate::{ensure, fusion_if, fusion_loop, fusion_match, fvec, verse, verse_with};
pub type Int = FInt;
pub type Bool = FBool;
/// Extends `String` and `str` with NeuralSeal capabilities.
pub trait FusionString {
    /// Attempt to sanitise this String using the default NeuralSanitiser.
    fn sanitise(&self) -> Result<FString, SecurityViolation>;
    /// Checks if the String looks like a valid system ID (simple heuristic).
    fn is_valid_id(&self) -> FBool;
}
impl FusionString for str {
    fn sanitise(&self) -> Result<FString, SecurityViolation> {
        let sanitiser = NeuralSanitiser::new();
        let cleaned = sanitiser.cleanse(self)?;
        Ok(String::from(cleaned.as_str()))
    }
    fn is_valid_id(&self) -> FBool {
        !self.is_empty()
            && self.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}
impl FusionString for FString {
    fn sanitise(&self) -> Result<FString, SecurityViolation> {
        self.as_str().sanitise()
    }
    fn is_valid_id(&self) -> FBool {
        self.as_str().is_valid_id()
    }
}