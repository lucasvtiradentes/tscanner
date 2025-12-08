import { basename } from 'node:path';
import { IssueRuleType, type ViewMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId } from '../../common/constants';
import { Command, TreeItemContextValue, formatIssueCount } from '../../common/lib/vscode-utils';
import { type FolderNode, type IssueResult, NodeKind } from '../../common/types';
import { getFolderIssueCount } from './tree-builder';

function getRuleTypeIcon(ruleType?: IssueRuleType): vscode.ThemeIcon {
  switch (ruleType) {
    case IssueRuleType.Builtin:
      return new vscode.ThemeIcon('symbol-keyword');
    case IssueRuleType.CustomRegex:
      return new vscode.ThemeIcon('regex');
    case IssueRuleType.CustomScript:
      return new vscode.ThemeIcon('terminal');
    case IssueRuleType.Ai:
      return new vscode.ThemeIcon('sparkle');
    default:
      return new vscode.ThemeIcon('list-filter');
  }
}

export class RuleGroupItem extends vscode.TreeItem {
  constructor(
    public readonly rule: string,
    public readonly results: IssueResult[],
    public readonly viewMode: ViewMode,
  ) {
    super(rule, vscode.TreeItemCollapsibleState.Collapsed);

    this.description = formatIssueCount(results.length);
    const ruleType = results[0]?.ruleType;
    this.iconPath = getRuleTypeIcon(ruleType);
    this.contextValue = TreeItemContextValue.RuleGroup;
  }
}

export class FolderResultItem extends vscode.TreeItem {
  constructor(public readonly node: FolderNode) {
    super(node.name, vscode.TreeItemCollapsibleState.Collapsed);

    this.description = formatIssueCount(getFolderIssueCount(node));
    this.iconPath = new vscode.ThemeIcon(NodeKind.Folder);
    this.contextValue = TreeItemContextValue.Folder;
  }
}

export class FileResultItem extends vscode.TreeItem {
  constructor(
    public readonly filePath: string,
    public readonly results: IssueResult[],
  ) {
    super(basename(filePath), vscode.TreeItemCollapsibleState.Collapsed);

    this.description = formatIssueCount(results.length);
    this.iconPath = new vscode.ThemeIcon(NodeKind.File);
    this.contextValue = TreeItemContextValue.File;
    this.resourceUri = vscode.Uri.file(filePath);
  }
}

export class LineResultItem extends vscode.TreeItem {
  constructor(public readonly result: IssueResult) {
    super(result.text, vscode.TreeItemCollapsibleState.None);

    this.description = `Ln ${result.line + 1}, Col ${result.column + 1}`;
    this.tooltip = result.text;

    this.command = {
      command: getCommandId(Command.OpenFile),
      title: 'Open File',
      arguments: [result.uri, result.line, result.column],
    };

    this.iconPath = getRuleTypeIcon(result.ruleType);
    this.contextValue = TreeItemContextValue.Issue;
  }
}
