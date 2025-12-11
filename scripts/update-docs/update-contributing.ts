import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { REPO_BLOB_URL } from 'tscanner-common';

type TFields = 'CONTRIBUTING';

const rootDir = resolve(__dirname, '..', '..');

export function updateContributing() {
  const getContributingContent = (useAbsoluteLink: boolean) => {
    const link = useAbsoluteLink ? `${REPO_BLOB_URL}/CONTRIBUTING.md` : 'CONTRIBUTING.md';

    return `## 🤝 Contributing<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Contributions are welcome! See [CONTRIBUTING.md](${link}) for setup instructions and development workflow.`;
  };

  const readmeConfigs = [
    { path: 'README.md', useAbsoluteLink: false },
    { path: 'packages/cli/README.md', useAbsoluteLink: true },
    { path: 'packages/github-action/README.md', useAbsoluteLink: true },
    { path: 'packages/vscode-extension/README.md', useAbsoluteLink: true },
  ];

  readmeConfigs.forEach(({ path: filePath, useAbsoluteLink }) => {
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    readme.updateField('CONTRIBUTING', getContributingContent(useAbsoluteLink));
    readme.saveFile();
  });

  console.log('✓ Updated CONTRIBUTING section');
}
