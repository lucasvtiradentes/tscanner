import { DEFAULT_TARGET_BRANCH, GroupMode, ScanMode, ViewMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { z } from 'zod';
import { getCommandId, getContextKey } from '../constants';

export { GroupMode, ScanMode, ViewMode };

export enum WorkspaceStateKey {
  ViewMode = 'viewMode',
  GroupMode = 'groupMode',
  ScanMode = 'scanMode',
  CompareBranch = 'compareBranch',
  CachedResults = 'cachedResults',
  CustomConfigDir = 'customConfigDir',
}

const workspaceStateSchema = z.object({
  [WorkspaceStateKey.ViewMode]: z.enum(ViewMode),
  [WorkspaceStateKey.GroupMode]: z.enum(GroupMode),
  [WorkspaceStateKey.ScanMode]: z.enum(ScanMode),
  [WorkspaceStateKey.CompareBranch]: z.string(),
  [WorkspaceStateKey.CachedResults]: z.array(z.any()),
  [WorkspaceStateKey.CustomConfigDir]: z.string().nullable(),
});

type WorkspaceStateSchema = z.infer<typeof workspaceStateSchema>;
type WorkspaceStateKeyType = keyof WorkspaceStateSchema;

const defaultValues: WorkspaceStateSchema = {
  [WorkspaceStateKey.ViewMode]: ViewMode.List,
  [WorkspaceStateKey.GroupMode]: GroupMode.File,
  [WorkspaceStateKey.ScanMode]: ScanMode.Codebase,
  [WorkspaceStateKey.CompareBranch]: DEFAULT_TARGET_BRANCH,
  [WorkspaceStateKey.CachedResults]: [],
  [WorkspaceStateKey.CustomConfigDir]: null,
};

const CONTEXT_PREFIX = 'tscanner';

const keyMapping: Record<WorkspaceStateKeyType, string> = Object.fromEntries(
  Object.values(WorkspaceStateKey).map((key) => [key, `${CONTEXT_PREFIX}.${key}`]),
) as Record<WorkspaceStateKeyType, string>;

export enum ContextKey {
  ViewMode = 'tscannerViewMode',
  GroupMode = 'tscannerGroupMode',
  ScanMode = 'tscannerScanMode',
  Searching = 'tscannerSearching',
}

export enum Command {
  FindIssue = 'findIssue',
  ManageRules = 'manageRules',
  OpenSettingsMenu = 'openSettingsMenu',
  CycleViewMode = 'cycleViewMode',
  OpenFile = 'openFile',
  CopyPath = 'copyPath',
  CopyRelativePath = 'copyRelativePath',
  Refresh = 'refresh',
  HardScan = 'hardScan',
  GoToNextIssue = 'goToNextIssue',
  GoToPreviousIssue = 'goToPreviousIssue',
  ShowLogs = 'showLogs',
  CopyRuleIssues = 'copyRuleIssues',
  CopyFileIssues = 'copyFileIssues',
  CopyFolderIssues = 'copyFolderIssues',
  CopyAllIssues = 'copyAllIssues',
}

export enum TreeItemContextValue {
  RuleGroup = 'TscannerNodeRuleGroup',
  Folder = 'TscannerNodeFolder',
  File = 'TscannerNodeFile',
  Issue = 'TscannerNodeIssue',
}

const contextKeyMapping: Partial<Record<WorkspaceStateKeyType, ContextKey>> = {
  viewMode: ContextKey.ViewMode,
  groupMode: ContextKey.GroupMode,
  scanMode: ContextKey.ScanMode,
};

export function getWorkspaceState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
): WorkspaceStateSchema[K] {
  const storageKey = keyMapping[key];
  const value = context.workspaceState.get(storageKey);
  const defaultValue = defaultValues[key];

  if (value === undefined) {
    return defaultValue;
  }

  const schema = workspaceStateSchema.shape[key];
  const result = schema.safeParse(value);

  return (result.success ? result.data : defaultValue) as WorkspaceStateSchema[K];
}

export function setWorkspaceState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
  value: WorkspaceStateSchema[K],
): Thenable<void> {
  const storageKey = keyMapping[key];
  return context.workspaceState.update(storageKey, value);
}

export function setContextKey(key: ContextKey, value: unknown): Thenable<unknown> {
  return vscode.commands.executeCommand('setContext', getContextKey(key), value);
}

export function updateState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
  value: WorkspaceStateSchema[K],
): void {
  setWorkspaceState(context, key, value);
  const contextKey = contextKeyMapping[key];
  if (contextKey) {
    setContextKey(contextKey, value);
  }
}

export function executeCommand(command: Command, ...args: any[]): Thenable<unknown> {
  return vscode.commands.executeCommand(getCommandId(command), ...args);
}

export function registerCommand(command: Command, callback: (...args: any[]) => any): vscode.Disposable {
  return vscode.commands.registerCommand(getCommandId(command), callback);
}

export enum ToastKind {
  Info = 'info',
  Warning = 'warning',
  Error = 'error',
}

export function showToastMessage(kind: ToastKind, message: string, ...items: string[]): Thenable<string | undefined> {
  switch (kind) {
    case ToastKind.Info:
      return vscode.window.showInformationMessage(message, ...items);
    case ToastKind.Warning:
      return vscode.window.showWarningMessage(message, ...items);
    case ToastKind.Error:
      return vscode.window.showErrorMessage(message, ...items);
  }
}

export function openTextDocument(uri: vscode.Uri): Thenable<vscode.TextDocument> {
  return vscode.workspace.openTextDocument(uri);
}

export function getWorkspaceFolders(): readonly vscode.WorkspaceFolder[] | undefined {
  return vscode.workspace.workspaceFolders;
}

export function getCurrentWorkspaceFolder(): vscode.WorkspaceFolder | undefined {
  return vscode.workspace.workspaceFolders?.[0];
}

export function requireWorkspace(): vscode.WorkspaceFolder {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    throw new Error('No workspace folder open');
  }
  return workspaceFolder;
}

export function requireWorkspaceOrNull(): vscode.WorkspaceFolder | null {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder open');
    return null;
  }
  return workspaceFolder;
}

export type QuickPickItemWithId<T extends string = string> = {
  id: T;
} & vscode.QuickPickItem;

export function formatIssueCount(count: number): string {
  return `${count} ${count === 1 ? 'issue' : 'issues'}`;
}

export async function navigateToPosition(uri: vscode.Uri, line: number, column: number): Promise<void> {
  const doc = await openTextDocument(uri);
  const editor = await vscode.window.showTextDocument(doc);
  const position = new vscode.Position(line, column);
  editor.selection = new vscode.Selection(position, position);
  editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
}

export function copyToClipboard(text: string): Thenable<void> {
  return vscode.env.clipboard.writeText(text);
}
