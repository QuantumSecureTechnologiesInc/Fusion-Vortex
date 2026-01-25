// __FU_COMPAT_START__
#![allow(missing_docs)]
#[allow(missing_docs, dead_code)] type FBool = bool;
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FU32 = u32;
// __FU_COMPAT_END__
use fusion_core::FusionResult;
use fusion_llm_inference_engine::InferenceEngine;
use fusion_llm_tokenizers::bpe::BPETokenizer;

/// Runtime LLM configuration loaded from environment.
pub struct LlmConfig {
    pub device: FString,
    pub mixed_precision: FBool,
    pub quantization: FString,
    pub batch_size: FU32,
}

impl LlmConfig {
    pub fn from_env() -> Self {
        let device = std::env::var("FUSION_AI_DEVICE").unwrap_or_else(|_| "cpu".to_string());
        let mixed_precision = std::env::var("FUSION_AI_MIXED_PRECISION")
            .ok()
            .and_then(|v| v.parse::<FBool>().ok())
            .unwrap_or(false);
        let quantization = std::env::var("FUSION_AI_QUANTIZATION")
            .unwrap_or_else(|_| "none".to_string());
        let batch_size = std::env::var("FUSION_AI_BATCH_SIZE")
            .ok()
            .and_then(|v| v.parse::<FU32>().ok())
            .unwrap_or(1);
        Self {
            device,
            mixed_precision,
            quantization,
            batch_size,
        }
    }
}

/// LLM runtime bundle combining inference and tokenization.
pub struct LlmRuntime {
    pub config: LlmConfig,
    pub inference: InferenceEngine,
    pub tokenizer: BPETokenizer,
}

impl LlmRuntime {
    pub fn new(model_path: FString, tokenizer: BPETokenizer) -> Self {
        let config = LlmConfig::from_env();
        let inference = InferenceEngine::new(model_path);
        Self {
            config,
            inference,
            tokenizer,
        }
    }

    pub fn infer(&self, input: &fusion_core::types::tensor::Matrix<f64>) -> FusionResult<fusion_core::types::tensor::Matrix<f64>> {
        self.inference.infer(input)
    }
}
