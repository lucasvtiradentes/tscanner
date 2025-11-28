import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'INSPIRATIONS';

const rootDir = path.resolve(__dirname, '..', '..');

const getInspirationsContent = () => {
  return `## 💡 Inspirations<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

- [Biome](https://github.com/biomejs/biome) - High-performance Rust-based linter and formatter for web projects
- [VSCode Bookmarks](https://github.com/alefragnani/vscode-bookmarks) - Bookmarks Extension for Visual Studio Code`;
};

const readmeConfigs = [
  { path: 'README.md' },
  { path: 'packages/cli/README.md' },
  { path: 'packages/github-action/README.md' },
  { path: 'packages/vscode-extension/README.md' },
  { path: 'packages/core/README.md' },
];

readmeConfigs.forEach(({ path: filePath }) => {
  const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
  readme.updateField('INSPIRATIONS', getInspirationsContent());
  readme.saveFile();
});

console.log('✓ Updated INSPIRATIONS section');
