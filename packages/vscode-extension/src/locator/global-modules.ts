import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { PLATFORM_PACKAGE_MAP, getBinaryName, getPlatformKey } from 'tscanner-common';

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

    const tscannerPath = join(globalPath, 'tscanner');
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
    const npmRoot = execSync('npm root -g', { encoding: 'utf8', timeout: 5000 }).trim();
    if (npmRoot && existsSync(npmRoot)) {
      paths.push(npmRoot);
    }
  } catch {
    // npm global not found
  }

  try {
    const pnpmRoot = execSync('pnpm root -g', { encoding: 'utf8', timeout: 5000 }).trim();
    if (pnpmRoot && existsSync(pnpmRoot)) {
      paths.push(pnpmRoot);
    }
  } catch {
    // pnpm global not found
  }

  const bunPath = join(process.env.HOME ?? '', '.bun', 'install', 'global', 'node_modules');
  if (existsSync(bunPath)) {
    paths.push(bunPath);
  }

  return paths;
}
