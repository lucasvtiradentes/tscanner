import { chmodSync, copyFileSync, existsSync, mkdirSync, unlinkSync } from 'node:fs';
import { arch, platform } from 'node:os';
import { join } from 'node:path';

const SCRIPT_DIR = __dirname;
const ROOT_DIR = join(SCRIPT_DIR, '..', '..');
const CLI_DIR = join(ROOT_DIR, 'packages', 'cli');
const CORE_DIR = join(ROOT_DIR, 'packages', 'rust-core');

const logger = console;

function main() {
  if (process.env.CI || process.env.GITHUB_ACTIONS) {
    logger.log('Skipping local CLI installation in CI environment');
    process.exit(0);
  }

  const success = copyBinary();
  if (success) {
    printSuccessMessage();
  }
}

main();

function copyBinary(): boolean {
  logger.log('[CLI] Step 1/1 - Copying Rust binary for current platform...');

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
    return false;
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
    return false;
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
          return false;
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
        return false;
      }
    }
    throw error;
  }

  try {
    chmodSync(DEST_PATH, 0o755);
  } catch {}

  logger.log(`[CLI]    ✅ Copied binary for ${NPM_PLATFORM}`);
  return true;
}

function printSuccessMessage() {
  logger.log('[CLI] ✅ Build complete!');
  logger.log('[CLI]    Binary is ready to use\n');
}
