import { copyFileSync, existsSync, mkdirSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

const extensionRoot = resolve(__dirname, '..');
const outBinariesDir = join(extensionRoot, 'out', 'binaries');
const rustCoreDir = join(extensionRoot, '..', 'lino-core');

console.log('Copying Rust binaries to out/binaries...');

const binaryName = process.platform === 'win32' ? 'lino-server.exe' : 'lino-server';
const releaseBinary = join(rustCoreDir, 'target', 'release', binaryName);
const debugBinary = join(rustCoreDir, 'target', 'debug', binaryName);

const sourceBinary = existsSync(releaseBinary) ? releaseBinary : existsSync(debugBinary) ? debugBinary : null;

if (!sourceBinary) {
  console.warn('⚠️  Rust binary not found');
  console.log('Run: cd packages/lino-core && cargo build --release');
} else {
  if (!existsSync(outBinariesDir)) {
    mkdirSync(outBinariesDir, { recursive: true });
  }

  const platform = process.platform;
  const arch = process.arch;
  const targetMap: Record<string, string> = {
    'linux-x64': 'x86_64-unknown-linux-gnu',
    'linux-arm64': 'aarch64-unknown-linux-gnu',
    'darwin-x64': 'x86_64-apple-darwin',
    'darwin-arm64': 'aarch64-apple-darwin',
    'win32-x64': 'x86_64-pc-windows-msvc',
  };

  const target = targetMap[`${platform}-${arch}`];
  const destBinaryName = target ? `lino-server-${target}${platform === 'win32' ? '.exe' : ''}` : binaryName;
  const destBinary = join(outBinariesDir, destBinaryName);

  copyFileSync(sourceBinary, destBinary);
  console.log(`✅ Copied ${sourceBinary.includes('release') ? 'release' : 'debug'} binary to out/binaries/`);
}

const existingBinaries = existsSync(outBinariesDir)
  ? readdirSync(outBinariesDir).filter((f) => f.startsWith('lino-server'))
  : [];
if (existingBinaries.length > 0) {
  console.log(`\n📦 Binaries in out/binaries/ (${existingBinaries.length}):`);
  existingBinaries.forEach((f) => console.log(`   - ${f}`));
}

console.log('\n✅ Copy binaries complete!');
