import * as vscode from 'vscode';
import { collectFolderIssues, copyIssuesBase, copyScanContext } from '../../common/lib/copy-utils';
import { Command, ToastKind, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';
import type { FileResultItem, FolderResultItem, RuleGroupItem } from '../../issues-panel/utils/tree-items';

export function createCopyRuleIssuesCommand() {
  return registerCommand(Command.CopyRuleIssues, async (item: RuleGroupItem) => {
    if (!item?.results) return;

    await copyIssuesBase({
      results: item.results,
      groupMode: 'rule',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('rule', item.rule);
        return `TScanner report searching for all the issues of the rule "${item.rule}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
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
      groupMode: 'file',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('glob', relativePath);
        return `TScanner report searching for all the issues in file "${relativePath}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
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
      groupMode: 'file',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('glob', `${relativeFolderPath}/**/*`);
        return `TScanner report searching for all the issues in folder "${item.node.name}" in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${allResults.length} issues from folder "${item.node.name}"`,
    });
  });
}
