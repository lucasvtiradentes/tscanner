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
import { StoreKey, extensionStore } from '../common/state/extension-store';
import type { RegularIssuesView } from '../issues-panel';

const ROOT_PATH = '.';

function isRootPath(p: string): boolean {
  return p === ROOT_PATH || p === '';
}

function toConfigDir(selectedPath: string): string | null {
  return isRootPath(selectedPath) ? null : selectedPath;
}

function joinPath(base: string, segment: string): string {
  return isRootPath(base) ? segment : path.posix.join(base, segment);
}

type ConfigLocationContext = {
  updateStatusBar: () => Promise<void>;
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
  const currentConfigDir = extensionStore.get(StoreKey.ConfigDir);
  const currentHasConfig = await hasConfig(workspacePath, currentConfigDir);

  const startPath = currentConfigDir ?? ROOT_PATH;
  const selectedPath = await showFolderPicker(workspaceFolder.uri, startPath);

  if (!selectedPath) return;

  const newConfigDir = toConfigDir(selectedPath);

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

  extensionStore.set(StoreKey.ConfigDir, newConfigDir);
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

async function showFolderPicker(workspaceRoot: vscode.Uri, currentPath: string): Promise<string | null> {
  const currentUri = isRootPath(currentPath) ? workspaceRoot : vscode.Uri.joinPath(workspaceRoot, currentPath);

  const subfolders = await getSubfolders(currentUri);
  const isRoot = isRootPath(currentPath);

  const items: QuickPickItemWithId<string>[] = [];

  items.push({
    id: '__select__',
    label: '$(check) Select this folder',
    detail: isRoot ? `Use: ${CONFIG_DIR_NAME} (project root)` : `Use: ${joinPath(currentPath, CONFIG_DIR_NAME)}`,
  });

  if (!isRoot) {
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
      detail: joinPath(currentPath, folder),
    });
  }

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: `Select folder for ${CONFIG_DIR_NAME}`,
    ignoreFocusOut: true,
  });

  if (!selected) return null;

  if (selected.id === '__select__') {
    return currentPath;
  }

  if (selected.id === '__parent__') {
    const parent = path.posix.dirname(currentPath);
    return showFolderPicker(workspaceRoot, isRootPath(parent) ? ROOT_PATH : parent);
  }

  return showFolderPicker(workspaceRoot, joinPath(currentPath, selected.id));
}
