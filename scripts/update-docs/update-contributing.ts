import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'CONTRIBUTING';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateContributing() {
  const getContributingContent = (useAbsoluteLink: boolean) => {
    const link = useAbsoluteLink
      ? 'https://github.com/lucasvtiradentes/tscanner/blob/main/CONTRIBUTING.md'
      : 'CONTRIBUTING.md';

    return `## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](${link}) for setup instructions and development workflow.

**Quick Setup:**

\`\`\`bash
git clone https://github.com/lucasvtiradentes/tscanner.git
cd tscanner
pnpm install
pnpm run build
\`\`\``;
  };

  const readmeConfigs = [
    { path: 'README.md', useAbsoluteLink: false },
    { path: 'packages/cli/README.md', useAbsoluteLink: true },
    { path: 'packages/github-action/README.md', useAbsoluteLink: true },
    { path: 'packages/vscode-extension/README.md', useAbsoluteLink: true },
    { path: 'packages/core/README.md', useAbsoluteLink: true },
  ];

  readmeConfigs.forEach(({ path: filePath, useAbsoluteLink }) => {
    const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
    readme.updateField('CONTRIBUTING', getContributingContent(useAbsoluteLink));
    readme.saveFile();
  });

  console.log('✓ Updated CONTRIBUTING section');
}
