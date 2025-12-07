import { GroupMode, ViewMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import { Command, registerCommand } from '../../common/lib/vscode-utils';
import { WorkspaceStateKey, updateState } from '../../common/state/workspace-state';
import type { AiIssuesView, RegularIssuesView } from '../../issues-panel';

type ViewState = {
  viewMode: ViewMode;
  groupMode: GroupMode;
};

const VIEW_STATES: ViewState[] = [
  { viewMode: ViewMode.List, groupMode: GroupMode.File },
  { viewMode: ViewMode.Tree, groupMode: GroupMode.File },
  { viewMode: ViewMode.List, groupMode: GroupMode.Rule },
  { viewMode: ViewMode.Tree, groupMode: GroupMode.Rule },
];

function cycleToNextMode(regularView: RegularIssuesView, aiView: AiIssuesView, context: vscode.ExtensionContext) {
  const currentViewMode = regularView.viewMode;
  const currentGroupMode = regularView.groupMode;

  const currentIndex = VIEW_STATES.findIndex(
    (state) => state.viewMode === currentViewMode && state.groupMode === currentGroupMode,
  );

  const nextIndex = (currentIndex + 1) % VIEW_STATES.length;
  const nextState = VIEW_STATES[nextIndex];

  regularView.viewMode = nextState.viewMode;
  regularView.groupMode = nextState.groupMode;
  aiView.viewMode = nextState.viewMode;
  aiView.groupMode = nextState.groupMode;

  updateState(context, WorkspaceStateKey.ViewMode, nextState.viewMode);
  updateState(context, WorkspaceStateKey.GroupMode, nextState.groupMode);
}

export function createCycleViewModeFileFlatViewCommand(
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeFileFlatView, () => cycleToNextMode(regularView, aiView, context));
}

export function createCycleViewModeFileTreeViewCommand(
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeFileTreeView, () => cycleToNextMode(regularView, aiView, context));
}

export function createCycleViewModeRuleFlatViewCommand(
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeRuleFlatView, () => cycleToNextMode(regularView, aiView, context));
}

export function createCycleViewModeRuleTreeViewCommand(
  regularView: RegularIssuesView,
  aiView: AiIssuesView,
  context: vscode.ExtensionContext,
) {
  return registerCommand(Command.CycleViewModeRuleTreeView, () => cycleToNextMode(regularView, aiView, context));
}
