import { copyIssuesBase, copyScanContext } from '../../common/lib/copy-utils';
import { Command, registerCommand } from '../../common/lib/vscode-utils';
import type { IssueResult } from '../../common/types';

export function createCopyAllIssuesCommand(getResults: () => IssueResult[]) {
  return registerCommand(Command.CopyAllIssues, async () => {
    const results = getResults();

    await copyIssuesBase({
      results,
      groupMode: 'file',
      buildHeader: (summary) => {
        const cliCommand = copyScanContext.buildCliCommand('glob', '**/*');
        return `TScanner report with all issues in the ${copyScanContext.getScanModeText()}\n\ncli command: ${cliCommand}\nfound issues: ${summary.total_issues} issues\n`;
      },
      successMessage: `Copied ${results.length} issues`,
    });
  });
}
