import { GroupMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { collectFolderIssues, copyIssuesBase } from '../../common/lib/copy-utils';
import { Command, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import type { IssueResult } from '../../common/types';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../../issues-panel';

export enum CopyMode {
  Regular = 'regular',
  Ai = 'ai',
}

export enum CopyScope {
  Rule = 'rule',
  File = 'file',
  Folder = 'folder',
  All = 'all',
}

type CopyCommandConfig = {
  mode: CopyMode;
  scope: CopyScope;
};

type AllScopeContext = {
  getResults: () => IssueResult[];
  getGroupMode: () => GroupMode;
};

const COMMAND_MAP: Record<CopyMode, Record<CopyScope, Command>> = {
  [CopyMode.Regular]: {
    [CopyScope.Rule]: Command.CopyRuleIssues,
    [CopyScope.File]: Command.CopyFileIssues,
    [CopyScope.Folder]: Command.CopyFolderIssues,
    [CopyScope.All]: Command.CopyAllIssues,
  },
  [CopyMode.Ai]: {
    [CopyScope.Rule]: Command.CopyAiRuleIssues,
    [CopyScope.File]: Command.CopyAiFileIssues,
    [CopyScope.Folder]: Command.CopyAiFolderIssues,
    [CopyScope.All]: Command.CopyAllAiIssues,
  },
};

function getFilterTypeLabel(mode: CopyMode, scope: CopyScope): string {
  const prefix = mode === CopyMode.Ai ? 'AI ' : '';
  const scopeLabels: Record<CopyScope, string> = {
    [CopyScope.Rule]: 'rule',
    [CopyScope.File]: 'file',
    [CopyScope.Folder]: 'folder',
    [CopyScope.All]: 'all issues',
  };
  return `${prefix}${scopeLabels[scope]}`;
}

function createRuleCopyHandler(config: CopyCommandConfig) {
  return async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.Rule,
      filterType: getFilterTypeLabel(config.mode, CopyScope.Rule),
      filterValue: item.rule,
      cliFilter: 'rule',
      cliFilterValue: item.rule,
      onlyAi: config.mode === CopyMode.Ai,
      successMessage: `Copied ${item.results.length} ${config.mode === CopyMode.Ai ? 'AI ' : ''}issues from "${item.rule}"`,
    });
  };
}

function createFileCopyHandler(config: CopyCommandConfig) {
  return async (item: FileResultItem) => {
    if (!item?.results) return;

    const relativePath = vscode.workspace.asRelativePath(item.filePath);

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.File,
      filterType: getFilterTypeLabel(config.mode, CopyScope.File),
      filterValue: relativePath,
      cliFilter: 'glob',
      cliFilterValue: relativePath,
      onlyAi: config.mode === CopyMode.Ai,
      successMessage: `Copied ${item.results.length} ${config.mode === CopyMode.Ai ? 'AI ' : ''}issues from "${relativePath}"`,
    });
  };
}

function createFolderCopyHandler(config: CopyCommandConfig) {
  return async (item: FolderResultItem) => {
    if (!item?.node) {
      showToastMessage(ToastKind.Error, 'No folder data available');
      return;
    }

    const allResults = collectFolderIssues(item.node);
    const relativeFolderPath = vscode.workspace.asRelativePath(item.node.path);

    await copyIssuesBase({
      results: allResults,
      groupMode: GroupMode.File,
      filterType: getFilterTypeLabel(config.mode, CopyScope.Folder),
      filterValue: item.node.name,
      cliFilter: 'glob',
      cliFilterValue: `${relativeFolderPath}/**/*`,
      onlyAi: config.mode === CopyMode.Ai,
      successMessage: `Copied ${allResults.length} ${config.mode === CopyMode.Ai ? 'AI ' : ''}issues from folder "${item.node.name}"`,
    });
  };
}

function createAllCopyHandler(config: CopyCommandConfig, ctx: AllScopeContext) {
  return async () => {
    const results = ctx.getResults();

    await copyIssuesBase({
      results,
      groupMode: ctx.getGroupMode(),
      filterType: getFilterTypeLabel(config.mode, CopyScope.All),
      onlyAi: config.mode === CopyMode.Ai,
      successMessage: `Copied ${results.length} ${config.mode === CopyMode.Ai ? 'AI ' : ''}issues`,
    });
  };
}

export function createCopyCommand(mode: CopyMode, scope: CopyScope.Rule): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: CopyScope.File): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: CopyScope.Folder): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: CopyScope.All, ctx: AllScopeContext): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: CopyScope, ctx?: AllScopeContext): vscode.Disposable {
  const command = COMMAND_MAP[mode][scope];
  const config: CopyCommandConfig = { mode, scope };

  switch (scope) {
    case CopyScope.Rule:
      return registerCommand(command, createRuleCopyHandler(config));
    case CopyScope.File:
      return registerCommand(command, createFileCopyHandler(config));
    case CopyScope.Folder:
      return registerCommand(command, createFolderCopyHandler(config));
    case CopyScope.All:
      return registerCommand(command, createAllCopyHandler(config, ctx!));
  }
}
