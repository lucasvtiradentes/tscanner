import type * as vscode from 'vscode';
import { z } from 'zod';

export enum RustSeverity {
  Error = 'Error',
  Warning = 'Warning',
}

export enum Severity {
  Error = 'error',
  Warning = 'warning',
}

export enum RuleType {
  Ast = 'ast',
  Regex = 'regex',
}

export enum RuleCategory {
  TypeSafety = 'typesafety',
  CodeQuality = 'codequality',
  Style = 'style',
  Performance = 'performance',
}

export enum CustomRuleType {
  Regex = 'regex',
  Script = 'script',
  Ai = 'ai',
}

const scanParamsSchema = z.object({
  root: z.string(),
  config: z.any().optional(),
  branch: z.string().optional(),
});

const scanFileParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
});

const scanContentParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
  content: z.string(),
  config: z.any().optional(),
});

const getRulesMetadataParamsSchema = z.object({});

const clearCacheParamsSchema = z.object({});

const issueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  message: z.string(),
  severity: z.enum(RustSeverity),
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
  ruleType: z.enum(RuleType),
  defaultSeverity: z.enum(Severity),
  defaultEnabled: z.boolean(),
  category: z.enum(RuleCategory),
});

const modifiedLineRangeSchema = z.object({
  startLine: z.number(),
  lineCount: z.number(),
});

const builtinRuleConfigSchema = z.object({
  enabled: z.boolean().optional(),
  severity: z.enum(Severity).optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

const customRuleConfigSchema = z.object({
  type: z.enum(CustomRuleType),
  pattern: z.string().optional(),
  script: z.string().optional(),
  prompt: z.string().optional(),
  message: z.string(),
  severity: z.enum(Severity).optional(),
  enabled: z.boolean().optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

const tscannerConfigSchema = z.object({
  builtinRules: z.record(z.string(), builtinRuleConfigSchema).optional(),
  customRules: z.record(z.string(), customRuleConfigSchema).optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

export type ScanParams = z.infer<typeof scanParamsSchema>;
export type ScanFileParams = z.infer<typeof scanFileParamsSchema>;
export type ScanContentParams = z.infer<typeof scanContentParamsSchema>;
export type GetRulesMetadataParams = z.infer<typeof getRulesMetadataParamsSchema>;
export type ClearCacheParams = z.infer<typeof clearCacheParamsSchema>;
export type Issue = z.infer<typeof issueSchema>;
export type FileResult = z.infer<typeof fileResultSchema>;
export type ScanResult = z.infer<typeof scanResultSchema>;
export type RuleMetadata = z.infer<typeof ruleMetadataSchema>;
export type ModifiedLineRange = z.infer<typeof modifiedLineRangeSchema>;
export type BuiltinRuleConfig = z.infer<typeof builtinRuleConfigSchema>;
export type CustomRuleConfig = z.infer<typeof customRuleConfigSchema>;
export type TscannerConfig = z.infer<typeof tscannerConfigSchema>;

export interface IssueResult {
  uri: vscode.Uri;
  line: number;
  column: number;
  text: string;
  rule: string;
  severity: 'error' | 'warning';
  message: string;
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

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;

  return !!(
    (config.builtinRules && Object.keys(config.builtinRules).length > 0) ||
    (config.customRules && Object.keys(config.customRules).length > 0)
  );
}
