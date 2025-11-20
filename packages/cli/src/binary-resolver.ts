import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { PLATFORM_MAP, getBinaryName, getPlatformKey } from './platform';

const __dirname = dirname(fileURLToPath(import.meta.url));

export function getBinaryPath(): string {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_MAP[platformKey];
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
    const packagePath = join(__dirname, '..', 'node_modules', packageName, binaryName);
    if (existsSync(packagePath)) {
      return packagePath;
    }
    throw new Error('Binary not found in node_modules');
  } catch (e) {
    const error = e as Error;
    throw new Error(
      `Failed to find tscanner binary for ${platformKey}\n` +
        `Please try reinstalling: npm install tscanner\n` +
        `Error: ${error.message}`,
    );
  }
}
