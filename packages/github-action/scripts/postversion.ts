import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const isCI = process.env.CI === 'true' || process.env.GITHUB_ACTIONS === 'true';

function log(message: string) {
  console.log(`[postversion] ${message}`);
}

function updateReadmeVersion() {
  const packageJsonPath = join(__dirname, '..', 'package.json');
  const readmePath = join(__dirname, '..', 'README.md');

  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
  const newVersion = packageJson.version;

  log(`Detected new version: ${newVersion}`);

  let readme = readFileSync(readmePath, 'utf-8');

  const versionPattern = /@v\d+\.\d+\.\d+/g;
  const newVersionTag = `@v${newVersion}`;

  const matches = readme.match(versionPattern);
  if (matches) {
    log(`Found ${matches.length} version references to update`);
    readme = readme.replace(versionPattern, newVersionTag);
    writeFileSync(readmePath, readme, 'utf-8');
    log(`Updated README.md to version ${newVersionTag}`);
  } else {
    log('No version references found in README.md');
  }
}

function main() {
  if (!isCI) {
    log('Not running in CI, skipping README update');
    return;
  }

  log('Running in CI, updating README version...');
  updateReadmeVersion();
  log('Postversion script completed');
}

main();
