import * as vscode from 'vscode';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder, openTextDocument } from '../common/lib/vscode-utils';
import type { IssueResult, TscannerConfig } from '../common/types';
import { ensureLspClient } from './client';
import {
  getRustBinaryPath,
  mapIssueToResult,
  parseConfigError,
  showBinaryNotFoundError,
  showConfigErrorToast,
  showScanErrorToast,
} from './utils';

export async function scanCodebase(fileFilter?: Set<string>, config?: TscannerConfig): Promise<IssueResult[]> {
  const workspaceFolder = getCurrentWorkspaceFolder();
  if (!workspaceFolder) {
    return [];
  }

  const binaryPath = getRustBinaryPath();
  if (!binaryPath) {
    logger.error('Rust binary not found');
    showBinaryNotFoundError();
    throw new Error('Rust binary not found');
  }

  try {
    const client = await ensureLspClient();

    const scanStart = Date.now();
    const result = await client.scan(workspaceFolder.uri.fsPath, config);
    const scanTime = Date.now() - scanStart;

    logger.info(
      `Codebase scan completed: ${result.total_issues} issues in ${result.duration_ms}ms (client: ${scanTime}ms)`,
    );

    const processStart = Date.now();

    let filesToLoad = [...new Set(result.files.map((f) => f.file))];

    if (fileFilter && fileFilter.size > 0) {
      filesToLoad = filesToLoad.filter((filePath) => {
        const relativePath = vscode.workspace.asRelativePath(vscode.Uri.file(filePath));
        return fileFilter.has(relativePath);
      });

      logger.debug(
        `Filtered ${result.files.length} → ${filesToLoad.length} files to load (fileFilter has ${fileFilter.size} entries)`,
      );
    }

    const results: IssueResult[] = [];

    for (const fileResult of result.files) {
      const uri = vscode.Uri.file(fileResult.file);

      for (const issue of fileResult.issues) {
        let lineText = issue.line_text ?? '';

        if (!lineText && fileFilter && fileFilter.has(vscode.workspace.asRelativePath(uri))) {
          try {
            const document = await openTextDocument(uri);
            lineText = document.lineAt(issue.line - 1).text;
          } catch (_error) {
            logger.error(`Failed to load line text for: ${fileResult.file}`);
            lineText = '';
          }
        }

        results.push(mapIssueToResult(uri, issue, lineText));
      }
    }

    const processTime = Date.now() - processStart;
    logger.debug(
      `Post-processing ${result.total_issues} issues from ${filesToLoad.length} files took ${processTime}ms total`,
    );

    return results;
  } catch (error) {
    logger.error(`Codebase scan failed: ${error}`);

    const errorMessage = String(error);
    const configError = parseConfigError(errorMessage);

    if (configError) {
      showConfigErrorToast(configError);
    } else {
      showScanErrorToast(error);
    }

    throw error;
  }
}
