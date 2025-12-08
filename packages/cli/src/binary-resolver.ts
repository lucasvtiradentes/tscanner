import { existsSync } from 'node:fs';
import { join } from 'node:path';
import {
  PACKAGE_DISPLAY_NAME,
  PACKAGE_NAME,
  PLATFORM_PACKAGE_MAP,
  getBinaryName,
  getPlatformKey,
} from 'tscanner-common';

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
      `Failed to find ${PACKAGE_DISPLAY_NAME} binary for ${platformKey}\nPlease try reinstalling: npm install ${PACKAGE_NAME}\nError: ${error.message}`,
    );
  }
}
