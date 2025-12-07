import type { GroupMode } from 'tscanner-common';
import { copyIssuesBase } from '../../common/lib/copy-utils';
import { Command, registerCommand } from '../../common/lib/vscode-utils';
import type { IssueResult } from '../../common/types';

export function createCopyAllIssuesCommand(getResults: () => IssueResult[], getGroupMode: () => GroupMode) {
  return registerCommand(Command.CopyAllIssues, async () => {
    const results = getResults();

    await copyIssuesBase({
      results,
      groupMode: getGroupMode(),
      filterType: 'all issues',
      successMessage: `Copied ${results.length} issues`,
    });
  });
}

export function createCopyAllAiIssuesCommand(getResults: () => IssueResult[], getGroupMode: () => GroupMode) {
  return registerCommand(Command.CopyAllAiIssues, async () => {
    const results = getResults();

    await copyIssuesBase({
      results,
      groupMode: getGroupMode(),
      filterType: 'all AI issues',
      onlyAi: true,
      successMessage: `Copied ${results.length} AI issues`,
    });
  });
}
