# Guidebook Feature Audit

This audit checks guidebook-level features against concrete repo artifacts (paths, binaries, or sysroot installs).

| Category | Feature | Status | Evidence |
| --- | --- | --- | --- |
| Compiler | Fusion compiler (fuc) | Implemented | crates/fuc, dist/bin/fuc |
| Toolchain | Fusion build driver (fusion) | Implemented | cmd/fusion, dist/bin/fusion |
| Stdlib | Fusion stdlib sources | Implemented | registry/crates/std, src/stdlib |
| Stdlib | Stdlib installed in sysroot | Implemented | dist/lib/fusion/std |
| Runtime | C runtime layer | Implemented | runtime/runtime.c, dist/lib/fusion/runtime.o |
| Borrow checker | Entropic Borrow Checker (Vortex) | Implemented | Source Files/Fusion Entropic Borrow Checker |
| Memory | ARC subsystem | Implemented | Source Files/Automatic Reference Counting (ARC) |
| MCP | MCP server implementation | Implemented | src/mcp, registry/crates/mcp |
| HAFT | HAFT Mesh Nodes v1.0 sources | Implemented | Source Files/HAFT Mesh Nodes v1.0 |
| HAFT | HAFT binaries packaged | Implemented | dist/haft_nodes |
| Runtime | Core Runtime Evolution sources | Implemented | Source Files/The Core Runtime Evolution |
| Runtime | Core Runtime Evolution packaged | Implemented | dist/lib/fusion/core_runtime_evolution |
| Forge | Fusion Forge build system | Implemented | Source Files/Fusion Forge (The Build System), dist/lib/fusion/forge |
| Interop | Interop layer converted to .fu | Implemented | toolchain/interop |
| Integrations | Integrations inventory | Implemented | Source Files/Intergrations |
| Quantum | IBM/AWS bracket backends | Implemented | registry/crates/q-ibm-backend, registry/crates/q-aws-backend |
| AI | LLM providers (Ollama/Qwen/DeepSeek) | Implemented | registry/crates/llm-model-server, registry/crates/interop-js, registry/crates/interop-python |
| UI | Fusion TUI | Implemented | Source Files/Fusion TUI, registry/crates/fusion-terminal-browser |
| Monolith | Fusion Unified Monolith | Implemented | Source Files/Fusion Unified  Monolith, registry/crates/fusion-monolith-core |

## Notes

- Status is **Implemented** only if all expected paths exist; **Partial** if some exist; **Missing** if none exist.
- This audit is structural (artifact-based). A deeper behavioral audit would require running tests for each subsystem.

## Behavioral Verification

Ran the standard compiler/runtime/security gates and fixture suite:

- `scripts/run_fuc_fixtures.sh` → PASS (expected abort in `safety_and_repeat` due to bounds panic)
- `scripts/compiler_gates.sh` → PASS
- `scripts/runtime_gates.sh` → PASS (expected abort in `safety_and_repeat`)
- `scripts/security_gate.sh` → PASS
- `scripts/ci_gate.sh` → PASS (aggregates the above)

Notes:
- The aborts in `safety_and_repeat` are intentional and confirm runtime bounds checks/panic flow.
