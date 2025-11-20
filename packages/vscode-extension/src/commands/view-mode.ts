import * as vscode from 'vscode';
import { Command, WorkspaceStateKey, registerCommand, updateState } from '../common/lib/vscode-utils';
import { SearchResultProvider } from '../sidebar/search-provider';

export function createSetListViewCommand(searchProvider: SearchResultProvider, context: vscode.ExtensionContext) {
  return registerCommand(Command.SetListView, () => {
    searchProvider.viewMode = 'list';
    updateState(context, WorkspaceStateKey.ViewMode, 'list');
  });
}

export function createSetTreeViewCommand(searchProvider: SearchResultProvider, context: vscode.ExtensionContext) {
  return registerCommand(Command.SetTreeView, () => {
    searchProvider.viewMode = 'tree';
    updateState(context, WorkspaceStateKey.ViewMode, 'tree');
  });
}

export function createSetGroupByDefaultCommand(searchProvider: SearchResultProvider, context: vscode.ExtensionContext) {
  return registerCommand(Command.SetGroupByDefault, () => {
    searchProvider.groupMode = 'default';
    updateState(context, WorkspaceStateKey.GroupMode, 'default');
  });
}

export function createSetGroupByRuleCommand(searchProvider: SearchResultProvider, context: vscode.ExtensionContext) {
  return registerCommand(Command.SetGroupByRule, () => {
    searchProvider.groupMode = 'rule';
    updateState(context, WorkspaceStateKey.GroupMode, 'rule');
  });
}
