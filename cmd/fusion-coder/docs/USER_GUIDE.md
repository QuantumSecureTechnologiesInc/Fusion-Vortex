<!--
Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
SPDX-License-Identifier: MIT OR Apache-2.0

This file is part of Fusion VSC CLI Coder
-->

# Fusion VSC CLI Coder - User Guide

## Overview

Fusion VSC CLI Coder is an advanced agent orchestration CLI that combines the best features of Antigravity IDE, Claude Code, and Codex to provide intelligent coding assistance.

## Quick Start

```bash

# Start in interactive mode (default: Planning)

fusion-coder

# Start in Fast mode

fusion-coder --mode fast

# Enable secure mode

fusion-coder --secure

# Specify workspace

fusion-coder --path /path/to/project
```text

## Agent Modes

### Planning Mode (Default)

**Best for**: Complex tasks, new features, refactoring

- Deep research and analysis
- Task breakdown into subtasks
- Artifact generation (plans, walkthroughs)
- Comprehensive testing

```bash
fusion-coder --mode planning
```text

### Fast Mode

**Best for**: Simple tasks, quick fixes, direct execution

- Immediate execution
- No task breakdown
- Minimal artifacts
- Quick responses

```bash
fusion-coder --mode fast
```text

## Secure Mode

Enhanced security with strict enforcement:

```bash
fusion-coder --secure
```text

**Features**:
- ✅ Workspace isolation (no access outside project)
- ✅ URL allowlist/denylist
- ✅ .gitignore respect
- ✅ Force review for all actions

## Commands

### Interactive Mode (Default)

Full-screen TUI with live updates:

```bash
fusion-coder
```text

### Resume Session

Continue a previous session:

```bash

# Pick from list

fusion-coder resume

# Resume last session

fusion-coder resume --last

# Resume specific session

fusion-coder resume <session-id>
```text

### Exec Mode

Non-interactive execution for automation:

```bash
fusion-coder exec "implement user authentication"

# JSON output for scripts

fusion-coder exec "run tests" --json
```text

### Shell Completions

```bash

# Bash

fusion-coder completion bash > /etc/bash_completion.d/fusion-coder

# Zsh

fusion-coder completion zsh > ~/.zsh/completion/_fusion-coder

# Fish

fusion-coder completion fish > ~/.config/fish/completions/fusion-coder.fish
```text

## Review Policies

Control what requires approval:

### Artifact Policy

- `AlwaysProceed`: Generate artifacts without review
- `RequestReview`: Ask before creating artifacts

### Terminal Policy

- Allow list: Commands that auto-execute
- Deny list: Commands that are blocked
- Safe readonly commands: `ls`, `cat`, `grep`, `git status`

### Browser Policy

- URL allowlist: Allowed domains
- URL denylist: Blocked domains
- JavaScript execution: Requires review by default

## Task Groups

In Planning mode, work is organized into task groups:

**Structure**:
- Goal: High-level objective
- Summary: Current progress
- Subtasks: Individual steps
- Edited Files: Tracked changes
- Pending Steps: Actions awaiting approval

**Status Indicators**:
- ○ Pending
- ⟳ In Progress
- ✓ Complete
- ✗ Failed

## Continuous Context

Send messages while agent is working:

**In TUI**: Type message and press Enter
**Via Flag**: `--send-to-agent "update"`

Agent will:
- Receive message during execution
- Adjust approach based on feedback
- Respond in conversation flow

## Tips

1. **Use Planning mode** for anything new or complex
2. **Use Fast mode** for quick fixes and simple tasks
3. **Enable secure mode** when working on sensitive projects
4. **Review policies** can be customized per project
5. **Resume sessions** to continue work later
6. **Exec mode** great for CI/CD integration

## Examples

```bash

# Complex feature in Planning mode with secure

fusion-coder --mode planning --secure
> "Implement OAuth2 authentication with tests"

# Quick fix in Fast mode

fusion-coder --mode fast
> "Fix typo in README"

# Resume yesterday's work

fusion-coder resume --last

# Automated testing in CI

fusion-coder exec "run all tests and generate coverage" --json
```text

## Configuration

Settings are loaded with precedence:
1. Enterprise settings (highest)
2. CLI arguments
3. Local project (`.fusion-coder/settings.json`)
4. Shared project (`fusion-coder.json`)
5. User global (`~/.fusion-coder/settings.json`)

See Settings Guide for details.

---

**Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team**