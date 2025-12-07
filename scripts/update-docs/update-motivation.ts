import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME } from 'tscanner-common';

type TFields = 'MOTIVATION';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateMotivation() {
  const getMotivationContent = () => {
    return `## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

${PACKAGE_DISPLAY_NAME} lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards. Stop repeating yourself in code reviews.

<div align="center">

<details>
<summary>Use cases for this project</summary>
<br />

- **Project Consistency** - Enforce import styles, naming conventions, and code organization rules
- **PR Quality Gates** - Auto-comment violations before merge so reviewers focus on logic
- **AI Code Validation** - Real-time feedback on AI-generated code before accepting
- **Flexible Customization** - Built-in rules + custom scripts and AI rules for complex logic 

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
