import { execFileSync } from 'node:child_process';
import { chmodSync, copyFileSync, existsSync, mkdirSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { arch, homedir, platform } from 'node:os';
import { join } from 'node:path';
import { PACKAGE_DEV_NAME } from 'tscanner-common';
import { scriptEnv } from '../env';

const SCRIPT_DIR = __dirname;
const ROOT_DIR = join(SCRIPT_DIR, '..', '..');
const CLI_DIR = join(ROOT_DIR, 'packages', 'cli');
const CORE_DIR = join(ROOT_DIR, 'packages', 'rust-core');
const PROGRAM_NAME_ENV = 'TSCANNER_PROG_NAME';

const logger = console;

function main() {
  if (scriptEnv.isCi) {
    logger.log('Skipping local CLI installation in CI environment');
    process.exit(0);
  }

  const binaryPath = copyBinary();
  if (binaryPath) {
    installDevBinary(binaryPath);
    printSuccessMessage();
  }
}

main();

function copyBinary(): string | null {
  logger.log('[CLI] Step 1/2 - Copying Rust binary for current platform...');

  const OS = platform();
  const ARCH = arch();

  let NPM_PLATFORM: string | undefined;

  if (OS === 'linux') {
    if (ARCH === 'x64') {
      NPM_PLATFORM = 'linux-x64';
    } else if (ARCH === 'arm64') {
      NPM_PLATFORM = 'linux-arm64';
    }
  } else if (OS === 'darwin') {
    if (ARCH === 'x64') {
      NPM_PLATFORM = 'darwin-x64';
    } else if (ARCH === 'arm64') {
      NPM_PLATFORM = 'darwin-arm64';
    }
  } else if (OS === 'win32') {
    NPM_PLATFORM = 'win32-x64';
  }

  if (!NPM_PLATFORM) {
    logger.log(`[CLI]    ⚠️  Unsupported platform: ${OS}-${ARCH} - skipping`);
    return null;
  }

  let SOURCE_PATH = join(CORE_DIR, 'target', 'release', 'tscanner');
  if (NPM_PLATFORM.startsWith('win32')) {
    SOURCE_PATH += '.exe';
  }

  const DEST_DIR = join(CLI_DIR, 'npm', `cli-${NPM_PLATFORM}`);
  let DEST_PATH = join(DEST_DIR, 'tscanner');
  if (NPM_PLATFORM.startsWith('win32')) {
    DEST_PATH += '.exe';
  }

  if (!existsSync(SOURCE_PATH)) {
    logger.log('[CLI]    ⚠️  Binary not found - skipping (not built yet)');
    return null;
  }

  mkdirSync(DEST_DIR, { recursive: true });

  // Try to delete destination file if it exists (it might be in use)
  if (existsSync(DEST_PATH)) {
    try {
      unlinkSync(DEST_PATH);
    } catch (error: unknown) {
      if (error instanceof Error && 'code' in error) {
        if (error.code === 'ETXTBSY' || error.code === 'EBUSY') {
          logger.log('[CLI]    ⚠️  Binary is in use - skipping copy (file is locked by another process)');
          logger.log('[CLI]    💡 Tip: Close any running tscanner processes and try again');
          return null;
        }
      }
    }
  }

  try {
    copyFileSync(SOURCE_PATH, DEST_PATH);
  } catch (error: unknown) {
    if (error instanceof Error && 'code' in error) {
      if (error.code === 'ETXTBSY' || error.code === 'EBUSY') {
        logger.log('[CLI]    ⚠️  Binary is in use - skipping copy (file is locked by another process)');
        logger.log('[CLI]    💡 Tip: Close any running tscanner processes and try again');
        return null;
      }
    }
    throw error;
  }

  try {
    chmodSync(DEST_PATH, 0o755);
  } catch {}

  logger.log(`[CLI]    ✅ Copied binary for ${NPM_PLATFORM}`);
  return DEST_PATH;
}

function installDevBinary(binaryPath: string) {
  logger.log(`[CLI] Step 2/2 - Installing local dev command: ${PACKAGE_DEV_NAME}`);

  const binDir = getDevBinDir();
  mkdirSync(binDir, { recursive: true });

  if (platform() === 'win32') {
    const targetPath = join(binDir, `${PACKAGE_DEV_NAME}.cmd`);
    rmSync(targetPath, { force: true });
    writeFileSync(targetPath, getWindowsShim(binaryPath));
  } else {
    const targetPath = join(binDir, PACKAGE_DEV_NAME);
    rmSync(targetPath, { force: true });
    writeFileSync(targetPath, getPosixShim(binaryPath));
    chmodSync(targetPath, 0o755);
  }

  logger.log(`[CLI]    ✅ Installed ${PACKAGE_DEV_NAME} to ${binDir}`);
}

function printSuccessMessage() {
  logger.log('[CLI] ✅ Build complete!');
  logger.log('[CLI]    Binary is ready to use\n');
}

function getDevBinDir(): string {
  if (scriptEnv.devBinDir) return scriptEnv.devBinDir;
  if (platform() === 'win32') return getNpmPrefix() ?? join(homedir(), 'AppData', 'Roaming', 'npm');
  return join(homedir(), '.local', 'bin');
}

function getNpmPrefix(): string | null {
  try {
    return execFileSync(getNpmExecutable(), ['config', 'get', 'prefix'], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    return null;
  }
}

function getNpmExecutable(): string {
  return platform() === 'win32' ? 'npm.cmd' : 'npm';
}

function getPosixShim(binaryPath: string): string {
  return `#!/usr/bin/env sh
set -eu
export ${PROGRAM_NAME_ENV}=${shellQuote(PACKAGE_DEV_NAME)}
exec ${shellQuote(binaryPath)} "$@"
`;
}

function getWindowsShim(binaryPath: string): string {
  return `@echo off
setlocal
set "${PROGRAM_NAME_ENV}=${PACKAGE_DEV_NAME}"
"${binaryPath}" %*
`;
}

function shellQuote(value: string): string {
  return `'${value.replaceAll("'", "'\\''")}'`;
}
