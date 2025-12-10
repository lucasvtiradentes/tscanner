import { type IssueRuleType, Severity } from 'tscanner-common';
import type * as vscode from 'vscode';

export type IssueResult = {
  uri: vscode.Uri;
  line: number;
  column: number;
  endColumn: number;
  text: string;
  rule: string;
  severity: Severity;
  message: string;
  ruleType?: IssueRuleType;
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
  switch (severity.toLowerCase()) {
    case 'error':
      return Severity.Error;
    case 'info':
      return Severity.Info;
    case 'hint':
      return Severity.Hint;
    default:
      return Severity.Warning;
  }
}

type SerializedIssueResult = Omit<IssueResult, 'uri'> & { uriString: string };

export function serializeResults(results: IssueResult[]): SerializedIssueResult[] {
  return results.map((r) => {
    const { uri, ...rest } = r;
    return { ...rest, uriString: uri.toString() };
  });
}
