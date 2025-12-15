import { existsSync } from 'node:fs';
import { join } from 'node:path';
import {
  PACKAGE_DISPLAY_NAME,
  PACKAGE_NAME,
  PLATFORM_PACKAGE_MAP,
  ensureBinaryExecutable,
  getBinaryName,
  getPlatformKey,
} from 'tscanner-common';

function tryBinaryPath(path: string): string | null {
  if (existsSync(path)) {
    ensureBinaryExecutable(path);
    return path;
  }
  return null;
}

export function getBinaryPath(): string {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGE_MAP[platformKey];
  const binaryName = getBinaryName();

  const devPath = join(__dirname, '..', 'npm', `cli-${platformKey}`, binaryName);
  const ciPath = join(__dirname, '..', 'npm', platformKey, binaryName);

  const localPath = tryBinaryPath(devPath) || tryBinaryPath(ciPath);
  if (localPath) {
    return localPath;
  }

  try {
    const binaryPath = require.resolve(`${packageName}/${binaryName}`);
    ensureBinaryExecutable(binaryPath);
    return binaryPath;
  } catch (e) {
    const error = e as Error;
    throw new Error(
      `Failed to find ${PACKAGE_DISPLAY_NAME} binary for ${platformKey}\nPlease try reinstalling: npm install ${PACKAGE_NAME}\nError: ${error.message}`,
    );
  }
}
