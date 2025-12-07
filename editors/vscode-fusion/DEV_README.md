# Fusion VS Code Extension

This directory contains the Visual Studio Code extension for Fusion language support.

## Development

```bash
# Install dependencies
npm install

# Compile TypeScript
npm run compile

# Watch for changes
npm run watch

# Package extension
npm run package
```

## Testing

1. Open this folder in VS Code
2. Press `F5` to launch Extension Development Host
3. Open a `.fu` file to test syntax highlighting and LSP features

## Directory Structure

```
vscode-fusion/
├── src/
│   └── extension.ts       # Extension entry point
├── syntaxes/
│   └── fusion.tmLanguage.json  # Syntax highlighting grammar
├── package.json           # Extension manifest
├── tsconfig.json          # TypeScript configuration
├── language-configuration.json  # Language features config
└── README.md              # User-facing documentation
```
