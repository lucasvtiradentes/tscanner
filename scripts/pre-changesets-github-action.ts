import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

function log(message: string) {
  console.log(`[pre-changesets] ${message}`);
}

function getChangesetFiles(): string[] {
  const changesetDir = join(process.cwd(), '.changeset');
  return readdirSync(changesetDir).filter((file) => file.endsWith('.md') && file !== 'README.md');
}

function hasGithubActionChangeset(): boolean {
  const changesets = getChangesetFiles();

  for (const file of changesets) {
    const content = readFileSync(join('.changeset', file), 'utf-8');
    if (content.includes('tscanner-github-action')) {
      log(`Found changeset for github-action: ${file}`);
      return true;
    }
  }

  return false;
}

function getBumpType(): 'major' | 'minor' | 'patch' {
  const changesets = getChangesetFiles();
  let bumpType: 'major' | 'minor' | 'patch' = 'patch';

  for (const file of changesets) {
    const content = readFileSync(join('.changeset', file), 'utf-8');
    if (content.includes('tscanner-github-action')) {
      if (content.includes('major')) {
        bumpType = 'major';
      } else if (content.includes('minor') && bumpType !== 'major') {
        bumpType = 'minor';
      }
    }
  }

  return bumpType;
}

function calculateNewVersion(currentVersion: string, bumpType: 'major' | 'minor' | 'patch'): string {
  const [major, minor, patch] = currentVersion.split('.').map(Number);

  switch (bumpType) {
    case 'major':
      return `${major + 1}.0.0`;
    case 'minor':
      return `${major}.${minor + 1}.0`;
    case 'patch':
      return `${major}.${minor}.${patch + 1}`;
  }
}

function updateReadmeVersion(newVersion: string) {
  const readmePath = join('packages', 'github-action', 'README.md');
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
  log('Checking for github-action changesets...');

  if (!hasGithubActionChangeset()) {
    log('No github-action changeset found, skipping README update');
    return;
  }

  const packageJsonPath = join('packages', 'github-action', 'package.json');
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
  const currentVersion = packageJson.version;

  log(`Current version: ${currentVersion}`);

  const bumpType = getBumpType();
  log(`Bump type: ${bumpType}`);

  const newVersion = calculateNewVersion(currentVersion, bumpType);
  log(`Calculated new version: ${newVersion}`);

  updateReadmeVersion(newVersion);
  log('Pre-changesets script completed successfully');
}

main();
