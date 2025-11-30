import { copyIssuesBase, copyScanContext } from '../../common/lib/copy-utils';
import { Command, registerCommand } from '../../common/lib/vscode-utils';
import type { GroupMode, IssueResult } from '../../common/types';

export function createCopyAllIssuesCommand(getResults: () => IssueResult[], getGroupMode: () => GroupMode) {
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
