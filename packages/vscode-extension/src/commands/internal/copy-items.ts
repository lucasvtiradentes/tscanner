import { GroupMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { collectFolderIssues, copyIssuesBase } from '../../common/lib/copy-utils';
import { Command, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../../issues-panel';

export function createCopyRuleIssuesCommand() {
  return registerCommand(Command.CopyRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.Rule,
      filterType: 'rule',
      filterValue: item.rule,
      cliFilter: 'rule',
      cliFilterValue: item.rule,
      successMessage: `Copied ${item.results.length} issues from "${item.rule}"`,
    });
  });
}

export function createCopyFileIssuesCommand() {
  return registerCommand(Command.CopyFileIssues, async (item: FileResultItem) => {
    if (!item?.results) return;

    const relativePath = vscode.workspace.asRelativePath(item.filePath);

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.File,
      filterType: 'file',
      filterValue: relativePath,
      cliFilter: 'glob',
      cliFilterValue: relativePath,
      successMessage: `Copied ${item.results.length} issues from "${relativePath}"`,
    });
  });
}

export function createCopyFolderIssuesCommand() {
  return registerCommand(Command.CopyFolderIssues, async (item: FolderResultItem) => {
    if (!item?.node) {
      showToastMessage(ToastKind.Error, 'No folder data available');
      return;
    }

    const allResults = collectFolderIssues(item.node);
    const relativeFolderPath = vscode.workspace.asRelativePath(item.node.path);

    await copyIssuesBase({
      results: allResults,
      groupMode: GroupMode.File,
      filterType: 'folder',
      filterValue: item.node.name,
      cliFilter: 'glob',
      cliFilterValue: `${relativeFolderPath}/**/*`,
      successMessage: `Copied ${allResults.length} issues from folder "${item.node.name}"`,
    });
  });
}

export function createCopyAiRuleIssuesCommand() {
  return registerCommand(Command.CopyAiRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.Rule,
      filterType: 'AI rule',
      filterValue: item.rule,
      cliFilter: 'rule',
      cliFilterValue: item.rule,
      onlyAi: true,
      successMessage: `Copied ${item.results.length} AI issues from "${item.rule}"`,
    });
  });
}

export function createCopyAiFileIssuesCommand() {
  return registerCommand(Command.CopyAiFileIssues, async (item: FileResultItem) => {
    if (!item?.results) return;

    const relativePath = vscode.workspace.asRelativePath(item.filePath);

    await copyIssuesBase({
      results: item.results,
      groupMode: GroupMode.File,
      filterType: 'AI file',
      filterValue: relativePath,
      cliFilter: 'glob',
      cliFilterValue: relativePath,
      onlyAi: true,
      successMessage: `Copied ${item.results.length} AI issues from "${relativePath}"`,
    });
  });
}

export function createCopyAiFolderIssuesCommand() {
  return registerCommand(Command.CopyAiFolderIssues, async (item: FolderResultItem) => {
    if (!item?.node) {
      showToastMessage(ToastKind.Error, 'No folder data available');
      return;
    }

    const allResults = collectFolderIssues(item.node);
    const relativeFolderPath = vscode.workspace.asRelativePath(item.node.path);

    await copyIssuesBase({
      results: allResults,
      groupMode: GroupMode.File,
      filterType: 'AI folder',
      filterValue: item.node.name,
      cliFilter: 'glob',
      cliFilterValue: `${relativeFolderPath}/**/*`,
      onlyAi: true,
      successMessage: `Copied ${allResults.length} AI issues from folder "${item.node.name}"`,
    });
  });
}
