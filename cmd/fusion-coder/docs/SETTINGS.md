<!--
Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
SPDX-License-Identifier: MIT OR Apache-2.0

This file is part of Fusion VSC CLI Coder
-->

# Settings Architecture

Fusion VSC CLI Coder uses a hierarchical settings system inspired by Claude Code.

## Precedence Order

Settings are merged with the following precedence (highest to lowest):

1. **Enterprise** - Organization-wide policies
2. **CLI Arguments** - Command-line flags
3. **Local Project** - `.fusion-coder/settings.json` (gitignored)
4. **Shared Project** - `fusion-coder.json` (checked in)
5. **User Global** - `~/.fusion-coder/settings.json`

## Settings File Locations

### User Settings

`~/.fusion-coder/settings.json`

```json
{
  "agent": {
    "default_mode": "planning"
  },
  "permissions": {
    "allow": ["git*", "cargo check"],
    "deny": ["rm", "sudo"]
  }
}
```text

### Project Settings (Shared)

`<project>/fusion-coder.json`

```json
{
  "agent": {
    "default_mode": "fast"
  },
  "permissions": {
    "allow": ["cargo*", "npm*"]
  }
}
```text

### Local Project Settings

`<project>/.fusion-coder/settings.json` (add to `.gitignore`)

```json
{
  "env": {
    "FUSION_CODER_API_KEY": "sk-..."
  }
}
```text

## Available Settings

### Agent Configuration

```json
{
  "agent": {
    "default_mode": "planning",
    "reasoning_enabled": true
  }
}
```text

### Permissions

```json
{
  "permissions": {
    "allow": ["git*", "cargo check"],
    "ask": ["cargo build"],
    "deny": ["rm", "sudo -rf"]
  }
}
```text

### Sandbox Settings

```json
{
  "sandbox": {
    "enabled": true,
    "auto_allow_if_sandboxed": true,
    "excluded_commands": ["git"],
    "network": {
      "allow_unix_sockets": true
    }
  }
}
```text

### Hooks

```json
{
  "hooks": {
    "session_start": ["git status"],
    "pre_tool_use": [],
    "post_tool_use": ["git add ."]
  }
}
```text

### MCP Servers

```json
{
  "mcp": {
    "enable_all_project": false,
    "allowed_servers": ["github", "filesystem"],
    "denied_servers": []
  }
}
```text

### Other Settings

```json
{
  "model": "gpt-4",
  "status_line": true,
  "output_style": "verbose",
  "attribution": {
    "commit_messages": true,
    "pr_descriptions": true
  }
}
```text

## Environment Variables

Managed via settings files:

```json
{
  "env": {
    "FUSION_CODER_TELEMETRY": "false",
    "RUST_BACKTRACE": "1"
  }
}
```text

## Glob Patterns

Permissions support glob patterns:
- `git*` - Matches `git status`, `git diff`, etc.
- `cargo check*` - Matches `cargo check`, `cargo check --all`, etc.
- `**/*.rs` - Matches all Rust files

## Example: Team Configuration

**Project root: `fusion-coder.json`**

```json
{
  "agent": {
    "default_mode": "planning"
  },
  "permissions": {
    "allow": [
      "git status",
      "git diff",
      "cargo check",
      "cargo test"
    ],
    "deny": [
      "cargo publish",
      "git push --force"
    ]
  },
  "hooks": {
    "session_start": ["git fetch"]
  }
}
```text

**User override: `~/.fusion-coder/settings.json`**

```json
{
  "agent": {
    "default_mode": "fast"
  },
  "model": "gpt-4-turbo"
}
```text

Result: User gets Fast mode by default, but team permissions still apply.

---

**Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team**