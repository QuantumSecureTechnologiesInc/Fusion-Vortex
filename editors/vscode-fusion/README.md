# Fusion Language Support for VS Code

Official Visual Studio Code extension for the Fusion programming language, providing syntax highlighting, intelligent code completion, and real-time diagnostics via the Language Server Protocol (LSP).

## Features

### Syntax Highlighting

Full syntax highlighting for Fusion code including:
- Keywords (`fn`, `class`, `let`, `mut`, `if`, `while`, `for`, etc.)
- Types (primitive and generic)
- Functions and methods
- Strings, numbers, and comments
- Operators and punctuation

### IntelliSense & Code Completion

Context-aware code completion powered by the Fusion LSP server:
- Standard library types (`Vector`, `Option`, `Result`, `LinkedList`)
- Built-in functions (`println`, `malloc`, `free`)
- Local variables and function parameters
- Method suggestions based on type

### Real-Time Diagnostics

Instant feedback on your code:
- **Parse errors** - Syntax mistakes highlighted as you type
- **Semantic errors** - Type mismatches, borrow checker violations
- **Warnings** - Unused variables, unreachable code

### Go-to-Definition

Navigate to symbol definitions with `F12`:
- Jump to function declarations
- Navigate to class definitions
- Find variable declarations

### Hover Information

See type information by hovering over symbols:
- Function signatures
- Variable types
- Documentation (when available)

## Requirements

- **Fusion Compiler**: Install the Fusion compiler (`fusion_lang`) and ensure it's in your PATH
- **VS Code**: Version 1.80.0 or higher

## Installation

### From VSIX (Recommended)

1. Download the latest `.vsix` file from releases
2. Open VS Code
3. Go to Extensions (`Ctrl+Shift+X`)
4. Click the `...` menu → "Install from VSIX..."
5. Select the downloaded file

### From Source

```bash
cd editors/vscode-fusion
npm install
npm run compile
npm run package
```

Then install the generated `.vsix` file.

## Extension Settings

This extension contributes the following settings:

- `fusion.server.path`: Path to the Fusion language server executable (default: `"fusion_lang"`)
- `fusion.server.args`: Additional arguments to pass to the server (default: `[]`)
- `fusion.trace.server`: Trace LSP communication for debugging (`"off"` | `"messages"` | `"verbose"`)

### Example Configuration

```json
{
  "fusion.server.path": "/usr/local/bin/fusion_lang",
  "fusion.trace.server": "verbose"
}
```

## Commands

- **Fusion: Restart Language Server** (`fusion.restartServer`) - Restart the LSP server
- **Fusion: Show Output** (`fusion.showOutput`) - Show LSP server output logs

## Known Issues

- Symbol navigation (go-to-definition) is basic - full implementation in progress
- Hover type information shows placeholders - being enhanced
- Code formatting not yet implemented

## Development Roadmap

- [x] Syntax highlighting
- [x] LSP client integration
- [x] Basic auto-completion
- [x] Real-time diagnostics
- [ ] Enhanced symbol navigation
- [ ] Full hover type information
- [ ] Code formatting
- [ ] Snippet completions
- [ ] Debug adapter protocol (DAP)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## Release Notes

### 0.1.0 (2025-12-07)

**Initial Release**

- ✅ Fusion syntax highlighting
- ✅ LSP client integration
- ✅ Document synchronization
- ✅ Real-time diagnostics
- ✅ Basic auto-completion
- ✅ Hover support (basic)
- ✅ Go-to-definition (basic)

## License

MIT - See [LICENSE](../../LICENSE) for details

---

**Developed by**: QuantumSecure Technologies Ltd  
**Language**: Fusion Programming Language  
**Website**: [fusion-lang.org](https://fusion-lang.org)
