import * as vscode from 'vscode';
import { getConfigPath, hasConfig } from '../../common/lib/config-manager';
import { ensureLocalConfigFile } from '../../common/lib/local-config';
import {
  Command,
  ToastKind,
  registerCommand,
  requireWorkspaceOrNull,
  showToastMessage,
} from '../../common/lib/vscode-utils';

async function openFile(path: string): Promise<void> {
  const document = await vscode.workspace.openTextDocument(vscode.Uri.file(path));
  await vscode.window.showTextDocument(document);
}

export function createOpenProjectConfigCommand(): vscode.Disposable {
  return registerCommand(Command.OpenProjectConfig, async () => {
    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    if (!(await hasConfig(workspacePath))) {
      showToastMessage(ToastKind.Info, 'Run "tscanner init" to create config in the project root');
      return;
    }

    await openFile(getConfigPath(workspacePath));
  });
}

export function createOpenLocalConfigCommand(): vscode.Disposable {
  return registerCommand(Command.OpenLocalConfig, async () => {
    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    if (!(await hasConfig(workspacePath))) {
      showToastMessage(ToastKind.Info, 'Run "tscanner init" to create config in the project root');
      return;
    }

    const localConfigPath = await ensureLocalConfigFile(workspacePath);
    await openFile(localConfigPath);
  });
}
