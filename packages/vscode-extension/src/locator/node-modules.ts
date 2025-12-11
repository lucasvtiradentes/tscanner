import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { PACKAGE_NAME, PLATFORM_PACKAGE_MAP, getBinaryName, getPlatformKey } from 'tscanner-common';

export async function findInNodeModules(workspaceRoot: string): Promise<string | null> {
  try {
    const platformKey = getPlatformKey();
    const packageName = PLATFORM_PACKAGE_MAP[platformKey];
    const binaryName = getBinaryName();

    const require = createRequire(join(workspaceRoot, 'package.json'));

    try {
      const packageJsonPath = require.resolve(`${packageName}/package.json`);
      const packageDir = dirname(packageJsonPath);
      const binaryPath = join(packageDir, binaryName);

      if (existsSync(binaryPath)) {
        return binaryPath;
      }
    } catch {
      // Package not found directly
    }

    try {
      const tscannerPath = require.resolve(`${PACKAGE_NAME}/package.json`);
      const tscannerDir = dirname(tscannerPath);
      const tscannerRequire = createRequire(join(tscannerDir, 'package.json'));
      const platformPackagePath = tscannerRequire.resolve(`${packageName}/package.json`);
      const platformPackageDir = dirname(platformPackagePath);
      const binaryPath = join(platformPackageDir, binaryName);

      if (existsSync(binaryPath)) {
        return binaryPath;
      }
    } catch {
      // tscanner package not found
    }
  } catch {
    // Error searching node_modules
  }

  return null;
}
