import type { AiExecutionMode, TscannerConfig } from 'tscanner-common';
import * as vscode from 'vscode';
import { logger } from '../common/lib/logger';
import { getCurrentWorkspaceFolder, openTextDocument } from '../common/lib/vscode-utils';
import type { IssueResult } from '../common/types';
import { ensureLspClient } from './client';
import { mapIssueToResult, parseConfigError, showConfigErrorToast, showScanErrorToast } from './utils';

type ScanOptions = {
  branch?: string;
  staged?: boolean;
  fileFilter?: Set<string>;
  config?: TscannerConfig;
  configDir?: string;
  aiMode?: AiExecutionMode;
  noCache?: boolean;
};

export async function scan(options: ScanOptions = {}): Promise<IssueResult[]> {
  const { branch, staged, fileFilter, config, configDir, aiMode, noCache } = options;
  const workspaceFolder = getCurrentWorkspaceFolder();

  if (!workspaceFolder) {
    return [];
  }

  const getScanType = () => {
    if (staged) return 'Uncommitted';
    if (branch) return 'Branch';
    return 'Codebase';
  };
  const scanType = getScanType();

  try {
    const client = await ensureLspClient();

    logger.info(
      `Calling LSP scan: scanType=${scanType}, noCache=${noCache ?? false}, branch=${branch ?? 'none'}, staged=${staged ?? false}`,
    );
    const scanStart = Date.now();
    const result = await client.scan(workspaceFolder.uri.fsPath, config, configDir, branch, staged, aiMode, noCache);
    const scanTime = Date.now() - scanStart;

    logger.info(
      `${scanType} scan completed: ${result.total_issues} issues in ${result.duration_ms}ms (files: ${result.total_files}, cached: ${result.cached_files}, scanned: ${result.scanned_files}) (client: ${scanTime}ms)`,
    );

    const processStart = Date.now();

    let filesToLoad = [...new Set(result.files.map((f) => f.file))];

    if (fileFilter && fileFilter.size > 0) {
      filesToLoad = filesToLoad.filter((filePath) => {
        const relativePath = vscode.workspace.asRelativePath(vscode.Uri.file(filePath));
        return fileFilter.has(relativePath);
      });

      logger.debug(`Filtered ${result.files.length} → ${filesToLoad.length} files (fileFilter: ${fileFilter.size})`);
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
          } catch {
            logger.error(`Failed to load line text for: ${fileResult.file}`);
            lineText = '';
          }
        }

        results.push(mapIssueToResult(uri, issue, lineText));
      }
    }

    const processTime = Date.now() - processStart;
    logger.debug(
      `Post-processing ${result.total_issues} issues from ${filesToLoad.length} files took ${processTime}ms`,
    );

    return results;
  } catch (error) {
    logger.error(`${scanType} scan failed: ${error}`);

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
