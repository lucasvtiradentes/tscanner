import * as vscode from 'vscode';
import type { RustClient } from '../common/lib/rust-client';
import { Command, ScanMode, ToastKind, registerCommand, showToastMessage } from '../common/lib/vscode-utils';
import { DEFAULT_TARGET_BRANCH } from '../common/scripts-constants';
import { type FolderNode, type IssueResult, NodeKind, type ScanResult } from '../common/types';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../sidebar/tree-items';

let currentScanMode: ScanMode = ScanMode.Codebase;
let currentCompareBranch = DEFAULT_TARGET_BRANCH;
let getRustClientFn: (() => RustClient | null) | null = null;

export function setCopyRustClient(getRustClient: () => RustClient | null) {
  getRustClientFn = getRustClient;
}

export function setCopyScanContext(scanMode: ScanMode, compareBranch: string) {
  currentScanMode = scanMode;
  currentCompareBranch = compareBranch;
}

function convertToScanResult(results: IssueResult[]): ScanResult {
  const fileMap = new Map<string, IssueResult[]>();

  for (const result of results) {
    const filePath = result.uri.fsPath;
    if (!fileMap.has(filePath)) {
      fileMap.set(filePath, []);
    }
    fileMap.get(filePath)!.push(result);
  }

  const files = Array.from(fileMap.entries()).map(([filePath, issues]) => ({
    file: filePath,
    issues: issues.map((issue) => ({
      rule: issue.rule,
      file: filePath,
      message: issue.message,
      line: issue.line,
      column: issue.column,
      severity: issue.severity as 'error' | 'warning',
      line_text: issue.text,
    })),
  }));

  return {
    files,
    total_issues: results.length,
    duration_ms: 0,
  };
}

export function createCopyRuleIssuesCommand() {
  return registerCommand(Command.CopyRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results || item.results.length === 0) {
      showToastMessage(ToastKind.Error, 'No issues to copy');
      return;
    }

    const rustClient = getRustClientFn?.();
    if (!rustClient) {
      showToastMessage(ToastKind.Error, 'Scanner not initialized');
      return;
    }

    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceRoot) {
      showToastMessage(ToastKind.Error, 'No workspace folder found');
      return;
    }

    const scanResult = convertToScanResult(item.results);
    const result = await rustClient.formatResults(workspaceRoot, scanResult, 'rule');

    const scanModeText = currentScanMode === ScanMode.Branch ? 'branch mode' : 'workspace mode';
    const branch = currentScanMode === ScanMode.Branch ? currentCompareBranch : undefined;
    const cliCommand = branch
      ? `tscanner check --rule "${item.rule}" --branch ${branch}`
      : `tscanner check --rule "${item.rule}"`;

    const header = `tscanner report searching for all the issues of the rule "${item.rule}" in the ${scanModeText}\n\ncli command: ${cliCommand}\nfound issues: ${result.summary.total_issues} issues\n`;

    const summaryText = `\n\nIssues: ${result.summary.total_issues} (${result.summary.error_count} errors, ${result.summary.warning_count} warnings)\nFiles: ${result.summary.file_count}\nRules: ${result.summary.rule_count}`;

    const finalText = header + result.output + summaryText;

    await vscode.env.clipboard.writeText(finalText);
    showToastMessage(ToastKind.Info, `Copied ${item.results.length} issues from "${item.rule}"`);
  });
}

export function createCopyFileIssuesCommand() {
  return registerCommand(Command.CopyFileIssues, async (item: FileResultItem) => {
    if (!item?.results || item.results.length === 0) {
      showToastMessage(ToastKind.Error, 'No issues to copy');
      return;
    }

    const rustClient = getRustClientFn?.();
    if (!rustClient) {
      showToastMessage(ToastKind.Error, 'Scanner not initialized');
      return;
    }

    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceRoot) {
      showToastMessage(ToastKind.Error, 'No workspace folder found');
      return;
    }

    const scanResult = convertToScanResult(item.results);
    const result = await rustClient.formatResults(workspaceRoot, scanResult, 'file');

    const relativePath = vscode.workspace.asRelativePath(item.filePath);
    const scanModeText = currentScanMode === ScanMode.Branch ? 'branch mode' : 'workspace mode';
    const branch = currentScanMode === ScanMode.Branch ? currentCompareBranch : undefined;
    const cliCommand = branch
      ? `tscanner check --file "${relativePath}" --branch ${branch}`
      : `tscanner check --file "${relativePath}"`;

    const header = `tscanner report searching for all the issues in file "${relativePath}" in the ${scanModeText}\n\ncli command: ${cliCommand}\nfound issues: ${result.summary.total_issues} issues\n`;

    const summaryText = `\n\nIssues: ${result.summary.total_issues} (${result.summary.error_count} errors, ${result.summary.warning_count} warnings)\nFiles: ${result.summary.file_count}\nRules: ${result.summary.rule_count}`;

    const finalText = header + result.output + summaryText;

    await vscode.env.clipboard.writeText(finalText);
    showToastMessage(ToastKind.Info, `Copied ${item.results.length} issues from "${relativePath}"`);
  });
}

export function createCopyFolderIssuesCommand() {
  return registerCommand(Command.CopyFolderIssues, async (item: FolderResultItem) => {
    if (!item?.node) {
      showToastMessage(ToastKind.Error, 'No folder data available');
      return;
    }

    const allResults = collectFolderIssues(item.node);
    if (allResults.length === 0) {
      showToastMessage(ToastKind.Error, 'No issues to copy');
      return;
    }

    const rustClient = getRustClientFn?.();
    if (!rustClient) {
      showToastMessage(ToastKind.Error, 'Scanner not initialized');
      return;
    }

    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceRoot) {
      showToastMessage(ToastKind.Error, 'No workspace folder found');
      return;
    }

    const scanResult = convertToScanResult(allResults);
    const result = await rustClient.formatResults(workspaceRoot, scanResult, 'file');

    const relativeFolderPath = vscode.workspace.asRelativePath(item.node.path);
    const scanModeText = currentScanMode === ScanMode.Branch ? 'branch mode' : 'workspace mode';
    const branch = currentScanMode === ScanMode.Branch ? currentCompareBranch : undefined;
    const cliCommand = branch
      ? `tscanner check --file "${relativeFolderPath}/**/*" --branch ${branch}`
      : `tscanner check --file "${relativeFolderPath}/**/*"`;

    const header = `tscanner report searching for all the issues in folder "${item.node.name}" in the ${scanModeText}\n\ncli command: ${cliCommand}\nfound issues: ${result.summary.total_issues} issues\n`;

    const summaryText = `\n\nIssues: ${result.summary.total_issues} (${result.summary.error_count} errors, ${result.summary.warning_count} warnings)\nFiles: ${result.summary.file_count}\nRules: ${result.summary.rule_count}`;

    const finalText = header + result.output + summaryText;

    await vscode.env.clipboard.writeText(finalText);
    showToastMessage(ToastKind.Info, `Copied ${allResults.length} issues from folder "${item.node.name}"`);
  });
}

function collectFolderIssues(node: FolderNode): IssueResult[] {
  const results: IssueResult[] = [];

  for (const child of node.children.values()) {
    if (child.type === NodeKind.Folder) {
      results.push(...collectFolderIssues(child));
    } else {
      results.push(...child.results);
    }
  }

  return results;
}
