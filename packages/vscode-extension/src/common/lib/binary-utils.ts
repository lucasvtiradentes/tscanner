import { constants, accessSync, chmodSync } from 'node:fs';

/**
 * Ensures a binary file has execute permission on Unix systems.
 *
 * This is necessary because npm/pnpm don't preserve file permissions when publishing packages.
 * When binaries are installed via package managers (especially pnpm with its content-addressable
 * store and hardlinks), the execute bit is often lost even if it was set before publishing.
 *
 * @see https://github.com/npm/npm/issues/1104
 * @see https://github.com/pnpm/pnpm/issues/3140
 */
export function ensureBinaryExecutable(binaryPath: string): void {
  if (process.platform === 'win32') {
    return;
  }

  try {
    accessSync(binaryPath, constants.X_OK);
  } catch {
    chmodSync(binaryPath, 0o755);
  }
}
