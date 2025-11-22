import type * as vscode from 'vscode';
import { z } from 'zod';

const issueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  message: z.string(),
  severity: z.enum(['Error', 'Warning']),
  line_text: z.string().optional(),
});

const fileResultSchema = z.object({
  file: z.string(),
  issues: z.array(issueSchema),
});

const scanResultSchema = z.object({
  files: z.array(fileResultSchema),
  total_issues: z.number(),
  duration_ms: z.number(),
});

const ruleMetadataSchema = z.object({
  name: z.string(),
  displayName: z.string(),
  description: z.string(),
  ruleType: z.enum(['ast', 'regex']),
  defaultSeverity: z.enum(['error', 'warning']),
  defaultEnabled: z.boolean(),
  category: z.enum(['typesafety', 'codequality', 'style', 'performance']),
});

export type Issue = z.infer<typeof issueSchema>;
export type FileResult = z.infer<typeof fileResultSchema>;
export type ScanResult = z.infer<typeof scanResultSchema>;
export type RuleMetadata = z.infer<typeof ruleMetadataSchema>;

export interface IssueResult {
  uri: vscode.Uri;
  line: number;
  column: number;
  text: string;
  rule: string;
  severity: 'error' | 'warning';
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
