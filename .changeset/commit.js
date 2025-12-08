const { readFileSync, writeFileSync } = require('node:fs');
const { join } = require('node:path');
const { execSync } = require('node:child_process');

function log(message) {
  console.log(`[changeset-commit] ${message}`);
}

function updateRustWorkspaceVersion(newVersion) {
  const rustCoreFolder = join(process.cwd(), 'packages', 'rust-core');
  const cargoTomlPath = join(rustCoreFolder, 'Cargo.toml');

  try {
    let cargoToml = readFileSync(cargoTomlPath, 'utf-8');
    const versionPattern = /^(version = ")[\d.]+(")/m;

    if (versionPattern.test(cargoToml)) {
      cargoToml = cargoToml.replace(versionPattern, `$1${newVersion}$2`);
      writeFileSync(cargoTomlPath, cargoToml, 'utf-8');
      log(`Updated Rust workspace version to ${newVersion}`);

      log('Updating Cargo.lock...');
      execSync('cargo update --workspace', {
        cwd: rustCoreFolder,
        stdio: 'inherit',
      });

      execSync('git add packages/rust-core/Cargo.toml packages/rust-core/Cargo.lock', { stdio: 'inherit' });
      log('Added Cargo.toml and Cargo.lock to git staging');
    }
  } catch (error) {
    log(`Error updating Rust workspace version: ${error.message}`);
  }
}

function updateSchemaVersionInFiles(newVersion) {
  const filePaths = [
    join(process.cwd(), 'README.md'),
    join(process.cwd(), 'packages', 'cli', 'README.md'),
    join(process.cwd(), 'packages', 'vscode-extension', 'README.md'),
    join(process.cwd(), 'packages', 'github-action', 'README.md'),
  ];
  const schemaPattern = /(unpkg\.com\/tscanner@)[\d.]+(\/)schema\.json/g;

  try {
    for (const filePath of filePaths) {
      let content = readFileSync(filePath, 'utf-8');
      if (schemaPattern.test(content)) {
        schemaPattern.lastIndex = 0;
        content = content.replace(schemaPattern, `$1${newVersion}$2schema.json`);
        writeFileSync(filePath, content, 'utf-8');
        log(`Updated schema version in ${filePath} to ${newVersion}`);
      }
    }
  } catch (error) {
    log(`Error updating schema version in files: ${error.message}`);
  }
}

function updateReadmeVersions() {
  const cliPackageJsonPath = join(process.cwd(), 'packages', 'cli', 'package.json');
  const githubActionPackageJsonPath = join(process.cwd(), 'packages', 'github-action', 'package.json');
  const githubActionReadmePath = join(process.cwd(), 'packages', 'github-action', 'README.md');
  const rootReadmePath = join(process.cwd(), 'README.md');

  try {
    const cliPackageJson = JSON.parse(readFileSync(cliPackageJsonPath, 'utf-8'));
    const cliVersion = cliPackageJson.version;
    updateRustWorkspaceVersion(cliVersion);
    updateSchemaVersionInFiles(cliVersion);

    const packageJson = JSON.parse(readFileSync(githubActionPackageJsonPath, 'utf-8'));
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
