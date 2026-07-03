> **Phase 0 audit (2026-06-25):** `dist/` and `Source Files/` directories no longer exist after restructure.
> Rows referencing those paths are marked below. Treat this as a target layout, not current state.

# Guidebook Feature Audit

This audit checks guidebook-level features against concrete repo artifacts (paths, binaries, or sysroot installs).

| Category | Feature | Status | Evidence |
| --- | --- | --- | --- |
| Compiler | Fusion compiler (fuc) | Implemented | `crates/fuc/` |
| Toolchain | Fusion build driver (fusion) | Implemented | `cmd/fusion/` |
| Stdlib | Fusion stdlib sources | Implemented | `stdlib/`, `src/stdlib/` |
| Stdlib | Stdlib installed in sysroot | **Not Built** | `dist/lib/fusion/std` — *dist/ does not exist* |
| Runtime | C runtime layer | Implemented | `runtime/native/fusionrt.c` |
| Borrow checker | Entropic Borrow Checker (Vortex) | **Archived** | *was `Source Files/` — directory removed* |
| Memory | ARC subsystem | **Archived** | *was `Source Files/` — directory removed* |
| MCP | MCP server implementation | Implemented | `src/mcp/` |
| HAFT | HAFT Mesh Nodes v1.0 sources | **Archived** | *was `Source Files/` — directory removed* |
| HAFT | HAFT binaries packaged | **Not Built** | `dist/haft_nodes` — *dist/ does not exist* |
| Runtime | Core Runtime Evolution sources | **Archived** | *was `Source Files/` — directory removed* |
| Runtime | Core Runtime Evolution packaged | **Not Built** | `dist/lib/fusion/core_runtime_evolution` — *dist/ does not exist* |
| Forge | Fusion Forge build system | **Archived** | *was `Source Files/` — directory removed* |
| Interop | Interop layer converted to .fu | Implemented | `toolchain/interop/` |
| Integrations | Integrations inventory | Implemented | `docs/ecosystem/Integrations_Inventory.md` |
| Quantum | IBM/AWS bracket backends | **Aspirational** | `registry/crates/q-ibm-backend`, `registry/crates/q-aws-backend` — *most registry crates are empty stubs* |
| AI | LLM providers (Ollama/Qwen/DeepSeek) | **Aspirational** | `registry/crates/llm-model-server` — *most registry crates are empty stubs* |
| UI | Fusion TUI | **Archived** | *was `Source Files/Fusion TUI` — directory removed* |
| Monolith | Fusion Unified Monolith | Partial | `crates/fusion-monolith-core/` exists |

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
