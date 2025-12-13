import type * as vscode from 'vscode';
import type { TscannerLspClient } from '../../lsp/client';

export type CommandContext = {
  context: vscode.ExtensionContext;
  treeView: vscode.TreeView<vscode.TreeItem>;
  updateStatusBar: () => Promise<void>;
  getLspClient: () => TscannerLspClient | null;
};
