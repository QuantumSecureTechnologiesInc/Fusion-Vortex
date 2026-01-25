# MCP Server

**Dataset Category**: Tooling
**Training Level**: Intermediate
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

The MCP server is the AI context service. It exposes runtime state and build metadata via JSON-RPC over stdio.

## Example

```fusion
fn serve() {
    let request = Json::parse(Stdio::read_line());
    match request.method {
        "get_runtime_state" => Stdio::write_json(Kernel::snapshot()),
        _ => Stdio::error("Unknown method"),
    }
}
```text