import * as vscode from 'vscode';
import {
  Command,
  ToastKind,
  navigateToPosition,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import { logger } from '../../common/utils/logger';
import type { IssuesPanelContent } from '../../issues-panel/panel-content';

let currentIssueIndex = -1;

async function navigateToIssue(panelContent: IssuesPanelContent, direction: 'next' | 'previous'): Promise<void> {
  const results = panelContent.getResults();

  if (results.length === 0) {
    showToastMessage(ToastKind.Info, 'No issues found');
    return;
  }

  if (direction === 'next') {
    currentIssueIndex = (currentIssueIndex + 1) % results.length;
  } else {
    if (currentIssueIndex === -1) {
      currentIssueIndex = results.length - 1;
    } else {
      currentIssueIndex = (currentIssueIndex - 1 + results.length) % results.length;
    }
  }

  const issue = results[currentIssueIndex];
  logger.debug(`Navigating to ${direction} issue: ${currentIssueIndex + 1}/${results.length}`);

  await navigateToPosition(issue.uri, issue.line, issue.column);
  vscode.window.setStatusBarMessage(`Issue ${currentIssueIndex + 1}/${results.length}: ${issue.rule}`, 3000);
}

export function createGoToNextIssueCommand(panelContent: IssuesPanelContent) {
  return registerCommand(Command.GoToNextIssue, () => navigateToIssue(panelContent, 'next'));
}

export function createGoToPreviousIssueCommand(panelContent: IssuesPanelContent) {
  return registerCommand(Command.GoToPreviousIssue, () => navigateToIssue(panelContent, 'previous'));
}

export function resetIssueIndex() {
  currentIssueIndex = -1;
}
