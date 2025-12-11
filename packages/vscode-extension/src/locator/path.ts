import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { VSCODE_EXTENSION, getBinaryName } from 'tscanner-common';

export async function findInPath(): Promise<string | null> {
  const binaryName = getBinaryName();
  const command = process.platform === 'win32' ? 'where' : 'which';

  try {
    const result = execSync(`${command} ${binaryName}`, {
      encoding: 'utf8',
      timeout: VSCODE_EXTENSION.timeouts.binaryLookupMs,
      stdio: ['pipe', 'pipe', 'pipe'],
    }).trim();

    const firstPath = result.split('\n')[0];

    if (firstPath && existsSync(firstPath)) {
      return firstPath;
    }
  } catch {
    // tscanner not found in PATH
  }

  return null;
}
