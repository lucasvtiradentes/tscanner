import type { CliGroupBy } from 'tscanner-common';
import { copyIssuesBase, copyScanContext } from '../../common/lib/copy-utils';
import { Command, registerCommand } from '../../common/lib/vscode-utils';
import type { IssueResult } from '../../common/types';

export function createCopyAllIssuesCommand(getResults: () => IssueResult[], getGroupMode: () => CliGroupBy) {
  return registerCommand(Command.CopyAllIssues, async () => {
    const results = getResults();
    const groupMode = getGroupMode();

    await copyIssuesBase({
      results,
      groupMode,
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand(groupMode);
        return `TScanner report with all issues in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${results.length} issues`,
    });
  });
}
