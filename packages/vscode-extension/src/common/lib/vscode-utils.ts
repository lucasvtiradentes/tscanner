import * as vscode from 'vscode';
import type { RefreshAiIssuesParams } from '../../commands/public/refresh-ai-issues';
import type { RefreshIssuesParams } from '../../commands/public/refresh-issues';
import { getCommandId } from '../constants';

export enum Command {
  RefreshIssues = 'refreshIssues',
  RefreshIssuesCached = 'refreshIssuesCached',
  RefreshAiIssues = 'refreshAiIssues',
  RefreshAiIssuesCached = 'refreshAiIssuesCached',
  OpenSettingsMenu = 'openSettingsMenu',
  CycleViewModeFileFlatView = 'cycleViewModeFileFlatView',
  CycleViewModeFileTreeView = 'cycleViewModeFileTreeView',
  CycleViewModeRuleFlatView = 'cycleViewModeRuleFlatView',
  CycleViewModeRuleTreeView = 'cycleViewModeRuleTreeView',
  OpenFile = 'openFile',
  GoToNextIssue = 'goToNextIssue',
  GoToPreviousIssue = 'goToPreviousIssue',
  ShowLogs = 'showLogs',
  ClearScanCaches = 'clearScanCaches',
  CopyRuleIssues = 'copyRuleIssues',
  CopyFileIssues = 'copyFileIssues',
  CopyFolderIssues = 'copyFolderIssues',
  CopyAllIssues = 'copyAllIssues',
  CopyAiRuleIssues = 'copyAiRuleIssues',
  CopyAiFileIssues = 'copyAiFileIssues',
  CopyAiFolderIssues = 'copyAiFolderIssues',
  CopyAllAiIssues = 'copyAllAiIssues',
}

export enum TreeItemContextValue {
  RuleGroup = 'TscannerNodeRuleGroup',
  Folder = 'TscannerNodeFolder',
  File = 'TscannerNodeFile',
  Issue = 'TscannerNodeIssue',
}

export interface CommandParams {
  [Command.RefreshIssues]: RefreshIssuesParams;
  [Command.RefreshAiIssues]: RefreshAiIssuesParams;
  [Command.OpenFile]: { filePath: string; line: number; column: number };
}

export function executeCommand<T extends Command>(
  command: T,
  ...args: T extends keyof CommandParams ? [CommandParams[T]?] : []
): Thenable<unknown> {
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

export function getCurrentWorkspaceFolder(): vscode.WorkspaceFolder | undefined {
  return vscode.workspace.workspaceFolders?.[0];
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

export function formatIssueCount(count: number, prefix?: string): string {
  const label = `${count} ${count === 1 ? 'issue' : 'issues'}`;
  return prefix ? `${prefix}: ${label}` : label;
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
