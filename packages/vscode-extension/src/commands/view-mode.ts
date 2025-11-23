import type * as vscode from 'vscode';
import {
  Command,
  GroupMode,
  ViewMode,
  WorkspaceStateKey,
  registerCommand,
  updateState,
} from '../common/lib/vscode-utils';
import type { SearchResultProvider } from '../sidebar/search-provider';

type ViewState = {
  viewMode: ViewMode;
  groupMode: GroupMode;
};

const VIEW_STATES: ViewState[] = [
  { viewMode: ViewMode.List, groupMode: GroupMode.Default },
  { viewMode: ViewMode.Tree, groupMode: GroupMode.Default },
  { viewMode: ViewMode.List, groupMode: GroupMode.Rule },
  { viewMode: ViewMode.Tree, groupMode: GroupMode.Rule },
];

function cycleToNextMode(searchProvider: SearchResultProvider, context: vscode.ExtensionContext) {
  const currentViewMode = searchProvider.viewMode;
  const currentGroupMode = searchProvider.groupMode;

  const currentIndex = VIEW_STATES.findIndex(
    (state) => state.viewMode === currentViewMode && state.groupMode === currentGroupMode,
  );

  const nextIndex = (currentIndex + 1) % VIEW_STATES.length;
  const nextState = VIEW_STATES[nextIndex];

  searchProvider.viewMode = nextState.viewMode;
  searchProvider.groupMode = nextState.groupMode;

  updateState(context, WorkspaceStateKey.ViewMode, nextState.viewMode);
  updateState(context, WorkspaceStateKey.GroupMode, nextState.groupMode);
}

export function createCycleViewModeFileFlatViewCommand(
  searchProvider: SearchResultProvider,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeFileFlatView, () => cycleToNextMode(searchProvider, context));
}

export function createCycleViewModeFileTreeViewCommand(
  searchProvider: SearchResultProvider,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeFileTreeView, () => cycleToNextMode(searchProvider, context));
}

export function createCycleViewModeRuleFlatViewCommand(
  searchProvider: SearchResultProvider,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeRuleFlatView, () => cycleToNextMode(searchProvider, context));
}

export function createCycleViewModeRuleTreeViewCommand(
  searchProvider: SearchResultProvider,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeRuleTreeView, () => cycleToNextMode(searchProvider, context));
}
