# Fusion VS Code Extension - Release Notes

## Version 0.1.0 - Initial Release

**Release Date**: 2025-12-07

### Features

#### Syntax Highlighting
- Complete TextMate grammar for Fusion language
- Keywords: `fn`, `class`, `pub`, `mod`, `use`, `let`, `if`, `else`, `while`, `for`, `return`
- Type annotations: `int`, `float`, `bool`, `string`, custom types
- Operators, literals, comments, and punctuation
- Generic type support with `<T>` syntax

#### Language Server Protocol (LSP) Integration
- Real-time diagnostics and error reporting
- Auto-completion for stdlib types and functions
- Hover information (framework)
- Go-to-definition support (framework)
- Document synchronization

#### Editor Features
- Auto-closing brackets, quotes, and parentheses
- Comment toggling (Ctrl+/)
- Code folding for functions and classes
- Bracket matching
- Smart indentation

#### Status & Commands
- Status bar indicator showing LSP server status
- `Fusion: Restart Language Server` command
- `Fusion: Show Output` command
- Configurable server path and arguments

### Configuration

Available settings:
- `fusion.server.path`: Path to fusion_lang executable
- `fusion.server.args`: Additional arguments for the LSP server
- `fusion.trace.server`: Trace level for LSP communication

### Requirements

- Fusion compiler with LSP support (`fusion_lang --lsp`)
- The Fusion compiler should be in your PATH or configured via settings

### Installation

1. Install from VS Code Marketplace (search for "Fusion")
2. Or install from `.vsix` file: `code --install-extension fusion-0.1.0.vsix`

### Known Issues

- Hover information currently shows placeholder text
- Symbol navigation needs enhancement
- Some advanced LSP features pending implementation

### Roadmap

- Enhanced auto-completion with context awareness
- Full symbol navigation across modules
- Code refactoring support
- Semantic syntax highlighting
- Code formatting

### Feedback

Report issues at: https://github.com/your-org/fusion-lang/issues

---

**Author**: Fusion Language Team  
**License**: MIT  
**Repository**: https://github.com/your-org/fusion-lang
