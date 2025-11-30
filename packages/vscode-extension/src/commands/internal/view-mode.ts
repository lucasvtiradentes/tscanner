import { GroupMode, ViewMode } from 'tscanner-common';
import type * as vscode from 'vscode';
import { Command, WorkspaceStateKey, registerCommand, updateState } from '../../common/lib/vscode-utils';
import type { IssuesPanelContent } from '../../issues-panel/panel-content';

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

function cycleToNextMode(panelContent: IssuesPanelContent, context: vscode.ExtensionContext) {
  const currentViewMode = panelContent.viewMode;
  const currentGroupMode = panelContent.groupMode;

  const currentIndex = VIEW_STATES.findIndex(
    (state) => state.viewMode === currentViewMode && state.groupMode === currentGroupMode,
  );

  const nextIndex = (currentIndex + 1) % VIEW_STATES.length;
  const nextState = VIEW_STATES[nextIndex];

  panelContent.viewMode = nextState.viewMode;
  panelContent.groupMode = nextState.groupMode;

  updateState(context, WorkspaceStateKey.ViewMode, nextState.viewMode);
  updateState(context, WorkspaceStateKey.GroupMode, nextState.groupMode);
}

export function createCycleViewModeCommand(panelContent: IssuesPanelContent, context: vscode.ExtensionContext) {
  return registerCommand(Command.CycleViewMode, () => cycleToNextMode(panelContent, context));
}
