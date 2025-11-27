import path from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';

type TFields = 'DEFAULT_CONFIG';

const rootDir = path.resolve(__dirname, '..', '..');
const defaultConfigJson = getJson(path.join(rootDir, 'assets/default-config.json'));
const defaultConfigContent = `\`\`\`json\n${JSON.stringify(defaultConfigJson, null, 2)}\n\`\`\``;

const readmePaths = [
  path.join(rootDir, 'README.md'),
  path.join(rootDir, 'packages/core/README.md'),
  path.join(rootDir, 'packages/cli/README.md'),
  path.join(rootDir, 'packages/vscode-extension/README.md'),
  path.join(rootDir, 'packages/github-action/README.md'),
];

for (const filePath of readmePaths) {
  const readme = new DynMarkdown<TFields>(filePath);
  readme.updateField('DEFAULT_CONFIG', defaultConfigContent);
  readme.saveFile();
}

console.log(`✓ Updated DEFAULT_CONFIG in ${readmePaths.length} files`);
