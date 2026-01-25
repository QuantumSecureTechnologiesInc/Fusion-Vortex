# Fusion VSC CLI Coder - Training Manual & User Guide

**Version**: 1.0
**Date**: December 2024
**Author**: Fusion Programming Language Team

---

## 1. Introduction

Welcome to the **Fusion VSC CLI Coder**, a production-ready advanced agent orchestration CLI designed to revolutionize how you interact with your codebase. By combining the best features of:
* **Antigravity IDE** (Planning/Fast modes, Task Groups)
* **Claude Code** (Hierarchical permissions, Hooks)
* **Codex** (Interactive workflows, Resume capabilities)

...the Fusion VSC CLI Coder provides an intelligent, secure, and highly capable coding assistant directly in your terminal.

### Key Capabilities

* **Dual Agent Modes**: Switch between deep "Planning" for complex tasks and "Fast" for quick fixes.
* **Task Groups**: Automatically break down large goals into tracked subtasks.
* **Continuous Context**: Interact with the agent *while* it works, adjusting course in real-time.
* **Secure Execution**: Isolated workspaces, URL allowlists, and strict policy enforcement.
* **Session Management**: Pause and resume your work exactly where you left off.

---

## 2. Getting Started

### Prerequisites

* **Fusion Toolchain**: Ensure you have the toolchain installed (`./install.sh`).
* **Fusion Workspace**: Access to the Fusion Programming Language project.

### Installation

Build the CLI binary from the project root:

```bash
fusion build --release -p fusion-coder
```text

The binary will be available at `./target/release/fusion-coder`.

### Quick Start

1. **Launch Interactive Mode** (Default: Planning Mode)

    ```bash
    ./target/release/fusion-coder
```text

2. **Launch Fast Mode** (For quick tasks)

    ```bash
    ./target/release/fusion-coder --mode fast
```text

3. **Launch in specific directory**

    ```bash
    ./target/release/fusion-coder --path /path/to/your/project
```text

---

## 3. Core Concepts: Agent Modes

Understanding the two modes is crucial for efficient usage.

### 🧠 Planning Mode (The Architect)

**Best for**: Complex features, refactoring, research, and new projects.

* **Behavior**: The agent acts as a methodical architect. It analyzes the request, researches files, breaks the work into specific "Task Groups", and executes them step-by-step.
* **Artifacts**: Generates plans, implementation guides, and walkthroughs.
* **UI**: Displays a detailed collapsible view of tasks and progress.

### ⚡ Fast Mode (The Mechanic)

**Best for**: Quick bug fixes, typo corrections, and simple file edits.

* **Behavior**: The agent acts as a swift mechanic. It jumps directly to the solution with minimal overhead.
* **Speed**: Skips detailed task breakdowns and artifacts to deliver immediate results.
* **UI**: Simplified view focused on the immediate action.

---

## 4. Features & Workflows

### Task Groups (Planning Mode)

When you give a complex instruction in Planning Mode, the agent organizes work into **Task Groups**.

* **Goal**: The high-level objective (e.g., "Implement User Auth").
* **Subtasks**: Individual steps (e.g., "Create Login Component", "Add Route").
* **Status Indicators**:
    * `○` : Pending
    * `⟳` : In Progress
    * `✓` : Complete
    * `✗` : Failed

### Continuous Context

You don't have to wait for the agent to finish! You can send messages *during* execution.

* **How**: Type in the TUI input box and press Enter.
* **Why**: To correct a misunderstanding, add new information, or change priority mid-flight.
* **Result**: The agent receives your "Interrupt" message effectively injecting it into its context stream immediately.

### Session Management

Your work is saved automatically. You can close the terminal and resume later.

* **Resume Last Session**:

    ```bash
    fusion-coder resume --last
```text

* **Pick from History**:

    ```bash
    fusion-coder resume
```text

### Non-Interactive Execution ("Exec Mode")

Perfect for CI/CD pipelines or scripting.

```bash
fusion-coder exec "run tests and fix lint errors" --json
```text

---

## 5. Security & Review Policies

Fusion VSC CLI Coder is built with security as a priority.

### Secure Mode

Run with `--secure` for strict isolation.

```bash
fusion-coder --secure
```text

* **Workspace Isolation**: The agent simply **cannot** access files outside the project directory.
* **Gitignore**: Respects `.gitignore` to avoid reading secrets or build artifacts.
* **Force Review**: All modifications require manual approval.

### Review Policies

Control when the agent needs your permission.

1. **Terminal Policy**:
    * **Allow List**: Commands like `ls`, `git status` run automatically.
    * **Deny List**: Destructive commands like `rm -rf` are blocked.
    * **Ask**: Everything else prompts you Y/N.

2. **Browser Policy**:
    * Controls which URLs the agent can visit.
    * Controls if the agent can execute JavaScript on pages.

3. **Artifact Policy**:
    * `AlwaysProceed`: Create files immediately.
    * `RequestReview`: Ask before writing documentation artifacts.

---

## 6. Configuration

Settings are hierarchical, meaning a specific project setting overrides a global user setting.

### Hierarchy (Highest to Lowest)

1. **Enterprise Policies** (Organization enforced)
2. **CLI Arguments** (Flags passed at runtime)
3. **Local Project** (`.fusion-coder/settings.json` - gitignored)
4. **Shared Project** (`fusion-coder.json` - committed to repo)
5. **User Global** (`~/.fusion-coder/settings.json`)

### Example: `fusion-coder.json`

```json
{
  "agent": {
    "default_mode": "planning"
  },
  "permissions": {
    "allow": ["git status", "fusion check"],
    "deny": ["fusion publish"]
  },
  "hooks": {
    "session_start": ["git fetch"]
  }
}
```text

---

## 7. Command Reference

| Command                           | Description                                       |
| :-------------------------------- | :------------------------------------------------ |
| `fusion-coder`                    | Start interactive session (default).              |
| `fusion-coder --mode <mode>`      | Start in `planning` or `fast` mode.               |
| `fusion-coder --secure`           | Enable strict security sandbox.                   |
| `fusion-coder resume --last`      | Resume the most recent session.                   |
| `fusion-coder exec "<prompt>"`    | Run a single prompt and exit.                     |
| `fusion-coder completion <shell>` | Generate shell completion script (bash/zsh/fish). |

---

## 8. Troubleshooting

**Q: The agent seems stuck planning.**
A: Switch to **Fast Mode** (`--mode fast`) if the task is simple. Planning mode is designed to be thorough.

**Q: I can't run a specific command.**
A: Check your `fusion-coder.json` permissions. You may need to add it to the `allow` list or check if it's in the `deny` list.

**Q: The agent isn't seeing my files.**
A: Ensure you are running from the correct root directory or use the `--path` flag. Also check if `.gitignore` is hiding them in Secure Mode.

---

**Copyright (c) 2024 QuantumSecure Technologies Inc**
Fusion Programming Language Team
