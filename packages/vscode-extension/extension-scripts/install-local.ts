import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

interface PackageJson {
  publisher: string;
  name: string;
  version: string;
}

const packageJson: PackageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const extensionName = `${packageJson.publisher}.${packageJson.name}-${packageJson.version}`;
const homeDir = os.homedir();
const targetDir = path.join(homeDir, '.vscode', 'extensions', extensionName);

if (fs.existsSync(targetDir)) {
  fs.rmSync(targetDir, { recursive: true });
}

fs.mkdirSync(targetDir, { recursive: true });

function copyRecursive(src: string, dest: string): void {
  const stat = fs.statSync(src);

  if (stat.isDirectory()) {
    if (!fs.existsSync(dest)) {
      fs.mkdirSync(dest, { recursive: true });
    }

    const entries = fs.readdirSync(src);
    for (const entry of entries) {
      copyRecursive(path.join(src, entry), path.join(dest, entry));
    }
  } else {
    fs.copyFileSync(src, dest);
  }
}

copyRecursive('out', path.join(targetDir, 'out'));
copyRecursive('resources', path.join(targetDir, 'resources'));
fs.copyFileSync('package.json', path.join(targetDir, 'package.json'));

if (fs.existsSync('README.md')) {
  fs.copyFileSync('README.md', path.join(targetDir, 'README.md'));
}

console.log(`\n✅ Extension installed to: ${targetDir}`);
console.log(`\n🔄 Reload VSCode to activate the extension:`);
console.log(`   - Press Ctrl+Shift+P`);
console.log(`   - Type "Reload Window" and press Enter\n`);
