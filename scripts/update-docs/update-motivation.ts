import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'MOTIVATION';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateMotivation() {
  const getMotivationContent = () => {
    return `## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

TScanner lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.

<br />

<div align="center">


<details>
<summary>Understand why TScanner is not just for AI generated code</summary>
<br />

<div align="left">

We highlight AI in our messaging because it attracts attention, but TScanner solves a problem as old as programming itself.

Developers forget conventions. Teams struggle to maintain consistency. Code reviews become repetitive. These issues exist whether you use Copilot or type every character yourself.

TScanner automates the enforcement of your standards, freeing you from being the "pattern police" in every pull request.

</div>

</details>

</div>
`;
  };

  const readmeConfigs = [
    { path: 'README.md' },
    { path: 'packages/cli/README.md' },
    { path: 'packages/github-action/README.md' },
    { path: 'packages/vscode-extension/README.md' },
  ];

  readmeConfigs.forEach(({ path: filePath }) => {
    const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
    readme.updateField('MOTIVATION', getMotivationContent());
    readme.saveFile();
  });

  console.log('✓ Updated MOTIVATION section');
}
