import * as vscode from 'vscode';
import { Command, ScanMode, ToastKind, registerCommand, showToastMessage } from '../common/lib/vscode-utils';
import { type FolderNode, type IssueResult, NodeKind } from '../common/types';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../sidebar/tree-items';

let currentScanMode: ScanMode = ScanMode.Codebase;
let currentCompareBranch = 'main';

export function setCopyScanContext(scanMode: ScanMode, compareBranch: string) {
  currentScanMode = scanMode;
  currentCompareBranch = compareBranch;
}

export function createCopyRuleIssuesCommand() {
  return registerCommand(Command.CopyRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results || item.results.length === 0) {
      showToastMessage(ToastKind.Error, 'No issues to copy');
      return;
    }

    const formatted = formatRuleIssues(item.rule, item.results, currentScanMode, currentCompareBranch);
    await vscode.env.clipboard.writeText(formatted);
    showToastMessage(ToastKind.Info, `Copied ${item.results.length} issues from "${item.rule}"`);
  });
}

export function createCopyFileIssuesCommand() {
  return registerCommand(Command.CopyFileIssues, async (item: FileResultItem) => {
    if (!item?.results || item.results.length === 0) {
      showToastMessage(ToastKind.Error, 'No issues to copy');
      return;
    }

    const relativePath = vscode.workspace.asRelativePath(item.filePath);
    const formatted = formatFileIssues(relativePath, item.results, currentScanMode, currentCompareBranch);
    await vscode.env.clipboard.writeText(formatted);
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

    const formatted = formatFolderIssues(
      item.node.name,
      item.node.path,
      allResults,
      currentScanMode,
      currentCompareBranch,
    );
    await vscode.env.clipboard.writeText(formatted);
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

function formatRuleIssues(ruleName: string, results: IssueResult[], scanMode: ScanMode, compareBranch: string): string {
  const lines: string[] = [];
  const issueCount = results.length;
  const mode = scanMode === ScanMode.Branch ? 'branch' : 'codebase';

  lines.push(`tscanner report searching for all the issues of the rule "${ruleName}" in the ${mode} mode`);
  lines.push('');

  const cliCommand =
    scanMode === ScanMode.Branch
      ? `tscanner check --rule "${ruleName}" --branch ${compareBranch}`
      : `tscanner check --rule "${ruleName}"`;

  lines.push(`cli command: ${cliCommand}`);
  lines.push(`found issues: ${issueCount} ${issueCount === 1 ? 'issue' : 'issues'}`);
  lines.push('');

  const rulesMap = new Map<string, string>();
  for (const result of results) {
    if (!rulesMap.has(result.rule)) {
      rulesMap.set(result.rule, result.message);
    }
  }

  lines.push('found rules:');
  lines.push('');
  for (const [rule, message] of rulesMap) {
    lines.push(`  ${rule}: ${message}`);
  }

  lines.push('');
  lines.push('files:');
  lines.push('');

  const fileGroups = new Map<string, IssueResult[]>();
  for (const result of results) {
    const relativePath = vscode.workspace.asRelativePath(result.uri);
    if (!fileGroups.has(relativePath)) {
      fileGroups.set(relativePath, []);
    }
    fileGroups.get(relativePath)!.push(result);
  }

  for (const [filePath, fileResults] of fileGroups) {
    lines.push(`${filePath} - ${fileResults.length} ${fileResults.length === 1 ? 'issue' : 'issues'}`);
    lines.push('');

    for (const result of fileResults) {
      const icon = result.severity === 'error' ? '✖' : '⚠';
      const line = result.line + 1;
      const col = result.column + 1;
      lines.push(`  ${icon} ${line}:${col}`);
      lines.push(`    ${result.text.trim()}`);
    }

    lines.push('');
  }

  return lines.join('\n').trimEnd();
}

function formatFileIssues(fileName: string, results: IssueResult[], scanMode: ScanMode, compareBranch: string): string {
  const lines: string[] = [];
  const issueCount = results.length;
  const mode = scanMode === ScanMode.Branch ? 'branch' : 'codebase';

  lines.push(`tscanner report searching for all the issues in the file "${fileName}" in the ${mode} mode`);
  lines.push('');

  const cliCommand =
    scanMode === ScanMode.Branch
      ? `tscanner check --file "${fileName}" --branch ${compareBranch}`
      : `tscanner check --file "${fileName}"`;

  lines.push(`cli command: ${cliCommand}`);
  lines.push(`found issues: ${issueCount} ${issueCount === 1 ? 'issue' : 'issues'}`);
  lines.push('');

  const rulesMap = new Map<string, string>();
  for (const result of results) {
    if (!rulesMap.has(result.rule)) {
      rulesMap.set(result.rule, result.message);
    }
  }

  lines.push('found rules:');
  lines.push('');
  for (const [rule, message] of rulesMap) {
    lines.push(`  ${rule}: ${message}`);
  }

  lines.push('');
  lines.push('files:');
  lines.push('');

  const uniqueRules = new Set(results.map((r) => r.rule)).size;
  lines.push(
    `${fileName} - ${issueCount} ${issueCount === 1 ? 'issue' : 'issues'} - ${uniqueRules} ${uniqueRules === 1 ? 'rule' : 'rules'}`,
  );
  lines.push('');

  for (const result of results) {
    const icon = result.severity === 'error' ? '✖' : '⚠';
    const line = result.line + 1;
    const col = result.column + 1;
    lines.push(`  ${icon} ${line}:${col} - ${result.rule}`);
    lines.push(`    ${result.text.trim()}`);
  }

  return lines.join('\n');
}

function formatFolderIssues(
  folderName: string,
  folderPath: string,
  results: IssueResult[],
  scanMode: ScanMode,
  compareBranch: string,
): string {
  const lines: string[] = [];
  const issueCount = results.length;
  const mode = scanMode === ScanMode.Branch ? 'branch' : 'codebase';

  lines.push(`tscanner report searching for all the issues in the folder "${folderName}" in the ${mode} mode`);
  lines.push('');

  const relativeFolderPath = vscode.workspace.asRelativePath(folderPath);
  const folderPattern = `${relativeFolderPath}/**`;

  const cliCommand =
    scanMode === ScanMode.Branch
      ? `tscanner check --file "${folderPattern}" --branch ${compareBranch}`
      : `tscanner check --file "${folderPattern}"`;

  lines.push(`cli command: ${cliCommand}`);
  lines.push(`found issues: ${issueCount} ${issueCount === 1 ? 'issue' : 'issues'}`);
  lines.push('');

  const rulesMap = new Map<string, string>();
  for (const result of results) {
    if (!rulesMap.has(result.rule)) {
      rulesMap.set(result.rule, result.message);
    }
  }

  lines.push('found rules:');
  lines.push('');
  for (const [rule, message] of rulesMap) {
    lines.push(`  ${rule}: ${message}`);
  }

  lines.push('');
  lines.push('files:');
  lines.push('');

  const fileGroups = new Map<string, IssueResult[]>();
  for (const result of results) {
    const relativePath = vscode.workspace.asRelativePath(result.uri);
    if (!fileGroups.has(relativePath)) {
      fileGroups.set(relativePath, []);
    }
    fileGroups.get(relativePath)!.push(result);
  }

  for (const [filePath, fileResults] of fileGroups) {
    const uniqueRules = new Set(fileResults.map((r) => r.rule)).size;
    lines.push(
      `${filePath} - ${fileResults.length} ${fileResults.length === 1 ? 'issue' : 'issues'} - ${uniqueRules} ${uniqueRules === 1 ? 'rule' : 'rules'}`,
    );
    lines.push('');

    for (const result of fileResults) {
      const icon = result.severity === 'error' ? '✖' : '⚠';
      const line = result.line + 1;
      const col = result.column + 1;
      lines.push(`  ${icon} ${line}:${col} - ${result.rule}`);
      lines.push(`    ${result.text.trim()}`);
    }

    lines.push('');
  }

  return lines.join('\n').trimEnd();
}
