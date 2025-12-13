import {
  AiExecutionMode,
  DEFAULT_TARGET_BRANCH,
  type GroupMode,
  IssueRuleType,
  ScanMode,
  type ScanResult,
  buildCheckCommand,
} from 'tscanner-common';
import type { TscannerLspClient } from '../../lsp/client';
import type { FormatPrettyResult } from '../../lsp/requests/types';
import { type FolderNode, type IssueResult, NodeKind } from '../types';
import { ToastKind, copyToClipboard, getCurrentWorkspaceFolder, showToastMessage } from './vscode-utils';

declare const __AI_FIX_PROMPT__: string;

const CONTENT_PLACEHOLDER = '{{CONTENT}}';
const [AI_FIX_PROMPT_HEADER, AI_FIX_PROMPT_FOOTER] = __AI_FIX_PROMPT__.split(CONTENT_PLACEHOLDER);

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

  buildCliCommand(groupBy: GroupMode, filter?: string, filterValue?: string, onlyAi?: boolean): string {
    const branch = this.scanMode === ScanMode.Branch ? this.compareBranch : undefined;
    return buildCheckCommand({
      groupBy,
      branch,
      filter: filter && filterValue ? { type: filter, value: filterValue } : undefined,
      aiMode: onlyAi ? AiExecutionMode.Only : undefined,
    });
  }
}

const copyScanContext = new CopyScanContext();

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
      line: issue.line + 1,
      column: issue.column + 1,
      end_column: issue.endColumn + 1,
      severity: issue.severity,
      line_text: issue.text,
      rule_type: issue.ruleType ?? IssueRuleType.Builtin,
    })),
  }));

  return {
    files,
    total_issues: results.length,
    duration_ms: 0,
    regular_rules_duration_ms: 0,
    ai_rules_duration_ms: 0,
    total_files: files.length,
    cached_files: 0,
    scanned_files: files.length,
    warnings: [],
    errors: [],
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

type CopyParams = {
  results: IssueResult[];
  groupMode: GroupMode;
  filterType: string;
  filterValue?: string;
  cliFilter?: string;
  cliFilterValue?: string;
  onlyAi?: boolean;
  successMessage: string;
};

function buildContext(params: CopyParams, totalIssues: number): string {
  const cliCommand = copyScanContext.buildCliCommand(
    params.groupMode,
    params.cliFilter,
    params.cliFilterValue,
    params.onlyAi,
  );
  const filterDisplay = params.filterValue ? `${params.filterType} "${params.filterValue}"` : params.filterType;
  return `Filter: ${filterDisplay} | Mode: ${copyScanContext.getScanModeText()} | Issues: ${totalIssues}\nCLI: ${cliCommand}\n`;
}

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

  let result: FormatPrettyResult;
  try {
    result = await lspClient.formatResults(workspaceRoot, scanResult, params.groupMode);
  } catch (error) {
    const errorMsg = String(error);
    if (errorMsg.includes('connection got disposed')) {
      showToastMessage(ToastKind.Error, 'LSP connection lost. Please run a scan first to reconnect.');
    } else {
      showToastMessage(ToastKind.Error, `Failed to format results: ${errorMsg}`);
    }
    return;
  }

  const context = buildContext(params, result.summary.total_issues);
  const finalText = AI_FIX_PROMPT_HEADER + context + result.output.trimEnd() + AI_FIX_PROMPT_FOOTER;

  await copyToClipboard(finalText);
  showToastMessage(ToastKind.Info, params.successMessage);
}
