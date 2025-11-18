import { copyFileSync, existsSync, mkdirSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { BINARY_BASE_NAME } from '../src/common/constants';

const extensionRoot = resolve(__dirname, '..');
const sourceBinariesDir = join(extensionRoot, 'binaries');
const outBinariesDir = join(extensionRoot, 'out', 'binaries');

console.log('📦 Copying binaries to out/binaries...');

if (!existsSync(sourceBinariesDir)) {
  console.warn('⚠️  No binaries folder found at:', sourceBinariesDir);
  console.log('Run: pnpm build:binaries');
  process.exit(0);
}

const sourceBinaries = readdirSync(sourceBinariesDir).filter((f) => f.startsWith(BINARY_BASE_NAME));

if (sourceBinaries.length === 0) {
  console.warn('⚠️  No binaries found in:', sourceBinariesDir);
  console.log('Run: pnpm build:binaries');
  process.exit(0);
}

mkdirSync(outBinariesDir, { recursive: true });

let copiedCount = 0;
for (const binary of sourceBinaries) {
  const source = join(sourceBinariesDir, binary);
  const dest = join(outBinariesDir, binary);
  copyFileSync(source, dest);
  copiedCount++;
}

console.log(`✅ Copied ${copiedCount} binaries to out/binaries/`);

const existingBinaries = readdirSync(outBinariesDir).filter((f) => f.startsWith(BINARY_BASE_NAME));
console.log(`\n📦 Binaries in out/binaries/ (${existingBinaries.length}):`);
existingBinaries.forEach((f) => console.log(`   - ${f}`));

console.log('\n✅ Copy binaries complete!');
