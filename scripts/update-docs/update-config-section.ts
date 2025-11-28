import path from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';

type TFields = 'COMMON_SECTION_CONFIG';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateConfigSection() {
  const getConfigSectionContent = () => {
    const defaultConfigJson = getJson(path.join(rootDir, 'assets/default-config.json'));
    const defaultConfigContent = JSON.stringify(defaultConfigJson, null, 2);

    return `## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the TScanner config folder. Here's how to get started:

1. **VSCode Extension**: Click on TScanner icon in the status bar → \`Manage Rules\` → Select desired rules → \`Save\`
2. **CLI**: Run \`tscanner init\` in your project root
3. **Manual**: Copy the default config below to \`.tscanner/config.json\`

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
  };

  const readmeConfigs = [
    { path: 'README.md' },
    { path: 'packages/cli/README.md' },
    { path: 'packages/vscode-extension/README.md' },
    { path: 'packages/github-action/README.md' },
  ];

  readmeConfigs.forEach(({ path: filePath }) => {
    const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
    readme.updateField('COMMON_SECTION_CONFIG', getConfigSectionContent());
    readme.saveFile();
  });

  console.log('✓ Updated COMMON_SECTION_CONFIG section');
}
