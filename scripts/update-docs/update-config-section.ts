import path from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';

type TFieldsReadme = 'COMMON_SECTION_CONFIG';

const rootDir = path.resolve(__dirname, '..', '..');

const defaultConfigJson = getJson(path.join(rootDir, 'assets/default-config.json'));
const defaultConfigContent = JSON.stringify(defaultConfigJson, null, 2);

const configSectionContent = `## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To create TScanner configuration, you can use the following command:

\`\`\`bash
tscanner init
\`\`\`

or go the \`VSCode Extension\` \`status bar\` and click on the \`Manage Rules\` button, select the rules you want to enable and click on the \`Save\` button.

The default configuration is:

\`\`\`json
${defaultConfigContent}
\`\`\`

**Inline Disables:**

\`\`\`typescript
// tscanner-disable-next-line no-any-type
const data: any = fetchData();

// tscanner-disable-file
// Entire file is skipped
\`\`\``;

const readmePaths = [
  path.join(rootDir, 'README.md'),
  path.join(rootDir, 'packages/core/README.md'),
  path.join(rootDir, 'packages/cli/README.md'),
  path.join(rootDir, 'packages/vscode-extension/README.md'),
  path.join(rootDir, 'packages/github-action/README.md'),
];

for (const filePath of readmePaths) {
  const readme = new DynMarkdown<TFieldsReadme>(filePath);
  readme.updateField('COMMON_SECTION_CONFIG', configSectionContent);
  readme.saveFile();
}

console.log(`✓ Updated COMMON_SECTION_CONFIG in ${readmePaths.length} files`);
