import { execSync } from 'node:child_process';
import { VSCODE_EXTENSION } from 'tscanner-common';
import { Locator, LocatorSource } from './locator';

export type BinaryInfo = {
  source: LocatorSource;
  version: string | null;
};

export function getBinaryVersion(binaryPath: string): string | null {
  try {
    const output = execSync(`"${binaryPath}" --version`, {
      encoding: 'utf8',
      timeout: VSCODE_EXTENSION.timeouts.versionCheckSeconds * 1000,
    });
    const match = output.match(/(\d+\.\d+\.\d+)/);
    return match ? match[1] : null;
  } catch {
    return null;
  }
}

export async function loadBinaryInfo(workspacePath: string): Promise<BinaryInfo> {
  const locator = new Locator(workspacePath);
  const result = await locator.locate();

  if (!result) {
    return { source: LocatorSource.Global, version: null };
  }

  const version = getBinaryVersion(result.path);
  return { source: result.source, version };
}
