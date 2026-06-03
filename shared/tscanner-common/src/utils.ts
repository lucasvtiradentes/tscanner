import { constants, accessSync, chmodSync } from 'node:fs';
import { PlatformKey } from './types';

export function slugify(text: string): string {
  return text
    .toLowerCase()
    .trim()
    .replace(/\s+/g, '-')
    .replace(/[^\w-]+/g, '');
}

export function pluralize(count: number, singular: string): string {
  return count === 1 ? singular : `${singular}s`;
}

export function getPlatformKey(): PlatformKey {
  const platform = process.platform;
  const arch = process.arch;

  const SUPPORTED_PLATFORMS: Record<string, Partial<Record<string, PlatformKey>>> = {
    linux: { x64: PlatformKey.LinuxX64, arm64: PlatformKey.LinuxArm64 },
    darwin: { x64: PlatformKey.DarwinX64, arm64: PlatformKey.DarwinArm64 },
    win32: { x64: PlatformKey.Win32X64 },
  };
  const key = SUPPORTED_PLATFORMS[platform]?.[arch];

  if (!key) {
    throw new Error(
      `Unsupported platform: ${platform}-${arch}\ntscanner is only available for:\n  - Linux (x64, arm64)\n  - macOS (x64, arm64)\n  - Windows (x64)`,
    );
  }

  return key;
}

export function getBinaryName(baseName = 'tscanner'): string {
  return `${baseName}${process.platform === 'win32' ? '.exe' : ''}`;
}

export function formatDuration(ms: number): string {
  if (ms < 1000) {
    return `${ms}ms`;
  }
  if (ms < 60000) {
    return `${(ms / 1000).toFixed(1)}s`;
  }
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}m ${seconds}s`;
}

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
