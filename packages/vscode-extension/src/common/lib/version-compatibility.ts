export type VersionParts = {
  major: number;
  minor: number;
  patch: number;
};

export type VersionCompatibility = 'compatible' | 'extension-newer' | 'binary-newer' | 'unknown';

export function parseVersion(version: string | null | undefined): VersionParts | null {
  if (!version) return null;

  const match = version.match(/^(\d+)\.(\d+)\.(\d+)/);
  if (!match) return null;

  return {
    major: Number.parseInt(match[1], 10),
    minor: Number.parseInt(match[2], 10),
    patch: Number.parseInt(match[3], 10),
  };
}

export function compareVersions(v1: string | null | undefined, v2: string | null | undefined): VersionCompatibility {
  const parts1 = parseVersion(v1);
  const parts2 = parseVersion(v2);

  if (!parts1 || !parts2) return 'unknown';

  if (parts1.major !== parts2.major) {
    return parts1.major > parts2.major ? 'extension-newer' : 'binary-newer';
  }

  return 'compatible';
}

export function shouldShowVersionWarning(
  extensionVersion: string | null | undefined,
  binaryVersion: string | null | undefined,
): boolean {
  const compatibility = compareVersions(extensionVersion, binaryVersion);
  return compatibility === 'extension-newer' || compatibility === 'binary-newer';
}

export function getVersionWarningMessage(
  extensionVersion: string | null | undefined,
  binaryVersion: string | null | undefined,
): string | null {
  if (!shouldShowVersionWarning(extensionVersion, binaryVersion)) {
    return null;
  }

  const compatibility = compareVersions(extensionVersion, binaryVersion);

  if (compatibility === 'extension-newer') {
    return `Extension (v${extensionVersion}) is newer than binary (v${binaryVersion}). Update CLI recommended.`;
  }

  if (compatibility === 'binary-newer') {
    return `Binary (v${binaryVersion}) is newer than extension (v${extensionVersion}). Update extension recommended.`;
  }

  return null;
}
