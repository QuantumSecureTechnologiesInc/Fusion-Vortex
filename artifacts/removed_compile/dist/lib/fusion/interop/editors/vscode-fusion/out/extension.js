"use strict";
// src/extension.ts - Fusion VS Code Extension Entry Point
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    console.log('Fusion Language Extension is now active');
    // Get configuration
    const config = vscode.workspace.getConfiguration('fusion');
    const serverPath = config.get('server.path', 'fusion_lang');
    const serverArgs = config.get('server.args', []);
    const traceLevel = config.get('trace.server', 'off');
    // Server options - Launch the LSP server
    const serverOptions = {
        command: serverPath,
        args: [...serverArgs, '--lsp'],
        transport: node_1.TransportKind.stdio,
    };
    // Client options - Define which documents the server handles
    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'fusion' },
            { scheme: 'untitled', language: 'fusion' },
        ],
        synchronize: {
            // Notify the server about file changes to .fu files
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.fu'),
        },
        outputChannelName: 'Fusion Language Server',
        traceOutputChannel: vscode.window.createOutputChannel('Fusion LSP Trace'),
    };
    // Create the language client and start it
    client = new node_1.LanguageClient('fusionLanguageServer', 'Fusion Language Server', serverOptions, clientOptions);
    // Start the client (also starts the server)
    client.start();
    // Register commands
    const restartCommand = vscode.commands.registerCommand('fusion.restartServer', async () => {
        if (client) {
            await client.stop();
            client.start();
            vscode.window.showInformationMessage('Fusion Language Server restarted');
        }
    });
    const showOutputCommand = vscode.commands.registerCommand('fusion.showOutput', () => {
        client?.outputChannel.show();
    });
    context.subscriptions.push(restartCommand, showOutputCommand);
    // Status bar item
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(rocket) Fusion';
    statusBarItem.tooltip = 'Fusion Language Server Running';
    statusBarItem.command = 'fusion.showOutput';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
    // Log activation
    vscode.window.showInformationMessage('Fusion Language Support activated! LSP server starting...');
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map