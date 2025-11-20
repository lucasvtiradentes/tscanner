import { chmodSync, copyFileSync, existsSync, mkdirSync } from 'node:fs';
import { arch, platform } from 'node:os';
import { join } from 'node:path';

const SCRIPT_DIR = __dirname;
const CLI_DIR = join(SCRIPT_DIR, '..');
const TSCANNER_CORE_DIR = join(CLI_DIR, '..', 'core');

async function main() {
  if (process.env.CI || process.env.GITHUB_ACTIONS) {
    console.log('Skipping local installation in CI environment');
    process.exit(0);
  }

  await copyBinary();
  await printSuccessMessage();
}

main();

async function copyBinary() {
  console.log('Step 1/1 - Copying Rust binary for current platform...');

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
    console.log(`   ⚠️  Unsupported platform: ${OS}-${ARCH} - skipping`);
    return;
  }

  let SOURCE_PATH = join(TSCANNER_CORE_DIR, 'target', 'release', 'tscanner');
  if (NPM_PLATFORM.startsWith('win32')) {
    SOURCE_PATH += '.exe';
  }

  const DEST_DIR = join(CLI_DIR, 'npm', `cli-${NPM_PLATFORM}`);
  let DEST_PATH = join(DEST_DIR, 'tscanner');
  if (NPM_PLATFORM.startsWith('win32')) {
    DEST_PATH += '.exe';
  }

  if (!existsSync(SOURCE_PATH)) {
    console.log('   ⚠️  Binary not found - skipping (not built yet)');
    return;
  }

  mkdirSync(DEST_DIR, { recursive: true });
  copyFileSync(SOURCE_PATH, DEST_PATH);

  try {
    chmodSync(DEST_PATH, 0o755);
  } catch {}

  console.log(`   ✅ Copied binary for ${NPM_PLATFORM}`);
}

async function printSuccessMessage() {
  console.log('\n✅ Build complete!');
  console.log('   Binary is ready to use\n');
}
