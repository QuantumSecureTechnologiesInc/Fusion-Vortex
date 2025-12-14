# Fusion AI Training Datasets - Master Index

## Quick Reference

| Group | Stage   | Dataset File                 | Records | Purpose              | Epochs   |
| ----- | ------- | ---------------------------- | ------- | -------------------- | -------- |
| 1     | Stage 1 | `fusion_syntax_corpus.jsonl` | 20      | Syntax fluency       | 2        |
| 1     | Stage 2 | `fusion_llvm_ir_pairs.jsonl` | 7       | IR optimization      | 3        |
| 1     | Stage 3 | `security_invariants.jsonl`  | 20      | Security enforcement | 2        |
| 1     | Stage 4 | `preference_pairs.jsonl`     | 10      | Safety preference    | 1        |
| 2     | Stage 1 | `instruction_dataset.jsonl`  | 15      | Dev assistance       | 3 (LoRA) |

## Training Scripts

- **GROUP 1**: `GROUP 1 - The Architect/train_architect.py`
  - Full 4-stage pipeline for compiler intelligence
  - Run: `python train_architect.py --full`
  - Or stage-by-stage: `python train_architect.py --stage 1`

- **GROUP 2**: `GROUP 2 - The Oracle/train_oracle.py`
  - LoRA instruction fine-tuning for development assistant
  - Run: `python train_oracle.py`

## Configuration Files

- **GROUP 1**: `GROUP 1 - The Architect/training_config.yaml`
- **GROUP 2**: `GROUP 2 - The Oracle/training_config.yaml`

## Dependencies

```bash
pip install transformers datasets torch trl peft bitsandbytes accelerate wandb tensorboard pyyaml
```

## Hardware Requirements

### GROUP 1 (Full Model Training)
- **Minimum**: 1x NVIDIA A100 (40GB)
- **Recommended**: 2x NVIDIA A100 (80GB)
- **Alternative**: 1x H100 (80GB)

### GROUP 2 (LoRA Training)
- **Minimum**: 1x NVIDIA RTX 4090 (24GB)
- **Recommended**: 1x NVIDIA A100 (40GB)
- **4-bit quantization**: Fits on RTX 3090/4090

## Quick Start

### GROUP 1: Train The Architect
```bash
cd "GROUP 1 - The Architect"
python train_architect.py --config training_config.yaml --full
```

### GROUP 2: Train The Oracle
```bash
cd "GROUP 2 - The Oracle"
python train_oracle.py --config training_config.yaml
```

## Dataset Statistics

### Total Coverage
- **Syntax examples**: 20
- **LLVM IR mappings**: 7
- **Security examples**: 20 (violations + fixes)
- **Preference pairs**: 10
- **Instruction examples**: 15
- **Total training samples**: 72

### Category Distribution

**Syntax Categories** (20):
- Basic: variables, functions, structs (6)
- Intermediate: generics, traits, error handling (7)
- Advanced: ownership, async, tensors (7)

**Optimization Patterns** (7):
- Loops, SIMD, GPU, Constant-time, Matrix ops

**Security Attributes** (8):
- @constant_time, @manual_memory, @pure, @no_std, @gpu_kernel, @inline, @thread_safe, @deprecated

**Development Categories** (14):
- Web, ML, Concurrency, Data structures, Cryptography, UI, Database, Memory, Distributed

## Expected Training Time

### GROUP 1 (Sequential 4 stages)
- Stage 1: ~2 hours (2 epochs, 20 samples)
- Stage 2: ~2 hours (3 epochs, 7 samples)
- Stage 3: ~1.5 hours (2 epochs, 20 samples)
- Stage 4: ~0.5 hours (1 epoch, 10 samples)
- **Total**: ~6 hours on A100

### GROUP 2 (LoRA)
- Stage 1: ~3 hours (3 epochs, 15 samples)
- **Total**: ~3 hours on A100 or ~6 hours on RTX 4090

## Monitoring

- **W&B**: Set `WANDB_PROJECT` environment variable
- **TensorBoard**: Logs saved to `{output_dir}/runs`

```bash
tensorboard --logdir ./models/fusion-architect/runs
```

## Post-Training

### Merge LoRA Adapters (GROUP 2)
Automatically performed if `merge_adapters: true` in config

### Export for Inference
```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained("./models/fusion-architect-final")
tokenizer = AutoTokenizer.from_pretrained("./models/fusion-architect-final")
```

## Validation

Test models on held-out Fusion code:
```python
prompt = "Write a Fusion function to calculate factorial"
inputs = tokenizer(prompt, return_tensors="pt")
outputs = model.generate(**inputs, max_length=200)
print(tokenizer.decode(outputs[0]))
```

## Directory Structure
```
AI Training/
├── README.md (from previous conversation)
├── TRAINING_INDEX.md (this file)
├── GROUP 1 - The Architect/
│   ├── training_config.yaml
│   ├── train_architect.py
│   ├── Stage 1 - Fusion Syntax SFT/
│   │   └── fusion_syntax_corpus.jsonl
│   ├── Stage 2 - LLVM Optimization Mapping/
│   │   └── fusion_llvm_ir_pairs.jsonl
│   ├── Stage 3 - Security Invariant Enforcement/
│   │   └── security_invariants.jsonl
│   └── Stage 4 - Self-Correction DPO/
│       └── preference_pairs.jsonl
└── GROUP 2 - The Oracle/
    ├── training_config.yaml
    ├── train_oracle.py
    └── Stage 1 - Instruction Fine-Tuning/
        └── instruction_dataset.jsonl
```

## Notes

- All datasets use JSONL (JSON Lines) format
- Training configs use YAML for readability
- Scripts support both single-stage and full pipeline execution
- LoRA adapters significantly reduce training memory requirements
- DPO training requires reference model (loaded from Stage 3)

## Next Steps

1. Review dataset contents
2. Adjust training configs for your hardware
3. Set up W&B monitoring
4. Run training pipelines
5. Validate on held-out samples
6. Deploy for inference

## Support

See main project documentation for additional guidance.
