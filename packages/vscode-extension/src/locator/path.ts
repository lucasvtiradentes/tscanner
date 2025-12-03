import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { getBinaryName } from 'tscanner-common';

export async function findInPath(): Promise<string | null> {
  const binaryName = getBinaryName();
  const command = process.platform === 'win32' ? 'where' : 'which';

  try {
    const result = execSync(`${command} ${binaryName}`, {
      encoding: 'utf8',
      timeout: 5000,
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
