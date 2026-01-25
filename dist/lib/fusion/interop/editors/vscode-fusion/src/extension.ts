// src/extension.ts - Fusion VS Code Extension Entry Point

import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
  console.log('Fusion Language Extension is now active');

  // Get configuration
  const config = vscode.workspace.getConfiguration('fusion');
  const serverPath = config.get<string>('server.path', 'fusion_lang');
  const serverArgs = config.get<string[]>('server.args', []);
  const traceLevel = config.get<string>('trace.server', 'off');

  // Server options - Launch the LSP server
  const serverOptions: ServerOptions = {
    command: serverPath,
    args: [...serverArgs, '--lsp'],
    transport: TransportKind.stdio,
  };

  // Client options - Define which documents the server handles
  const clientOptions: LanguageClientOptions = {
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
  client = new LanguageClient(
    'fusionLanguageServer',
    'Fusion Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client (also starts the server)
  client.start();

  // Register commands
  const restartCommand = vscode.commands.registerCommand(
    'fusion.restartServer',
    async () => {
      if (client) {
        await client.stop();
        client.start();
        vscode.window.showInformationMessage('Fusion Language Server restarted');
      }
    }
  );

  const showOutputCommand = vscode.commands.registerCommand(
    'fusion.showOutput',
    () => {
      client?.outputChannel.show();
    }
  );

  context.subscriptions.push(restartCommand, showOutputCommand);

  // Status bar item
  const statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );
  statusBarItem.text = '$(rocket) Fusion';
  statusBarItem.tooltip = 'Fusion Language Server Running';
  statusBarItem.command = 'fusion.showOutput';
  statusBarItem.show();

  context.subscriptions.push(statusBarItem);

  // Log activation
  vscode.window.showInformationMessage(
    'Fusion Language Support activated! LSP server starting...'
  );
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
