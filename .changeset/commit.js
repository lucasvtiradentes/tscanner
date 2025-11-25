const { readFileSync, writeFileSync } = require('node:fs');
const { join } = require('node:path');
const { execSync } = require('node:child_process');

function log(message) {
  console.log(`[changeset-commit] ${message}`);
}

function updateReadmeVersions() {
  const packageJsonPath = join(process.cwd(), 'packages', 'github-action', 'package.json');
  const githubActionReadmePath = join(process.cwd(), 'packages', 'github-action', 'README.md');
  const rootReadmePath = join(process.cwd(), 'README.md');

  try {
    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
    const newVersion = packageJson.version;
    const versionPattern = /@v\d+\.\d+\.\d+/g;
    const newVersionTag = `@v${newVersion}`;

    log(`Updating READMEs to version ${newVersion}`);

    let githubActionReadme = readFileSync(githubActionReadmePath, 'utf-8');
    const githubActionMatches = githubActionReadme.match(versionPattern);
    if (githubActionMatches) {
      log(`Found ${githubActionMatches.length} version references in github-action README`);
      githubActionReadme = githubActionReadme.replace(versionPattern, newVersionTag);
      writeFileSync(githubActionReadmePath, githubActionReadme, 'utf-8');
    }

    let rootReadme = readFileSync(rootReadmePath, 'utf-8');
    const rootMatches = rootReadme.match(versionPattern);
    if (rootMatches) {
      log(`Found ${rootMatches.length} version references in root README`);
      rootReadme = rootReadme.replace(versionPattern, newVersionTag);
      writeFileSync(rootReadmePath, rootReadme, 'utf-8');
    }

    if (githubActionMatches || rootMatches) {
      execSync('git add packages/github-action/README.md README.md', { stdio: 'inherit' });
      log('Added README files to git staging');
    } else {
      log('No version references found in README files');
    }
  } catch (error) {
    log(`Error updating READMEs: ${error.message}`);
  }
}

async function getVersionMessage() {
  log('Running getVersionMessage hook');

  updateReadmeVersions();

  return 'Version Packages';
}

module.exports = {
  getVersionMessage,
};
