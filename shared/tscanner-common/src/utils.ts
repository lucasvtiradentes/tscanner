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

export function getPlatformKey(): string {
  const platform = process.platform;
  const arch = process.arch;

  const SUPPORTED_PLATFORMS: Record<string, Record<string, string>> = {
    linux: { x64: 'linux-x64', arm64: 'linux-arm64' },
    darwin: { x64: 'darwin-x64', arm64: 'darwin-arm64' },
    win32: { x64: 'win32-x64' },
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
