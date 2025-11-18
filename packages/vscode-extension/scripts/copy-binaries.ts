import { copyFileSync, existsSync, mkdirSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { BINARY_BASE_NAME, PLATFORM_TARGET_MAP, getBinaryName } from '../src/common/constants';

const extensionRoot = resolve(__dirname, '..');
const outBinariesDir = join(extensionRoot, 'out', 'binaries');
const coreTargetDir = join(extensionRoot, '..', 'core', 'target', 'release');

function getPlatformTarget(): string | null {
  const platform = `${process.platform}-${process.arch}`;
  return PLATFORM_TARGET_MAP[platform] || null;
}

function copyLocalBinary(): boolean {
  const target = getPlatformTarget();
  if (!target) {
    console.warn(`⚠️  Unsupported platform: ${process.platform}-${process.arch}`);
    return false;
  }

  const sourceBinary = join(coreTargetDir, getBinaryName());
  if (!existsSync(sourceBinary)) {
    console.warn(`⚠️  Binary not found: ${sourceBinary}`);
    return false;
  }

  mkdirSync(outBinariesDir, { recursive: true });

  const targetBinary = join(
    outBinariesDir,
    `${BINARY_BASE_NAME}-${target}${process.platform === 'win32' ? '.exe' : ''}`,
  );
  copyFileSync(sourceBinary, targetBinary);

  console.log(`✅ Copied binary for ${target}`);
  console.log(`   Source: ${sourceBinary}`);
  console.log(`   Target: ${targetBinary}`);

  return true;
}

console.log('📦 Setting up binaries...');

if (process.env.CI || process.env.GITHUB_ACTIONS) {
  console.log('🔧 CI environment detected - checking pre-copied binaries...');

  if (!existsSync(outBinariesDir)) {
    console.warn('⚠️  No binaries found in out/binaries/');
    console.warn('Binaries will be organized later in release workflow');
    console.log('✅ Skipping binary check in CI build phase');
    process.exit(0);
  }

  const existingBinaries = readdirSync(outBinariesDir).filter((f) => f.startsWith(BINARY_BASE_NAME));

  if (existingBinaries.length === 0) {
    console.warn('⚠️  No binaries found in out/binaries/');
    console.warn('Binaries will be organized later in release workflow');
    console.log('✅ Skipping binary check in CI build phase');
    process.exit(0);
  }

  console.log(`✅ Found ${existingBinaries.length} binaries in out/binaries/`);
  existingBinaries.forEach((f) => console.log(`   - ${f}`));
} else {
  console.log('🔧 Local development - copying current platform binary...');

  if (!copyLocalBinary()) {
    console.warn('⚠️  Failed to copy local binary');
    console.warn('Make sure packages/core is built: pnpm build --filter=core');
    process.exit(0);
  }
}

console.log('\n✅ Binaries setup complete!');
