import * as vscode from 'vscode';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { IssueResult } from '../common/types';
import type { ScanContentRequestOptions } from '../lsp/requests/types';
import { ensureLspClient } from './client';
import { mapIssueToResult } from './utils';

type ScanContentResult = {
  issues: IssueResult[];
  relatedFiles: string[];
};

type ScanContentParams = ScanContentRequestOptions & {
  filePath: string;
};

export async function scanContent(params: ScanContentParams): Promise<ScanContentResult> {
  const { filePath, content, config, configDir, branch, uncommitted } = params;
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return { issues: [], relatedFiles: [] };
  }

  try {
    const client = await ensureLspClient();
    const result = await client.scanContent({
      root: workspaceFolder.uri.fsPath,
      file: filePath,
      content,
      config,
      configDir,
      branch,
      uncommitted,
    });

    logger.debug(`scanContent() returned ${result.issues.length} results for ${filePath}`);

    const issues = result.issues.map((issue) => {
      const issueFile = issue.file ?? result.file;
      const uri = vscode.Uri.file(issueFile);
      return mapIssueToResult(uri, issue);
    });

    return {
      issues,
      relatedFiles: result.related_files ?? [],
    };
  } catch (error) {
    logger.error(`Failed to scan content for ${filePath}: ${error}`);
    throw error;
  }
}
