import { existsSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { BINARY_BASE_NAME } from '../src/common/constants';

const extensionRoot = resolve(__dirname, '..');
const outBinariesDir = join(extensionRoot, 'out', 'binaries');

console.log('📦 Checking binaries in out/binaries...');

if (!existsSync(outBinariesDir)) {
  console.warn('⚠️  No binaries found in out/binaries/');
  console.warn('Binaries should be moved directly to out/binaries/ by CI workflow');
  console.warn('For local development, run: pnpm build:binaries');
  process.exit(0);
}

const existingBinaries = readdirSync(outBinariesDir).filter((f) => f.startsWith(BINARY_BASE_NAME));

if (existingBinaries.length === 0) {
  console.warn('⚠️  No binaries found in out/binaries/');
  console.warn('For local development, run: pnpm build:binaries');
  process.exit(0);
}

console.log(`✅ Found ${existingBinaries.length} binaries in out/binaries/`);
existingBinaries.forEach((f) => console.log(`   - ${f}`));
console.log('\n✅ Binaries check complete!');
