import { Severity } from 'tscanner-common';
import type * as vscode from 'vscode';

export type {
  BuiltinRuleConfig,
  ClearCacheParams,
  CustomRuleConfig,
  FileResult,
  GetRulesMetadataParams,
  Issue,
  ModifiedLineRange,
  RuleMetadata,
  ScanContentParams,
  ScanFileParams,
  ScanParams,
  ScanResult,
  TscannerConfig,
} from 'tscanner-common';

export {
  CustomRuleType,
  GroupMode,
  RuleCategory,
  ScanMode,
  Severity,
  ViewMode,
  hasConfiguredRules,
} from 'tscanner-common';

export type IssueResult = {
  uri: vscode.Uri;
  line: number;
  column: number;
  endColumn: number;
  text: string;
  rule: string;
  severity: Severity;
  message: string;
};

export enum NodeKind {
  Folder = 'folder',
  File = 'file',
}

export type FolderNode = {
  type: NodeKind.Folder;
  path: string;
  name: string;
  children: Map<string, FolderNode | FileNode>;
};

export type FileNode = {
  type: NodeKind.File;
  path: string;
  name: string;
  results: IssueResult[];
};

export function parseSeverity(severity: string): Severity {
  return severity.toLowerCase() === 'error' ? Severity.Error : Severity.Warning;
}

export type SerializedIssueResult = Omit<IssueResult, 'uri'> & { uriString: string };

export function serializeResults(results: IssueResult[]): SerializedIssueResult[] {
  return results.map((r) => {
    const { uri, ...rest } = r;
    return { ...rest, uriString: uri.toString() };
  });
}
