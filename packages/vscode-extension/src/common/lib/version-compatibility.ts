export type VersionParts = {
  major: number;
  minor: number;
  patch: number;
};

export enum VersionCompatibility {
  Compatible = 'compatible',
  ExtensionNewer = 'extension-newer',
  BinaryNewer = 'binary-newer',
  Unknown = 'unknown',
}

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

  if (!parts1 || !parts2) return VersionCompatibility.Unknown;

  const num1 = parts1.major * 1000000 + parts1.minor * 1000 + parts1.patch;
  const num2 = parts2.major * 1000000 + parts2.minor * 1000 + parts2.patch;

  if (num1 > num2) return VersionCompatibility.ExtensionNewer;
  if (num1 < num2) return VersionCompatibility.BinaryNewer;

  return VersionCompatibility.Compatible;
}

export function shouldShowVersionWarning(
  extensionVersion: string | null | undefined,
  binaryVersion: string | null | undefined,
): boolean {
  const compatibility = compareVersions(extensionVersion, binaryVersion);
  return compatibility === VersionCompatibility.ExtensionNewer || compatibility === VersionCompatibility.BinaryNewer;
}

export function getVersionWarningMessage(
  extensionVersion: string | null | undefined,
  binaryVersion: string | null | undefined,
): string | null {
  if (!shouldShowVersionWarning(extensionVersion, binaryVersion)) {
    return null;
  }

  const compatibility = compareVersions(extensionVersion, binaryVersion);

  if (compatibility === VersionCompatibility.ExtensionNewer) {
    return `Extension (v${extensionVersion}) is newer than binary (v${binaryVersion}). Update CLI recommended.`;
  }

  if (compatibility === VersionCompatibility.BinaryNewer) {
    return `Binary (v${binaryVersion}) is newer than extension (v${extensionVersion}). Update extension recommended.`;
  }

  return null;
}
