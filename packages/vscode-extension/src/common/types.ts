import type * as vscode from 'vscode';

export interface IssueResult {
  uri: vscode.Uri;
  line: number;
  column: number;
  text: string;
  rule: string;
  severity: 'error' | 'warning';
}

export interface RuleMetadata {
  name: string;
  displayName: string;
  description: string;
  ruleType: 'ast' | 'regex';
  defaultSeverity: 'error' | 'warning';
  defaultEnabled: boolean;
  category: 'typesafety' | 'codequality' | 'style' | 'performance';
}

export interface Issue {
  rule: string;
  file: string;
  line: number;
  column: number;
  message: string;
  severity: 'Error' | 'Warning';
  line_text?: string;
}

export interface FileResult {
  file: string;
  issues: Issue[];
}

export interface ScanResult {
  files: FileResult[];
  total_issues: number;
  duration_ms: number;
}

export interface ModifiedLineRange {
  startLine: number;
  lineCount: number;
}

export enum NodeKind {
  Folder = 'folder',
  File = 'file',
}

export interface FolderNode {
  type: NodeKind.Folder;
  path: string;
  name: string;
  children: Map<string, FolderNode | FileNode>;
}

export interface FileNode {
  type: NodeKind.File;
  path: string;
  name: string;
  results: IssueResult[];
}

export type BuiltinRuleConfig = {
  enabled?: boolean;
  severity?: 'error' | 'warning';
  include?: string[];
  exclude?: string[];
};

export type CustomRuleConfig = {
  type: 'regex' | 'script' | 'ai';
  pattern?: string;
  script?: string;
  prompt?: string;
  message: string;
  severity?: 'error' | 'warning';
  enabled?: boolean;
  include?: string[];
  exclude?: string[];
};

export interface TscannerConfig {
  builtinRules?: Record<string, BuiltinRuleConfig>;
  customRules?: Record<string, CustomRuleConfig>;
  include?: string[];
  exclude?: string[];
}

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;

  return !!(
    (config.builtinRules && Object.keys(config.builtinRules).length > 0) ||
    (config.customRules && Object.keys(config.customRules).length > 0)
  );
}
