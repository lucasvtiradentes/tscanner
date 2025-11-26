import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { PLATFORM_PACKAGE_MAP, getBinaryName, getPlatformKey } from 'tscanner-common';

export function getBinaryPath(): string {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGE_MAP[platformKey];
  const binaryName = getBinaryName();

  const devPath = join(__dirname, '..', 'npm', `cli-${platformKey}`, binaryName);
  if (existsSync(devPath)) {
    return devPath;
  }

  const ciPath = join(__dirname, '..', 'npm', platformKey, binaryName);
  if (existsSync(ciPath)) {
    return ciPath;
  }

  try {
    const binaryPath = require.resolve(`${packageName}/${binaryName}`);
    return binaryPath;
  } catch (e) {
    const error = e as Error;
    throw new Error(
      `Failed to find TScanner binary for ${platformKey}\nPlease try reinstalling: npm install tscanner\nError: ${error.message}`,
    );
  }
}
