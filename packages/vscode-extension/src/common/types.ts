import {
  type BuiltinRuleConfig,
  type ClearCacheParams,
  type CustomRuleConfig,
  type FileResult,
  type GetRulesMetadataParams,
  type Issue,
  type ModifiedLineRange,
  type RuleMetadata,
  type ScanContentParams,
  type ScanFileParams,
  type ScanParams,
  type ScanResult,
  type TscannerConfig,
  hasConfiguredRules,
} from 'tscanner-common';
import type * as vscode from 'vscode';

export {
  type BuiltinRuleConfig,
  type ClearCacheParams,
  type CustomRuleConfig,
  type FileResult,
  type GetRulesMetadataParams,
  type Issue,
  type ModifiedLineRange,
  type RuleMetadata,
  type ScanContentParams,
  type ScanFileParams,
  type ScanParams,
  type ScanResult,
  type TscannerConfig,
  hasConfiguredRules,
};

export type IssueResult = {
  uri: vscode.Uri;
  line: number;
  column: number;
  endColumn: number;
  text: string;
  rule: string;
  severity: 'error' | 'warning';
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
