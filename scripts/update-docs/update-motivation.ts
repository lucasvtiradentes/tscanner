import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'MOTIVATION';

const rootDir = path.resolve(__dirname, '..', '..');

const getMotivationContent = () => {
  return `## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI-assisted code is great for fast iteration, but working code is just one requirement. It also needs to follow project patterns, be type-safe, and avoid code smells.

With real-time feedback on violations in the code editor and PR checks before merging, you get the best of both worlds:

1. Fast iteration
2. High-quality code that follows your standards`;
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
  readme.updateField('MOTIVATION', getMotivationContent());
  readme.saveFile();
});

console.log('✓ Updated MOTIVATION section');
