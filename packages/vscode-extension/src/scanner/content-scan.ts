import type { TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { IssueResult } from '../common/types';
import { ensureLspClient } from './client';
import { mapIssueToResult } from './utils';

type ScanContentResult = {
  issues: IssueResult[];
  relatedFiles: string[];
};

export async function scanContent(
  filePath: string,
  content: string,
  config?: TscannerConfig,
  configDir?: string,
  branch?: string,
  uncommitted?: boolean,
): Promise<ScanContentResult> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return { issues: [], relatedFiles: [] };
  }

  try {
    const client = await ensureLspClient();
    const result = await client.scanContent(
      workspaceFolder.uri.fsPath,
      filePath,
      content,
      config,
      configDir,
      branch,
      uncommitted,
    );

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
