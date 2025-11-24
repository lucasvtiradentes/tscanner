const { readFileSync, writeFileSync } = require('node:fs');
const { join } = require('node:path');
const { execSync } = require('node:child_process');

function log(message) {
  console.log(`[changeset-commit] ${message}`);
}

function updateGithubActionReadme() {
  const packageJsonPath = join(process.cwd(), 'packages', 'github-action', 'package.json');
  const readmePath = join(process.cwd(), 'packages', 'github-action', 'README.md');

  try {
    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
    const newVersion = packageJson.version;

    log(`Updating README to version ${newVersion}`);

    let readme = readFileSync(readmePath, 'utf-8');
    const versionPattern = /@v\d+\.\d+\.\d+/g;
    const newVersionTag = `@v${newVersion}`;

    const matches = readme.match(versionPattern);
    if (matches) {
      log(`Found ${matches.length} version references to update`);
      readme = readme.replace(versionPattern, newVersionTag);
      writeFileSync(readmePath, readme, 'utf-8');

      execSync('git add packages/github-action/README.md', { stdio: 'inherit' });
      log('Added README.md to git staging');
    } else {
      log('No version references found in README.md');
    }
  } catch (error) {
    log(`Error updating README: ${error.message}`);
  }
}

async function getVersionMessage() {
  log('Running getVersionMessage hook');

  updateGithubActionReadme();

  return 'Version Packages';
}

module.exports = {
  getVersionMessage,
};
