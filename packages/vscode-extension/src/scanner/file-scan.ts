import * as vscode from 'vscode';
import { getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import type { IssueResult } from '../common/types';
import { logger } from '../common/utils/logger';
import { ensureLspClient } from './client';
import { mapIssueToResult } from './utils';

export async function scanFile(filePath: string): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return [];
  }

  try {
    const client = await ensureLspClient();
    const result = await client.scanFile(workspaceFolder.uri.fsPath, filePath);

    logger.debug(`scanFile() returned ${result.issues.length} results for ${filePath}`);

    const uri = vscode.Uri.file(result.file);
    return result.issues.map((issue) => mapIssueToResult(uri, issue));
  } catch (error) {
    logger.error(`Failed to scan file ${filePath}: ${error}`);
    throw error;
  }
}
