import { type GroupMode, PACKAGE_NAME, ScanMode, type ScanResult } from 'tscanner-common';
import type { TscannerLspClient } from '../../lsp/client';
import { DEFAULT_TARGET_BRANCH } from '../scripts-constants';
import { type FolderNode, type IssueResult, NodeKind } from '../types';
import { ToastKind, copyToClipboard, getCurrentWorkspaceFolder, showToastMessage } from './vscode-utils';

class CopyScanContext {
  private scanMode: ScanMode = ScanMode.Codebase;
  private compareBranch = DEFAULT_TARGET_BRANCH;
  private getLspClientFn: (() => TscannerLspClient | null) | null = null;

  setLspClient(fn: () => TscannerLspClient | null) {
    this.getLspClientFn = fn;
  }

  setScanContext(scanMode: ScanMode, compareBranch: string) {
    this.scanMode = scanMode;
    this.compareBranch = compareBranch;
  }

  getLspClient(): TscannerLspClient | null {
    return this.getLspClientFn?.() ?? null;
  }

  getScanModeText(): string {
    return this.scanMode === ScanMode.Branch ? 'branch mode' : 'codebase mode';
  }

  buildCliCommand(groupBy: GroupMode, filter?: string, filterValue?: string): string {
    const branch = this.scanMode === ScanMode.Branch ? this.compareBranch : undefined;
    const filterArg = filter && filterValue ? ` --${filter} "${filterValue}"` : '';
    const groupByArg = ` --group-by ${groupBy}`;
    const branchArg = branch ? ` --branch ${branch}` : '';
    return `${PACKAGE_NAME} check${filterArg}${groupByArg}${branchArg}`;
  }
}

export const copyScanContext = new CopyScanContext();

export function setCopyLspClient(getLspClient: () => TscannerLspClient | null) {
  copyScanContext.setLspClient(getLspClient);
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
      severity: issue.severity,
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

export function collectFolderIssues(node: FolderNode): IssueResult[] {
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

export type CopyParams = {
  results: IssueResult[];
  groupMode: GroupMode;
  buildHeader: (summary: { total_issues: number }) => string;
  successMessage: string;
};

export async function copyIssuesBase(params: CopyParams): Promise<void> {
  if (params.results.length === 0) {
    showToastMessage(ToastKind.Error, 'No issues to copy');
    return;
  }

  const lspClient = copyScanContext.getLspClient();
  if (!lspClient) {
    showToastMessage(ToastKind.Error, 'Scanner not initialized');
    return;
  }

  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    showToastMessage(ToastKind.Error, 'No workspace folder found');
    return;
  }
  const workspaceRoot = workspaceFolder.uri.fsPath;

  const scanResult = convertToScanResult(params.results);
  const result = await lspClient.formatResults(workspaceRoot, scanResult, params.groupMode);

  const header = params.buildHeader(result.summary);
  const summaryText = `\n\nIssues: ${result.summary.total_issues} (${result.summary.error_count} errors, ${result.summary.warning_count} warnings)\nFiles: ${result.summary.file_count}\nRules: ${result.summary.rule_count}`;
  const finalText = header + result.output + summaryText;

  await copyToClipboard(finalText);
  showToastMessage(ToastKind.Info, params.successMessage);
}
