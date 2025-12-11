import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'FOOTER';

const rootDir = resolve(__dirname, '..', '..');

export function updateFooter() {
  const getFooterContent = () => {
    return `<div width="100%" align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png" />
</div>

<br />

<div align="center">
  <div>
    <a target="_blank" href="https://www.linkedin.com/in/lucasvtiradentes/"><img src="https://img.shields.io/badge/-linkedin-blue?logo=Linkedin&logoColor=white" alt="LinkedIn"></a>
    <a target="_blank" href="mailto:lucasvtiradentes@gmail.com"><img src="https://img.shields.io/badge/gmail-red?logo=gmail&logoColor=white" alt="Gmail"></a>
    <a target="_blank" href="https://x.com/lucasvtiradente"><img src="https://img.shields.io/badge/-X-black?logo=X&logoColor=white" alt="X"></a>
    <a target="_blank" href="https://github.com/lucasvtiradentes"><img src="https://img.shields.io/badge/-github-gray?logo=Github&logoColor=white" alt="Github"></a>
  </div>
</div>`;
  };

  const readmeConfigs = [
    'README.md',
    'packages/cli/README.md',
    'packages/github-action/README.md',
    'packages/vscode-extension/README.md',
  ];

  readmeConfigs.forEach((filePath) => {
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    readme.updateField('FOOTER', getFooterContent());
    readme.saveFile();
  });

  console.log('✓ Updated FOOTER section');
}
