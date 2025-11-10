import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { RustClient } from './rustClient';
import { logger } from './logger';

export interface AnyUsageResult {
  uri: vscode.Uri;
  line: number;
  column: number;
  text: string;
  type: 'colonAny' | 'asAny';
}

let rustClient: RustClient | null = null;

function getRustBinaryPath(): string | null {
  const extensionPath = vscode.extensions.getExtension('lucasvtiradentes.lino')?.extensionPath;
  if (!extensionPath) return null;

  const binaryPath = path.join(
    extensionPath,
    '..',
    '..',
    'lino-core',
    'target',
    'release',
    'lino-server'
  );

  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }

  return null;
}

async function processFileFallback(fileUri: vscode.Uri): Promise<AnyUsageResult[]> {
  const results: AnyUsageResult[] = [];
  const document = await vscode.workspace.openTextDocument(fileUri);
  const text = document.getText();

  const colonAnyRegex = /:\s*any\b/g;
  let match;
  while ((match = colonAnyRegex.exec(text)) !== null) {
    const position = document.positionAt(match.index);
    results.push({
      uri: fileUri,
      line: position.line,
      column: position.character,
      text: document.lineAt(position.line).text.trim(),
      type: 'colonAny'
    });
  }

  const asAnyRegex = /\bas\s+any\b/g;
  while ((match = asAnyRegex.exec(text)) !== null) {
    const position = document.positionAt(match.index);
    results.push({
      uri: fileUri,
      line: position.line,
      column: position.character,
      text: document.lineAt(position.line).text.trim(),
      type: 'asAny'
    });
  }

  return results;
}

async function findAnyTypesFallback(): Promise<AnyUsageResult[]> {
  const files = await vscode.workspace.findFiles(
    '**/*.{ts,tsx}',
    '**/node_modules/**'
  );

  const chunkSize = 10;
  const allResults: AnyUsageResult[] = [];

  for (let i = 0; i < files.length; i += chunkSize) {
    const chunk = files.slice(i, i + chunkSize);
    const chunkResults = await Promise.all(chunk.map(processFileFallback));
    allResults.push(...chunkResults.flat());
  }

  return allResults;
}

export async function findAnyTypes(): Promise<AnyUsageResult[]> {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  if (!workspaceFolder) {
    return [];
  }

  const binaryPath = getRustBinaryPath();

  if (binaryPath) {
    try {
      logger.info('Using Rust backend for scanning');

      if (!rustClient) {
        rustClient = new RustClient(binaryPath);
        await rustClient.start();
      }

      const results = await rustClient.scan(workspaceFolder.uri.fsPath);
      return results;
    } catch (error) {
      logger.error(`Rust backend failed, falling back to TypeScript: ${error}`);
      rustClient = null;
    }
  } else {
    logger.info('Rust binary not found, using TypeScript fallback');
  }

  return findAnyTypesFallback();
}

export function dispose() {
  if (rustClient) {
    rustClient.stop();
    rustClient = null;
  }
}
