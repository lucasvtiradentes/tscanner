import * as vscode from 'vscode';

export interface AnyUsageResult {
  uri: vscode.Uri;
  line: number;
  column: number;
  text: string;
  type: 'colonAny' | 'asAny';
}

async function processFile(fileUri: vscode.Uri): Promise<AnyUsageResult[]> {
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

export async function findAnyTypes(): Promise<AnyUsageResult[]> {
  const files = await vscode.workspace.findFiles(
    '**/*.{ts,tsx}',
    '**/node_modules/**'
  );

  const chunkSize = 10;
  const allResults: AnyUsageResult[] = [];

  for (let i = 0; i < files.length; i += chunkSize) {
    const chunk = files.slice(i, i + chunkSize);
    const chunkResults = await Promise.all(chunk.map(processFile));
    allResults.push(...chunkResults.flat());
  }

  return allResults;
}
