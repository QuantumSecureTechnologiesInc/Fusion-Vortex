# MCP Protocol

**Dataset Category**: Advanced Features
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion MCP exposes runtime context to LLMs via JSON-RPC over stdio. It enables AI assistants to query state, metrics, and diagnostics.

## Example

```fusion
fn serve() {
    let req = Json::parse(Stdio::read_line());
    if req.method == "get_runtime_state" { Stdio::write_json(Kernel::snapshot()); }
}
```text