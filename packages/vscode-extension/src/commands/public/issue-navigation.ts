import * as vscode from 'vscode';
import { logger } from '../../common/lib/logger';
import {
  Command,
  ToastKind,
  navigateToPosition,
  registerCommand,
  showToastMessage,
} from '../../common/lib/vscode-utils';
import type { RegularIssuesView } from '../../issues-panel';

enum NavigationDirection {
  Next = 'next',
  Previous = 'previous',
}

let currentIssueIndex = -1;

async function navigateToIssue(regularView: RegularIssuesView, direction: NavigationDirection): Promise<void> {
  const results = regularView.getResults();

  if (results.length === 0) {
    showToastMessage(ToastKind.Info, 'No issues found');
    return;
  }

  if (direction === NavigationDirection.Next) {
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

export function createGoToNextIssueCommand(regularView: RegularIssuesView) {
  return registerCommand(Command.GoToNextIssue, () => navigateToIssue(regularView, NavigationDirection.Next));
}

export function createGoToPreviousIssueCommand(regularView: RegularIssuesView) {
  return registerCommand(Command.GoToPreviousIssue, () => navigateToIssue(regularView, NavigationDirection.Previous));
}

export function resetIssueIndex() {
  currentIssueIndex = -1;
}
