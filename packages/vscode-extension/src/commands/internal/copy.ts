import { type ScanResult, Severity } from 'tscanner-common';
import * as vscode from 'vscode';
import type { RustClient } from '../../common/lib/rust-client';
import { Command, ScanMode, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import { DEFAULT_TARGET_BRANCH } from '../../common/scripts-constants';
import { type FolderNode, type IssueResult, NodeKind } from '../../common/types';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../../issues-panel/utils/tree-items';

class CopyScanContext {
  private scanMode: ScanMode = ScanMode.Codebase;
  private compareBranch = DEFAULT_TARGET_BRANCH;
  private getRustClientFn: (() => RustClient | null) | null = null;

  setRustClient(fn: () => RustClient | null) {
    this.getRustClientFn = fn;
  }

  setScanContext(scanMode: ScanMode, compareBranch: string) {
    this.scanMode = scanMode;
    this.compareBranch = compareBranch;
  }

  getRustClient(): RustClient | null {
    return this.getRustClientFn?.() ?? null;
  }

  getScanModeText(): string {
    return this.scanMode === ScanMode.Branch ? 'branch mode' : 'codebase mode';
  }

  buildCliCommand(filter: string, filterValue: string): string {
    const branch = this.scanMode === ScanMode.Branch ? this.compareBranch : undefined;
    return branch
      ? `tscanner check --${filter} "${filterValue}" --branch ${branch}`
      : `tscanner check --${filter} "${filterValue}"`;
  }
}

const copyScanContext = new CopyScanContext();

export function setCopyRustClient(getRustClient: () => RustClient | null) {
  copyScanContext.setRustClient(getRustClient);
}

export function setCopyScanContext(scanMode: ScanMode, compareBranch: string) {
  copyScanContext.setScanContext(scanMode, compareBranch);
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
      end_column: issue.endColumn,
      severity: (issue.severity === Severity.Error ? 'error' : 'warning') as 'error' | 'warning',
      line_text: issue.text,
    })),
  }));

  return {
    files,
    total_issues: results.length,
    duration_ms: 0,
    total_files: files.length,
    cached_files: 0,
    scanned_files: files.length,
  };
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

type CopyParams = {
  results: IssueResult[];
  groupMode: 'file' | 'rule';
  buildHeader: (summary: { total_issues: number }) => string;
  successMessage: string;
};

async function copyIssuesBase(params: CopyParams): Promise<void> {
  if (params.results.length === 0) {
    showToastMessage(ToastKind.Error, 'No issues to copy');
    return;
  }

  const rustClient = copyScanContext.getRustClient();
  if (!rustClient) {
    showToastMessage(ToastKind.Error, 'Scanner not initialized');
    return;
  }

  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (!workspaceRoot) {
    showToastMessage(ToastKind.Error, 'No workspace folder found');
    return;
  }

  const scanResult = convertToScanResult(params.results);
  const result = await rustClient.formatResults(workspaceRoot, scanResult, params.groupMode);

  const header = params.buildHeader(result.summary);
  const summaryText = `\n\nIssues: ${result.summary.total_issues} (${result.summary.error_count} errors, ${result.summary.warning_count} warnings)\nFiles: ${result.summary.file_count}\nRules: ${result.summary.rule_count}`;
  const finalText = header + result.output + summaryText;

  await vscode.env.clipboard.writeText(finalText);
  showToastMessage(ToastKind.Info, params.successMessage);
}

export function createCopyRuleIssuesCommand() {
  return registerCommand(Command.CopyRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: 'rule',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('rule', item.rule);
        return `TScanner report searching for all the issues of the rule "${item.rule}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${item.results.length} issues from "${item.rule}"`,
    });
  });
}

export function createCopyFileIssuesCommand() {
  return registerCommand(Command.CopyFileIssues, async (item: FileResultItem) => {
    if (!item?.results) return;

    const relativePath = vscode.workspace.asRelativePath(item.filePath);

    await copyIssuesBase({
      results: item.results,
      groupMode: 'file',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('glob', relativePath);
        return `TScanner report searching for all the issues in file "${relativePath}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${item.results.length} issues from "${relativePath}"`,
    });
  });
}

export function createCopyFolderIssuesCommand() {
  return registerCommand(Command.CopyFolderIssues, async (item: FolderResultItem) => {
    if (!item?.node) {
      showToastMessage(ToastKind.Error, 'No folder data available');
      return;
    }

    const allResults = collectFolderIssues(item.node);
    const relativeFolderPath = vscode.workspace.asRelativePath(item.node.path);

    await copyIssuesBase({
      results: allResults,
      groupMode: 'file',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('glob', `${relativeFolderPath}/**/*`);
        return `TScanner report searching for all the issues in folder "${item.node.name}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${allResults.length} issues from folder "${item.node.name}"`,
    });
  });
}
