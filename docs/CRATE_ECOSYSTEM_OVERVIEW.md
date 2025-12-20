# Fusion Crate Ecosystem Documentation

**Generated**: 2025-12-17 15:06:19 UTC  
**Total Crates**: 250

## Overview

The Fusion Programming Language ecosystem consists of 250 crates organized into six primary archetypes:
- **Algorithm**: 91 crates
- **Experimental**: 6 crates
- **Foundation**: 12 crates
- **Framework**: 29 crates
- **Integration**: 27 crates
- **Tool**: 85 crates

---

## Crates by Archetype

### Foundation (12 crates)
> **Foundation crates** provide core primitives and building blocks. They are dependency-minimal, panic-free, and designed for composition.

| Crate | Version | Description |
|-------|---------|-------------|
| `fusion_finance` | 0.2.0 | Foundation: High-frequency trading primitives with sub-10Î¼s latency. |
| `fusion_finite_fields` | 0.2.0 | Foundation: Panic-free finite field arithmetic primitives for cryptography and quantum computing |
| `fusion_retry` | 0.2.0 | Foundation: Fusion Foundation crate: retry. |
| `fusion_std` | 0.2.0 | Foundation: Fusion standard library extensions and panic-free error handling primitives |
| `fusion_std_ext` | 0.2.0 | Foundation: Fusion Foundation crate: std-ext. |
| `fusion_tensor_sparse` | 0.2.0 | Foundation: Panic-free sparse tensor primitives with O(nnz) complexity guarantees |
| `fusion_ui_component_lib` | 0.2.0 | Foundation: Reusable UI component primitives (buttons, grids, modals). |
| `fusion-core` | 0.2.0 | Foundation: Core type system and abstractions for Fusion. The standard library base. |
| `math-finite-fields` | 0.2.0 | Foundation: Mathematical primitive: finite fields. |
| `math-sparse` | 0.2.0 | Foundation: Mathematical primitive: sparse. |
| `math-sparse-ops` | 0.2.0 | Foundation: Mathematical primitive: sparse ops. |
| `math-tensor-sparse` | 0.2.0 | Foundation: Mathematical primitive: tensor sparse. |

### Algorithm (91 crates)
> **Algorithm crates** implement specific computational methods with documented complexity guarantees.

| Crate | Version | Description |
|-------|---------|-------------|
| `auto-prompt` | 0.2.0 | Algorithm: Fusion Algorithm crate: auto-prompt. |
| `fusion_attention` | 0.2.0 | Algorithm: Multi-head attention with O(nÂ²Â·d) complexity. Optimized for transformer models. |
| `fusion_block` | 0.2.0 | Algorithm: Fusion Algorithm crate: block. |
| `fusion_clustering` | 0.2.0 | Algorithm: K-Means clustering with K-Means++ initialization. O(nÂ·kÂ·i) complexity. |
| `fusion_density_matrix` | 0.2.0 | Algorithm: Quantum density matrix operations. O(2^n) space for n qubits. |
| `fusion_embeddings` | 0.2.0 | Algorithm: Token and positional embeddings for sequence models. |
| `fusion_gate_decomposition` | 0.2.0 | Algorithm: Fusion Algorithm crate: gate-decomposition. |
| `fusion_inference_graph` | 0.2.0 | Algorithm: Fusion Algorithm crate: inference-graph. |
| `fusion_jordan_wigner` | 0.2.0 | Algorithm: Jordan-Wigner transformation for fermions-to-qubits. O(n) qubit mapping. |
| `fusion_llm_attention_mask` | 0.2.0 | Algorithm: Large Language Model component: llm-attention-mask. Optimized for performance. |
| `fusion_llm_cache_compression` | 0.2.0 | Algorithm: Large Language Model component: llm-cache-compression. Optimized for performance. |
| `fusion_llm_cuda_kernel_lib` | 0.2.0 | Algorithm: Optimized CUDA kernels (Attention, RMSNorm, MatMul). |
| `fusion_llm_data_tokenizer` | 0.2.0 | Algorithm: Large Language Model component: llm-data-tokenizer. Optimized for performance. |
| `fusion_llm_distill` | 0.2.0 | Algorithm: Large Language Model component: llm-distillation. Optimized for performance. |
| `fusion_llm_distributed_training` | 0.2.0 | Algorithm: Large Language Model component: llm-distributed-training. Optimized for performance. |
| `fusion_llm_gqa_kernel` | 0.2.0 | Algorithm: Large Language Model component: llm-gqa-kernel. Optimized for performance. |
| `fusion_llm_inference_engine` | 0.2.0 | Algorithm: Large Language Model component: llm-inference. Optimized for performance. |
| `fusion_llm_llama` | 0.2.0 | Algorithm: Llama attention mechanism with RoPE. O(nÂ²Â·d) complexity. |
| `fusion_llm_logits_processor` | 0.2.0 | Algorithm: Large Language Model component: llm-logits-processor. Optimized for performance. |
| `fusion_llm_lora_kernel` | 0.2.0 | Algorithm: Large Language Model component: llm-lora-kernel. Optimized for performance. |
| `fusion_llm_lora_manager` | 0.2.0 | Algorithm: Large Language Model component: llm-lora-manager. Optimized for performance. |
| `fusion_llm_mixtral_routing` | 0.2.0 | Algorithm: Large Language Model component: llm-mixtral-routing. Optimized for performance. |
| `fusion_llm_model_server` | 0.2.0 | Algorithm: Large Language Model component: llm-model-server. Optimized for performance. |
| `fusion_llm_prompt_prefill` | 0.2.0 | Algorithm: Fusion Algorithm crate: prompt-prefill. |
| `fusion_llm_quantization` | 0.2.0 | Algorithm: INT8/INT4/INT2 quantization for LLM weights. O(n) per-layer complexity. |
| `fusion_llm_rag` | 0.2.0 | Algorithm: Large Language Model component: llm-rag. Optimized for performance. |
| `fusion_llm_rerope` | 0.2.0 | Algorithm: Large Language Model component: llm-rerope. Optimized for performance. |
| `fusion_llm_rotary_opt` | 0.2.0 | Algorithm: Large Language Model component: llm-rotary-opt. Optimized for performance. |
| `fusion_llm_stream_parser` | 0.2.0 | Algorithm: Large Language Model component: llm-stream-parser. Optimized for performance. |
| `fusion_llm_tensor_parallel` | 0.2.0 | Algorithm: Fusion Algorithm crate: tensor-parallel. |
| `fusion_llm_tokenizers` | 0.2.0 | Algorithm: LLM tokenizers (BPE, WordPiece, SentencePiece). O(nÂ·log(v)) complexity. |
| `fusion_llm_trie_search` | 0.2.0 | Algorithm: Fusion Algorithm crate: trie-search. |
| `fusion_llm_vision_adapter` | 0.2.0 | Algorithm: Large Language Model component: llm-vision-adapter. Optimized for performance. |
| `fusion_model_server` | 0.2.0 | Algorithm: Fusion Algorithm crate: model-server-core. |
| `fusion_moe_diagnostics` | 0.2.0 | Algorithm: Large Language Model component: llm-moe-tools. Optimized for performance. |
| `fusion_nn_3d_conv` | 0.2.0 | Algorithm: Neural Network layer/module: nn-3d-conv. Optimized for performance. |
| `fusion_nn_attention_block` | 0.2.0 | Algorithm: Attention blocks (Perceiver, Linformer, Flash Attention). O(nÂ²) to O(n) complexity. |
| `fusion_nn_embed` | 0.2.0 | Algorithm: Neural Network layer/module: nn-embed. Optimized for performance. |
| `fusion_nn_gan_layers` | 0.2.0 | Algorithm: Neural Network layer/module: nn-gan-layers. Optimized for performance. |
| `fusion_nn_metrics` | 0.2.0 | Algorithm: Neural Network layer/module: nn-metrics. Optimized for performance. |
| `fusion_nn_rbf` | 0.2.0 | Algorithm: Neural Network layer/module: nn-rbf. Optimized for performance. |
| `fusion_ops` | 0.2.0 | Algorithm: Fusion Algorithm crate: ops. |
| `fusion_pqc_proxy` | 0.2.0 | Algorithm: Fusion Algorithm crate: pqc-proxy. |
| `fusion_q_algo` | 0.2.0 | Algorithm: Quantum algorithm/module: q-algo. Optimized for performance. |
| `fusion_q_aws_backend` | 0.2.0 | Algorithm: Quantum algorithm/module: q-aws-backend. Optimized for performance. |
| `fusion_q_compiler_pass` | 0.2.0 | Algorithm: Quantum compiler optimization passes and gate synthesis. |
| `fusion_q_error_corr` | 0.2.0 | Algorithm: Quantum Error Correction (surface codes, stabilizer). |
| `fusion_q_ibm_backend` | 0.2.0 | Algorithm: Quantum algorithm/module: q-ibm-backend. Optimized for performance. |
| `fusion_q_sim` | 0.2.0 | Algorithm: High-performance Quantum Circuit Simulator. State vector and density matrix modes. |
| `fusion_qaoa` | 0.2.0 | Algorithm: Quantum Approximate Optimization Algorithm with O(pÂ·\|V\|+\|E\|) gate complexity. |
| `fusion_quantum` | 0.2.0 | Algorithm: Fusion Algorithm crate: fusion_quantum. |
| `fusion_quantum_sdk` | 0.2.0 | Algorithm: Fusion Algorithm crate: quantum-sdk. |
| `fusion_qubo` | 0.2.0 | Algorithm: Fusion Algorithm crate: qubo. |
| `fusion_resnet` | 0.2.0 | Algorithm: Fusion Algorithm crate: resnet. |
| `fusion_rl_algorithms` | 0.2.0 | Algorithm: Fusion Algorithm crate: rl-algorithms. |
| `fusion_safetensors` | 0.2.0 | Algorithm: Fusion Algorithm crate: safetensors. |
| `fusion_solver` | 0.2.0 | Algorithm: Fusion Algorithm crate: solver. |
| `fusion_tensor_optim` | 0.2.0 | Algorithm: Fusion Algorithm crate: tensor-optim. |
| `fusion_tokenizers` | 0.2.0 | Algorithm: Fusion Algorithm crate: tokenizers. |
| `fusion_transform` | 0.2.0 | Algorithm: Fusion Algorithm crate: transform. |
| `fusion_tree` | 0.2.0 | Algorithm: Fusion Algorithm crate: tree. |
| `fusion_ui_data_vis` | 0.2.0 | Algorithm: High-performance data visualization (charts, graphs). |
| `fusion_ui_layout_builder` | 0.2.0 | Algorithm: Fusion Algorithm crate: layout-builder. |
| `llm-auto-prompt` | 0.2.0 | Algorithm: Large Language Model component: llm-auto-prompt. Optimized for performance. |
| `llm-beam-search` | 0.2.0 | Algorithm: Beam search decoding for LLMs. O(kÂ·nÂ·v) where k=beam width. |
| `llm-custom-tokenizer` | 0.2.0 | Algorithm: Large Language Model component: llm-custom-tokenizer. Optimized for performance. |
| `llm-distill` | 0.2.0 | Algorithm: Large Language Model component: llm-distill. Optimized for performance. |
| `llm-dynamic-batch` | 0.2.0 | Algorithm: Large Language Model component: llm-dynamic-batch. Optimized for performance. |
| `llm-inference-graph` | 0.2.0 | Algorithm: Large Language Model component: llm-inference-graph. Optimized for performance. |
| `llm-offload` | 0.2.0 | Algorithm: Large Language Model component: llm-offload. Optimized for performance. |
| `llm-prompt-prefill` | 0.2.0 | Algorithm: Large Language Model component: llm-prompt-prefill. Optimized for performance. |
| `llm-prompt-tuning` | 0.2.0 | Algorithm: Large Language Model component: llm-prompt-tuning. Optimized for performance. |
| `llm-rlhf` | 0.2.0 | Algorithm: Large Language Model component: llm-rlhf. Optimized for performance. |
| `llm-tensor-optim` | 0.2.0 | Algorithm: Large Language Model component: llm-tensor-optim. Optimized for performance. |
| `llm-tensor-parallel` | 0.2.0 | Algorithm: Large Language Model component: llm-tensor-parallel. Optimized for performance. |
| `nn-gcn` | 0.2.0 | Algorithm: Neural Network layer/module: nn-gcn. Optimized for performance. |
| `nn-gnn` | 0.2.0 | Algorithm: Neural Network layer/module: nn-gnn. Optimized for performance. |
| `nn-layer-norm` | 0.2.0 | Algorithm: Layer normalization with O(n) complexity. Stabilizes transformer training. |
| `nn-lstm` | 0.2.0 | Algorithm: Long Short-Term Memory (LSTM) with O(4Â·dÂ²) gate operations per timestep. |
| `nn-maxpool` | 0.2.0 | Algorithm: Neural Network layer/module: nn-maxpool. Optimized for performance. |
| `nn-norm` | 0.2.0 | Algorithm: Neural Network layer/module: nn-norm. Optimized for performance. |
| `nn-pooling` | 0.2.0 | Algorithm: Neural Network layer/module: nn-pooling. Optimized for performance. |
| `nn-resnet` | 0.2.0 | Algorithm: ResNet (Residual Network) implementation optimized for Fusion. O(L) depth complexity. |
| `nn-rnn` | 0.2.0 | Algorithm: Neural Network layer/module: nn-rnn. Optimized for performance. |
| `q-error-correction` | 0.2.0 | Algorithm: Quantum algorithm/module: q-error-correction. Optimized for performance. |
| `q-gate-decomposition` | 0.2.0 | Algorithm: Quantum algorithm/module: q-gate-decomposition. Optimized for performance. |
| `q-measurement-opt` | 0.2.0 | Algorithm: Quantum algorithm/module: q-measurement-opt. Optimized for performance. |
| `q-optimizer-hybrid` | 0.2.0 | Algorithm: Quantum algorithm/module: q-optimizer-hybrid. Optimized for performance. |
| `q-pqc-proxy` | 0.2.0 | Algorithm: Quantum algorithm/module: q-pqc-proxy. Optimized for performance. |
| `q-pulse-seq` | 0.2.0 | Algorithm: Quantum algorithm/module: q-pulse-seq. Optimized for performance. |
| `q-visualization` | 0.2.0 | Algorithm: Quantum algorithm/module: q-visualization. Optimized for performance. |

### Integration (27 crates)
> **Integration crates** connect Fusion to external services, languages, and protocols.

| Crate | Version | Description |
|-------|---------|-------------|
| `cloud-aws` | 0.2.0 | Integration: AWS cloud connector with S3, Lambda, and EC2 support |
| `cloud-azure` | 0.2.0 | Integration: Cloud provider connector for AZURE. |
| `cloud-gcp` | 0.2.0 | Integration: Cloud provider connector for GCP. |
| `fusion_api_graphql` | 0.2.0 | Integration: Fusion Integration crate: graphql. |
| `fusion_api_rest_server` | 0.2.0 | Integration: Fusion Integration crate: rest-server. |
| `fusion_bridge_c` | 0.2.0 | Integration: C FFI bridge for Fusion interoperability. |
| `fusion_cuda_interface` | 0.2.0 | Integration: CUDA kernel interface for GPU operations. |
| `fusion_http` | 0.2.0 | Integration: Ergonomic HTTP server and client library with async/blocking support |
| `fusion_interop_cargo_converter` | 0.2.0 | Integration: Converts Rust Cargo metadata to Fusion FFI definitions. |
| `fusion_interop_python_pkg` | 0.2.0 | Integration: Fusion Integration crate: python-converter. |
| `fusion_kv_cache` | 0.2.0 | Integration: Fusion Integration crate: kv-cache. |
| `fusion_net` | 0.2.0 | Integration: Fusion Integration crate: fusion_net. |
| `fusion_python_pkg` | 0.2.0 | Integration: Fusion Integration crate: python-pkg. |
| `fusion_q_cloud_agent` | 0.2.0 | Integration: Cloud provider connector for AGENT. |
| `fusion_sec_trust_anchor` | 0.2.0 | Integration: Fusion Integration crate: trusted-anchor. |
| `fusion_server_grpc` | 0.2.0 | Integration: High-performance gRPC (HTTP/2) server with async/blocking support |
| `fusion_server_wasm` | 0.2.0 | Integration: Fusion Integration crate: wasm-server. |
| `fusion_ui_react` | 0.2.0 | Integration: Fusion Integration crate: react-hooks. |
| `fusion_ui_webasm_renderer` | 0.2.0 | Integration: Fusion Integration crate: webasm-renderer. |
| `fusion_vault` | 0.2.0 | Integration: Fusion Integration crate: vault. |
| `fusion-deploy` | 0.2.0 | Integration: Cloud deployment adapters for AWS, GCP, Azure. |
| `fusion-github` | 0.2.0 | Integration: GitHub API integration and workflow automation. |
| `fusion-k8s-operator` | 0.2.0 | Integration: Fusion Integration crate: k8s-operator. |
| `interop-java` | 0.2.0 | Integration: Language interoperability layer for java. |
| `interop-js` | 0.2.0 | Integration: Language interoperability layer for js. |
| `interop-python` | 0.2.0 | Integration: Language interoperability layer for python. |
| `interop-python-pkgmgr` | 0.2.0 | Integration: Language interoperability layer for python-pkgmgr. |

### Framework (29 crates)
> **Framework crates** provide opinionated, batteries-included platforms for specific domains.

| Crate | Version | Description |
|-------|---------|-------------|
| `flux-resolve-v2-hive-mind` | 2.0.0 | Framework: Distributed dependency resolution with GPU acceleration. |
| `fusion_ai_core` | unknown | Framework: Opinionated AI/ML framework with autodiff, zero-copy tensors, and GPU acceleration |
| `fusion_api_rate_limiter` | 0.2.0 | Framework: Fusion Framework crate: rate-limiter. |
| `fusion_llm_batch_scheduler` | 0.2.0 | Framework: Dynamic batching scheduler for LLM inference. |
| `fusion_llm_gpu_scheduler` | 0.2.0 | Framework: Fusion Framework crate: gpu-scheduler. |
| `fusion_llm_offload` | 0.2.0 | Framework: Fusion Framework crate: offload. |
| `fusion_llm_stream_monitor` | 0.2.0 | Framework: Fusion Framework crate: stream-monitor. |
| `fusion_metrics` | 0.2.0 | Framework: Fusion Framework crate: metrics. |
| `fusion_runtime_core` | unknown | Framework: Opinionated heterogeneous runtime for Quantum/AI/Classical hybrid workloads |
| `fusion_runtime_hal` | 0.2.0 | Framework: Hardware Abstraction Layer (HAL) for Fusion Runtime. |
| `fusion_runtime_mem_mgr` | 0.2.0 | Framework: Memory Manager (GC & Allocator) for Fusion Runtime. |
| `fusion_runtime_scheduler` | 0.2.0 | Framework: Preemptive Task Scheduler for Fusion Runtime. |
| `fusion_safety_monitor` | 0.2.0 | Framework: Fusion Framework crate: safety-monitor. |
| `fusion_sec_policy_engine` | 0.2.0 | Framework: Fusion Framework crate: policy-engine. |
| `fusion_sec_sandbox` | 0.2.0 | Framework: Fusion Framework crate: sandbox-manager. |
| `fusion_server_event_bus` | 0.2.0 | Framework: Asynchronous message queue and event-driven architecture. |
| `fusion_server_faas` | 0.2.0 | Framework: Serverless Function-as-a-Service runtime and emulator. |
| `fusion_server_observability` | 0.2.0 | Framework: Fusion Framework crate: observability. |
| `fusion_server_router_mesh` | 0.2.0 | Framework: Service mesh with dynamic discovery and routing. |
| `fusion_telemetry_ingestor` | 0.2.0 | Framework: Fusion Framework crate: telemetry-ingestor. |
| `fusion_training` | 0.2.0 | Framework: Fusion Framework crate: training. |
| `fusion_vram_scheduler` | 0.2.0 | Framework: Fusion Framework crate: vram-scheduler. |
| `fusion-agentic-core` | 0.1.0 | Framework: Core agentic capabilities for Fusion (Chain-of-Thought, Vibe Coding). |
| `fusion-agents` | 1.6 | Framework: Multi-agent orchestration framework for parallel AI workflows. |
| `fusion-ai-core` | 0.2.0 | Framework: AI infrastructure core with adapters and safety checks. |
| `fusion-ai-models` | 0.2.0 | Framework: Local model runners (llama.cpp, ONNX) with unified interface. |
| `fusion-mcp` | 1.0 | Framework: Opinionated Model Context Protocol (MCP) framework for AI agent orchestration |
| `fusion-profiler` | 0.2.0 | Framework: Fusion Framework crate: profiler. |
| `sentinael-tribrid` | 1.0.0 | Framework: Fusion Framework crate: sentinel-tribrid. |

### Tool (85 crates)
> **Tool crates** are CLI utilities and development tools with excellent error reporting.

| Crate | Version | Description |
|-------|---------|-------------|
| `fusion_api_schema_validator` | 0.2.0 | Tool: Fusion Tool crate: schema-validator. |
| `fusion_api_sdk_generator` | 0.2.0 | Tool: Fusion Tool crate: sdk-generator. |
| `fusion_auth` | 0.2.0 | Tool: Fusion Tool crate: auth. |
| `fusion_crate_analyzer` | 0.2.0 | Tool: Static analysis and dependency graph tool for Fusion crates. |
| `fusion_llm_custom_tokenizer` | 0.2.0 | Tool: BPE/SentencePiece training utility for custom vocabularies. |
| `fusion_sbom_generator` | 0.2.0 | Tool: Fusion Tool crate: sbom-generator. |
| `fusion_sec_forensics` | 0.2.0 | Tool: Security tool: sec-forensics. Excellent error reporting. |
| `fusion_sec_penetration` | 0.2.0 | Tool: Security tool: sec-penetration. Excellent error reporting. |
| `fusion_sec_supply_chain` | 0.2.0 | Tool: Fusion Tool crate: supply-chain. |
| `fusion_toolchain_ext` | 0.2.0 | Tool: Fusion Tool crate: toolchain-ext. |
| `fusion_version` | 0.2.0 | Tool: Fusion Tool crate: version. |
| `fusion-ai-cli` | 0.2.0 | Tool: AI-powered CLI with subcommands and workspace management. |
| `fusion-ai-cli-enhanced` | 0.1.0 | Tool: Fusion tool: fusion-ai-cli-enhanced. Excellent error reporting. |
| `fusion-ai-daemon` | 0.2.0 | Tool: Background daemon for heavy LLM inference workloads. |
| `fusion-audio` | 0.2.0 | Tool: Fusion tool: fusion-audio. Excellent error reporting. |
| `fusion-audit` | 0.2.0 | Tool: Fusion Tool crate: audit. |
| `fusion-blockchain` | 0.2.0 | Tool: Fusion tool: fusion-blockchain. Excellent error reporting. |
| `fusion-calendar` | 0.2.0 | Tool: Fusion tool: fusion-calendar. Excellent error reporting. |
| `fusion-charts` | 0.2.0 | Tool: Fusion tool: fusion-charts. Excellent error reporting. |
| `fusion-clustering` | 0.2.0 | Tool: Fusion tool: fusion-clustering. Excellent error reporting. |
| `fusion-component-lib` | 0.2.0 | Tool: Fusion tool: fusion-component-lib. Excellent error reporting. |
| `fusion-compression` | 0.2.0 | Tool: Fusion tool: fusion-compression. Excellent error reporting. |
| `fusion-crate-analyzer` | 0.2.0 | Tool: Fusion tool: fusion-crate-analyzer. Excellent error reporting. |
| `fusion-cryptography` | 0.2.0 | Tool: Fusion tool: fusion-cryptography. Excellent error reporting. |
| `fusion-cuda-kernel` | 0.2.0 | Tool: Fusion tool: fusion-cuda-kernel. Excellent error reporting. |
| `fusion-database` | 0.2.0 | Tool: Fusion tool: fusion-database. Excellent error reporting. |
| `fusion-data-vis` | 0.2.0 | Tool: Fusion tool: fusion-data-vis. Excellent error reporting. |
| `fusion-debugger` | 0.2.0 | Tool: Debug Adapter Protocol (DAP) implementation for Fusion. |
| `fusion-diagnostics` | 0.2.0 | Tool: Fusion tool: fusion-diagnostics. Excellent error reporting. |
| `fusion-distributed-training` | 0.2.0 | Tool: Fusion tool: fusion-distributed-training. Excellent error reporting. |
| `fusion-docgen` | 0.2.0 | Tool: Documentation generator with search indexing. |
| `fusion-event-bus` | 0.2.0 | Tool: Fusion tool: fusion-event-bus. Excellent error reporting. |
| `fusion-faas` | 0.2.0 | Tool: Fusion tool: fusion-faas. Excellent error reporting. |
| `fusion-formatter` | 0.2.0 | Tool: Fusion Tool crate: formatter. |
| `fusion-fuzz-harness` | 0.2.0 | Tool: Fusion tool: fusion-fuzz-harness. Excellent error reporting. |
| `fusion-geo` | 0.2.0 | Tool: Fusion tool: fusion-geo. Excellent error reporting. |
| `fusion-gpu-scheduler` | 0.2.0 | Tool: Fusion tool: fusion-gpu-scheduler. Excellent error reporting. |
| `fusion-graphql` | 0.2.0 | Tool: Fusion tool: fusion-graphql. Excellent error reporting. |
| `fusion-id-provider` | 0.2.0 | Tool: Fusion tool: fusion-id-provider. Excellent error reporting. |
| `fusion-image` | 0.2.0 | Tool: Fusion tool: fusion-image. Excellent error reporting. |
| `fusion-iot` | 0.2.0 | Tool: Fusion tool: fusion-iot. Excellent error reporting. |
| `fusion-layout-builder` | 0.2.0 | Tool: Fusion tool: fusion-layout-builder. Excellent error reporting. |
| `fusion-mail` | 0.2.0 | Tool: Fusion tool: fusion-mail. Excellent error reporting. |
| `fusion-math` | 0.2.0 | Tool: Fusion tool: fusion-math. Excellent error reporting. |
| `fusion-observability` | 0.2.0 | Tool: Fusion tool: fusion-observability. Excellent error reporting. |
| `fusion-optimization` | 0.2.0 | Tool: Fusion tool: fusion-optimization. Excellent error reporting. |
| `fusion-physics` | 0.2.0 | Tool: Fusion tool: fusion-physics. Excellent error reporting. |
| `fusion-rate-limiter` | 0.2.0 | Tool: Fusion tool: fusion-rate-limiter. Excellent error reporting. |
| `fusion-react-bridge` | 0.2.0 | Tool: Fusion tool: fusion-react-bridge. Excellent error reporting. |
| `fusion-redis` | 0.2.0 | Tool: Fusion tool: fusion-redis. Excellent error reporting. |
| `fusion-regex` | 0.2.0 | Tool: Fusion tool: fusion-regex. Excellent error reporting. |
| `fusion-rest-server` | 0.2.0 | Tool: Fusion tool: fusion-rest-server. Excellent error reporting. |
| `fusion-router-mesh` | 0.2.0 | Tool: Fusion tool: fusion-router-mesh. Excellent error reporting. |
| `fusion-runtime-core-v2-nebula` | 2.0.0 | Tool: Fusion tool: fusion-runtime-core-v2-nebula. Excellent error reporting. |
| `fusion-safety-monitor` | 0.2.0 | Tool: Fusion tool: fusion-safety-monitor. Excellent error reporting. |
| `fusion-sandbox` | 0.2.0 | Tool: Fusion tool: fusion-sandbox. Excellent error reporting. |
| `fusion-sbom-generator` | 0.2.0 | Tool: Development tool: fusion-sbom-generator. Excellent error reporting. |
| `fusion-schema-validator` | 0.2.0 | Tool: Development tool: fusion-schema-validator. Excellent error reporting. |
| `fusion-sdk-generator` | 0.2.0 | Tool: Development tool: fusion-sdk-generator. Excellent error reporting. |
| `fusion-service-router` | 0.2.0 | Tool: Fusion tool: fusion-service-router. Excellent error reporting. |
| `fusion-stream-monitor` | 0.2.0 | Tool: Fusion tool: fusion-stream-monitor. Excellent error reporting. |
| `fusion-supply-chain` | 0.2.0 | Tool: Fusion tool: fusion-supply-chain. Excellent error reporting. |
| `fusion-telemetry` | 0.2.0 | Tool: Fusion tool: fusion-telemetry. Excellent error reporting. |
| `fusion-terminal-browser` | 0.2.0 | Tool: Fusion tool: fusion-terminal-browser. Excellent error reporting. |
| `fusion-tester` | 0.2.0 | Tool: Fusion Tool crate: tester. |
| `fusion-trie-search` | 0.2.0 | Tool: Fusion tool: fusion-trie-search. Excellent error reporting. |
| `fusion-ui` | 0.2.0 | Tool: Fusion tool: fusion-ui. Excellent error reporting. |
| `fusion-video` | 0.2.0 | Tool: Fusion tool: fusion-video. Excellent error reporting. |
| `fusion-vision` | 0.2.0 | Tool: Fusion tool: fusion-vision. Excellent error reporting. |
| `fusion-vram-scheduler` | 0.2.0 | Tool: Fusion tool: fusion-vram-scheduler. Excellent error reporting. |
| `fusion-wasm-runtime` | 0.2.0 | Tool: Fusion tool: fusion-wasm-runtime. Excellent error reporting. |
| `fusion-wasm-server` | 0.2.0 | Tool: Fusion tool: fusion-wasm-server. Excellent error reporting. |
| `fusion-webasm-renderer` | 0.2.0 | Tool: Fusion tool: fusion-webasm-renderer. Excellent error reporting. |
| `fusion-web-server` | 0.2.0 | Tool: Fusion tool: fusion-web-server. Excellent error reporting. |
| `fusion-xml` | 0.2.0 | Tool: Fusion tool: fusion-xml. Excellent error reporting. |
| `fusion-yaml` | 0.2.0 | Tool: Fusion tool: fusion-yaml. Excellent error reporting. |
| `sec-incident-response` | 0.2.0 | Tool: Security tool: sec-incident-response. Excellent error reporting. |
| `sec-network-segmentation` | 0.2.0 | Tool: Security tool: sec-network-segmentation. Excellent error reporting. |
| `sec-os-hardener` | 0.2.0 | Tool: Security tool: sec-os-hardener. Excellent error reporting. |
| `sec-policy-compiler` | 0.2.0 | Tool: Security tool: sec-policy-compiler. Excellent error reporting. |
| `sec-policy-engine` | 0.2.0 | Tool: Security tool: sec-policy-engine. Excellent error reporting. |
| `sec-runtime-policy` | 0.2.0 | Tool: Security tool: sec-runtime-policy. Excellent error reporting. |
| `sec-secrets-auditor` | 0.2.0 | Tool: Security tool: sec-secrets-auditor. Excellent error reporting. |
| `sec-threat-intel` | 0.2.0 | Tool: Security tool: sec-threat-intel. Excellent error reporting. |
| `sec-trusted-anchor` | 0.2.0 | Tool: Security tool: sec-trusted-anchor. Excellent error reporting. |

### Experimental (6 crates)
| Crate | Version | Description |
|-------|---------|-------------|
| `fusion_carver` | 0.2.0 | Experimental: EXPERIMENTAL: Code carving and extraction utilities. |
| `fusion_client` | 0.2.0 | Experimental: EXPERIMENTAL: Generic client interface. |
| `fusion_executor` | 0.2.0 | Experimental: EXPERIMENTAL: Task execution engine. |
| `fusion_experimental_diagnostics` | 0.2.0 | Experimental: EXPERIMENTAL: Mixture-of-Experts diagnostics and profiling. |
| `fusion_graph` | 0.2.0 | Experimental: EXPERIMENTAL: Graph data structures and algorithms. |
| `haft_fusion` | 0.2.0 | Experimental: EXPERIMENTAL: Research prototype. |

---

## Quick Reference

### Core Infrastructure
- **usion_core**: Foundation type system
- **usion_std**: Standard library extensions
- **usion_runtime_core**: Heterogeneous runtime
- **usion_ai_core**: AI/ML framework

### Quantum Computing
- **usion_quantum**: Quantum primitives
- **q-sim**: Quantum circuit simulator
- **qaoa**: Quantum optimization algorithm
- **q-error-correction**: Error correction codes

### Neural Networks
- **
n-lstm**: LSTM layers
- **
n-attention-block**: Attention mechanisms
- **
n-layer-norm**: Layer normalization
- **esnet**: ResNet implementation

### Large Language Models
- **llm-tokenizers**: BPE/WordPiece tokenizers
- **llm-quantization**: Model quantization
- **llm-beam-search**: Beam search decoding
- **llm-rag**: Retrieval-Augmented Generation

### Cloud Integration
- **cloud-aws**: AWS connector
- **cloud-gcp**: Google Cloud connector
- **cloud-azure**: Azure connector

### Security Tools
- **sec-penetration**: Penetration testing
- **sec-forensics**: Security forensics
- **sec-policy-engine**: Policy enforcement

---

## Getting Started

### Using a Crate

Add to your Cargo.toml:

`	oml
[dependencies]
fusion_core = { workspace = true }
# Or specify version if not in Fusion workspace
fusion_ai_core = "0.2.0"
`

### Building from Source

`ash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p fusion_runtime_core

# Run tests
cargo test --workspace
`

### Documentation

Generate docs for all crates:

`ash
cargo doc --workspace --no-deps --open
`

---

**For detailed crate documentation, see individual README.md files in each crate directory.**
