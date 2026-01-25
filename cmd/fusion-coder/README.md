# Fusion VSC CLI Coder

Advanced agent orchestration CLI combining the best of:
- **Antigravity IDE**: Planning/Fast modes, Task Groups, continuous context
- **Claude Code**: Hierarchical settings, permissions, hooks
- **Codex**: Interactive workflows, resume, code review

## Project Structure

```text
cmd/fusion-coder/          # Main CLI binary
crates/
  ├── fusion-agent-core/    # Agent orchestration engine
  ├── fusion-task-groups/   # Task breakdown & progress tracking
  ├── fusion-review-policy/ # Review policy enforcement
  ├── fusion-browser-agent/ # Browser subagent
  └── fusion-settings/      # Hierarchical settings management
```text

## Features

### Agent Modes

- **Planning Mode**: Deep research, task groups, artifacts
- **Fast Mode**: Direct execution for simple tasks

### Review Policies

- Artifact review (AlwaysProceed / RequestReview)
- Terminal command approval with allow/deny lists
- Browser navigation & JavaScript execution control

### Interactive Features

- Full-screen TUI with live notifications
- Resume previous sessions
- Code review presets
- Non-interactive exec mode

## Quick Start

```bash

# Build

cargo build --release -p fusion-coder

# Run interactive mode (planning)

./target/release/fusion-coder

# Run in fast mode

./target/release/fusion-coder --mode fast

# Enable secure mode

./target/release/fusion-coder --secure

# Execute task non-interactively

./target/release/fusion-coder exec "analyze codebase"
```text

## Development

This project is part of the Fusion Programming Language ecosystem.

## License

MIT OR Apache-2.0