import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME } from 'tscanner-common';

type TFields = 'MOTIVATION';

const rootDir = resolve(__dirname, '..', '..');

export function updateMotivation() {
  const getMotivationContent = () => {
    return `## ❓ Motivation<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

AI generates code fast, but it doesn't know your project's conventions, preferred patterns, or forbidden shortcuts. You end up reviewing the same issues over and over.

${PACKAGE_DISPLAY_NAME} lets you define those rules once. Every AI-generated file, every PR, every save: automatically checked against your standards.

Here is a diagram that shows how TScanner fits into the coding workflow:

<div align="center">
  <img width="80%" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-and-the-coding-workflow.png" alt="TScanner and the coding workflow">
  <br>
  <em>TScanner and the coding workflow</em>
</div>

Legend: 

- TS1: before commit, you can see issues in the code editor; also you can add it to lintstaged so no error will be committed (unless you want)
- TS2: before opening a PR, you can check all the issues in your branch compared to origin/main and fix them all
- TS3: every new commit push to a PR will be checked for issues and you'll be notified about them in a single comment with clickable links to the exact lines

So what? 

- this will allow you to go fast plus knowing exactly what issues you need to fix before merging or committing.
- this will reduce to zero the rejected pr's due to **styling or poor code quality patterns**.

<div align="center">

<details>
<summary>Use cases for this project</summary>
<br />

<div align="left">

- **Project Consistency** - Enforce import styles, naming conventions, and code organization rules
- **PR Quality Gates** - Auto-comment violations before merge so reviewers focus on logic
- **AI Code Validation** - Real-time feedback on AI-generated code before accepting
- **Flexible Customization** - Built-in rules + custom scripts and AI rules for complex logic 

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
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    readme.updateField('MOTIVATION', getMotivationContent());
    readme.saveFile();
  });

  console.log('✓ Updated MOTIVATION section');
}
