import { GroupMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { collectFolderIssues, copyIssuesBase } from '../../common/lib/copy-utils';
import { Command, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import type { IssueResult } from '../../common/types';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../../issues-panel';

type CopyMode = 'regular' | 'ai';
type CopyScope = 'rule' | 'file' | 'folder' | 'all';

type CopyCommandConfig = {
  mode: CopyMode;
  scope: CopyScope;
};

type AllScopeContext = {
  getResults: () => IssueResult[];
  getGroupMode: () => GroupMode;
};

const COMMAND_MAP: Record<CopyMode, Record<CopyScope, Command>> = {
  regular: {
    rule: Command.CopyRuleIssues,
    file: Command.CopyFileIssues,
    folder: Command.CopyFolderIssues,
    all: Command.CopyAllIssues,
  },
  ai: {
    rule: Command.CopyAiRuleIssues,
    file: Command.CopyAiFileIssues,
    folder: Command.CopyAiFolderIssues,
    all: Command.CopyAllAiIssues,
  },
};

function getFilterTypeLabel(mode: CopyMode, scope: CopyScope): string {
  const prefix = mode === 'ai' ? 'AI ' : '';
  const scopeLabels: Record<CopyScope, string> = {
    rule: 'rule',
    file: 'file',
    folder: 'folder',
    all: 'all issues',
  };
  return `${prefix}${scopeLabels[scope]}`;
}

function createRuleCopyHandler(config: CopyCommandConfig) {
  return async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.Rule,
      filterType: getFilterTypeLabel(config.mode, 'rule'),
      filterValue: item.rule,
      cliFilter: 'rule',
      cliFilterValue: item.rule,
      onlyAi: config.mode === 'ai',
      successMessage: `Copied ${item.results.length} ${config.mode === 'ai' ? 'AI ' : ''}issues from "${item.rule}"`,
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
      filterType: getFilterTypeLabel(config.mode, 'file'),
      filterValue: relativePath,
      cliFilter: 'glob',
      cliFilterValue: relativePath,
      onlyAi: config.mode === 'ai',
      successMessage: `Copied ${item.results.length} ${config.mode === 'ai' ? 'AI ' : ''}issues from "${relativePath}"`,
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
      filterType: getFilterTypeLabel(config.mode, 'folder'),
      filterValue: item.node.name,
      cliFilter: 'glob',
      cliFilterValue: `${relativeFolderPath}/**/*`,
      onlyAi: config.mode === 'ai',
      successMessage: `Copied ${allResults.length} ${config.mode === 'ai' ? 'AI ' : ''}issues from folder "${item.node.name}"`,
    });
  };
}

function createAllCopyHandler(config: CopyCommandConfig, ctx: AllScopeContext) {
  return async () => {
    const results = ctx.getResults();

    await copyIssuesBase({
      results,
      groupMode: ctx.getGroupMode(),
      filterType: getFilterTypeLabel(config.mode, 'all'),
      onlyAi: config.mode === 'ai',
      successMessage: `Copied ${results.length} ${config.mode === 'ai' ? 'AI ' : ''}issues`,
    });
  };
}

export function createCopyCommand(mode: CopyMode, scope: 'rule'): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: 'file'): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: 'folder'): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: 'all', ctx: AllScopeContext): vscode.Disposable;
export function createCopyCommand(mode: CopyMode, scope: CopyScope, ctx?: AllScopeContext): vscode.Disposable {
  const command = COMMAND_MAP[mode][scope];
  const config: CopyCommandConfig = { mode, scope };

  switch (scope) {
    case 'rule':
      return registerCommand(command, createRuleCopyHandler(config));
    case 'file':
      return registerCommand(command, createFileCopyHandler(config));
    case 'folder':
      return registerCommand(command, createFolderCopyHandler(config));
    case 'all':
      return registerCommand(command, createAllCopyHandler(config, ctx!));
  }
}
