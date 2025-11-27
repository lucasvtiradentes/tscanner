import { chmodSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const SCRIPT_DIR = __dirname;
const ROOT_DIR = join(SCRIPT_DIR, '..', '..');
const CLI_DIR = join(ROOT_DIR, 'packages', 'cli');
const CLI_MANIFEST_PATH = join(CLI_DIR, 'package.json');

const logger = console;

const PLATFORMS = [
  { platform: 'win32', arch: 'x64' },
  { platform: 'darwin', arch: 'x64' },
  { platform: 'darwin', arch: 'arm64' },
  { platform: 'linux', arch: 'x64' },
  { platform: 'linux', arch: 'arm64' },
];

async function main() {
  if (!process.env.CI && !process.env.GITHUB_ACTIONS) {
    logger.log('This script should only run in CI/CD environment');
    process.exit(1);
  }

  const cliManifest = JSON.parse(readFileSync(CLI_MANIFEST_PATH, 'utf-8'));

  await generateNativePackages(cliManifest);
  await updateCliPackageVersion(cliManifest);
  await printSuccessMessage();
}

main();

async function generateNativePackages(cliManifest: any) {
  logger.log(`Step 1/2 - Generating native packages for ${PLATFORMS.length} platforms...`);

  for (const { platform, arch } of PLATFORMS) {
    const os = platform;
    const buildName = `cli-${platform}-${arch}`;
    const packageRoot = join(CLI_DIR, 'npm', buildName);
    const packageName = `@tscanner/${buildName}`;

    const { version, license, repository } = cliManifest;

    const binaryName = os === 'win32' ? 'tscanner.exe' : 'tscanner';

    const manifest = JSON.stringify(
      {
        name: packageName,
        version,
        license,
        repository: repository.url,
        main: binaryName,
        os: [os],
        cpu: [arch],
        files: [binaryName],
      },
      null,
      2,
    );

    if (!existsSync(packageRoot)) {
      mkdirSync(packageRoot, { recursive: true });
    }

    const manifestPath = join(packageRoot, 'package.json');
    writeFileSync(manifestPath, manifest);

    const ext = os === 'win32' ? '.exe' : '';
    const binaryPath = join(packageRoot, `tscanner${ext}`);

    if (existsSync(binaryPath)) {
      chmodSync(binaryPath, 0o755);
      logger.log(`   ✅ Generated package for ${platform}-${arch}`);
    } else {
      logger.log(`   ⚠️  Binary not found for ${platform}-${arch} - ensure workflow copied it`);
    }
  }
}

async function updateCliPackageVersion(cliManifest: any) {
  logger.log('Step 2/2 - Updating CLI package version...');

  const manifest = JSON.parse(readFileSync(CLI_MANIFEST_PATH, 'utf-8'));
  const { version } = cliManifest;

  if (manifest.optionalDependencies) {
    for (const dependency of Object.keys(manifest.optionalDependencies)) {
      if (dependency.startsWith('@tscanner/')) {
        manifest.optionalDependencies[dependency] = version;
      }
    }
  }

  writeFileSync(CLI_MANIFEST_PATH, `${JSON.stringify(manifest, null, 2)}\n`);
  logger.log('   ✅ Updated CLI package version');
}

async function printSuccessMessage() {
  logger.log('\n✅ All packages generated successfully!');
  logger.log('   Ready for publishing\n');
}
