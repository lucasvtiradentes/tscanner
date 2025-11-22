export const PLATFORM_MAP: Record<string, string> = {
  'linux-x64': '@tscanner/cli-linux-x64',
  'linux-arm64': '@tscanner/cli-linux-arm64',
  'darwin-x64': '@tscanner/cli-darwin-x64',
  'darwin-arm64': '@tscanner/cli-darwin-arm64',
  'win32-x64': '@tscanner/cli-win32-x64',
};

export function getPlatformKey(): string {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'linux') {
    if (arch === 'x64') return 'linux-x64';
    if (arch === 'arm64') return 'linux-arm64';
  }

  if (platform === 'darwin') {
    if (arch === 'x64') return 'darwin-x64';
    if (arch === 'arm64') return 'darwin-arm64';
  }

  if (platform === 'win32') {
    if (arch === 'x64') return 'win32-x64';
  }

  throw new Error(
    `Unsupported platform: ${platform}-${arch}\ntscanner is only available for:\n  - Linux (x64, arm64)\n  - macOS (x64, arm64)\n  - Windows (x64)`,
  );
}

export function getBinaryName(): string {
  return `tscanner${process.platform === 'win32' ? '.exe' : ''}`;
}
