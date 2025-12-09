import * as path from 'node:path';
import { CONFIG_DIR_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { getConfigDirLabel, hasConfig, moveConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import {
  Command,
  type QuickPickItemWithId,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { WorkspaceStateKey, updateState } from '../common/state/workspace-state';
import type { RegularIssuesView } from '../issues-panel';

type ConfigLocationContext = {
  updateStatusBar: () => Promise<void>;
  currentConfigDirRef: { current: string | null };
  context: vscode.ExtensionContext;
  regularView: RegularIssuesView;
};

export function getCurrentLocationLabel(configDir: string | null, hasConfigFile: boolean): string {
  if (!hasConfigFile) {
    return 'No config set';
  }
  return `Current: ${getConfigDirLabel(configDir)}`;
}

export async function showConfigLocationMenu(ctx: ConfigLocationContext): Promise<void> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return;
  }

  const workspacePath = workspaceFolder.uri.fsPath;
  const currentConfigDir = ctx.currentConfigDirRef.current;
  const currentHasConfig = await hasConfig(workspacePath, currentConfigDir);

  const startPath = currentConfigDir ?? '.';
  const selectedPath = await showFolderPicker(workspaceFolder.uri, startPath);

  if (!selectedPath) return;

  const newConfigDir = selectedPath === '.' ? null : selectedPath;

  if (newConfigDir === currentConfigDir) {
    return;
  }

  const newHasConfig = await hasConfig(workspacePath, newConfigDir);

  if (currentHasConfig && !newHasConfig) {
    const shouldMove = await askToMoveConfig(currentConfigDir, newConfigDir);
    if (shouldMove) {
      await moveConfig(workspacePath, currentConfigDir, newConfigDir);
    }
  }

  ctx.currentConfigDirRef.current = newConfigDir;
  updateState(ctx.context, WorkspaceStateKey.CustomConfigDir, newConfigDir);
  logger.info(`Config dir changed to: ${getConfigDirLabel(newConfigDir)}`);

  await ctx.updateStatusBar();
  ctx.regularView.setResults([]);
  executeCommand(Command.FindIssue, { silent: true });
}

async function askToMoveConfig(fromDir: string | null, toDir: string | null): Promise<boolean> {
  const fromLabel = getConfigDirLabel(fromDir);
  const toLabel = getConfigDirLabel(toDir);

  const result = await vscode.window.showWarningMessage(
    `Move config from "${fromLabel}" to "${toLabel}"?`,
    { modal: true },
    'Move',
    'Just point to new location',
  );

  return result === 'Move';
}

async function getSubfolders(dirUri: vscode.Uri): Promise<string[]> {
  try {
    const entries = await vscode.workspace.fs.readDirectory(dirUri);
    return entries.filter(([_, type]) => type === vscode.FileType.Directory).map(([name]) => name);
  } catch {
    return [];
  }
}

async function showFolderPicker(workspaceRoot: vscode.Uri, currentRelativePath: string): Promise<string | null> {
  const currentUri =
    currentRelativePath === '.' ? workspaceRoot : vscode.Uri.joinPath(workspaceRoot, currentRelativePath);

  const subfolders = await getSubfolders(currentUri);

  const items: QuickPickItemWithId<string>[] = [];

  items.push({
    id: '__select__',
    label: '$(check) Select this folder',
    detail:
      currentRelativePath === '.'
        ? `Use: ${CONFIG_DIR_NAME} (project root)`
        : `Use: ${path.posix.join(currentRelativePath, CONFIG_DIR_NAME)}`,
  });

  if (currentRelativePath !== '.') {
    items.push({
      id: '__parent__',
      label: '$(arrow-up) ..',
      detail: 'Go to parent folder',
    });
  }

  if (subfolders.length > 0) {
    items.push({
      id: '__separator__',
      label: '',
      kind: vscode.QuickPickItemKind.Separator,
    });
  }

  for (const folder of subfolders.sort()) {
    items.push({
      id: folder,
      label: `$(folder) ${folder}`,
      detail: currentRelativePath === '.' ? folder : path.posix.join(currentRelativePath, folder),
    });
  }

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: `Select folder for ${CONFIG_DIR_NAME}`,
    ignoreFocusOut: true,
  });

  if (!selected) return null;

  if (selected.id === '__select__') {
    return currentRelativePath;
  }

  if (selected.id === '__parent__') {
    const parent = path.posix.dirname(currentRelativePath);
    const parentPath = parent === '.' || parent === '' ? '.' : parent;
    return showFolderPicker(workspaceRoot, parentPath);
  }

  const nextPath = currentRelativePath === '.' ? selected.id : path.posix.join(currentRelativePath, selected.id);
  return showFolderPicker(workspaceRoot, nextPath);
}
