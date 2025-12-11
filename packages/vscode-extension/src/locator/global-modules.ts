import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import { homedir } from 'node:os';
import { dirname, join } from 'node:path';
import { PACKAGE_NAME, PLATFORM_PACKAGE_MAP, VSCODE_EXTENSION, getBinaryName, getPlatformKey } from 'tscanner-common';

export async function findInGlobalModules(): Promise<string | null> {
  const globalPaths = await getGlobalNodeModulesPaths();
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGE_MAP[platformKey];
  const binaryName = getBinaryName();

  for (const globalPath of globalPaths) {
    const directPath = join(globalPath, packageName, binaryName);
    if (existsSync(directPath)) {
      return directPath;
    }

    const tscannerPath = join(globalPath, PACKAGE_NAME);
    if (existsSync(tscannerPath)) {
      try {
        const require = createRequire(join(tscannerPath, 'package.json'));
        const platformPackagePath = require.resolve(`${packageName}/package.json`);
        const binaryPath = join(dirname(platformPackagePath), binaryName);

        if (existsSync(binaryPath)) {
          return binaryPath;
        }
      } catch {
        // Could not resolve platform package
      }
    }
  }

  return null;
}

async function getGlobalNodeModulesPaths(): Promise<string[]> {
  const paths: string[] = [];

  try {
    const npmRoot = execSync('npm root -g', {
      encoding: 'utf8',
      timeout: VSCODE_EXTENSION.timeouts.binaryLookupMs,
    }).trim();
    if (npmRoot && existsSync(npmRoot)) {
      paths.push(npmRoot);
    }
  } catch {
    // npm global not found
  }

  try {
    const pnpmRoot = execSync('pnpm root -g', {
      encoding: 'utf8',
      timeout: VSCODE_EXTENSION.timeouts.binaryLookupMs,
    }).trim();
    if (pnpmRoot && existsSync(pnpmRoot)) {
      paths.push(pnpmRoot);
    }
  } catch {
    // pnpm global not found
  }

  const bunPath = join(homedir(), VSCODE_EXTENSION.paths.bunGlobalModules);
  if (existsSync(bunPath)) {
    paths.push(bunPath);
  }

  return paths;
}
