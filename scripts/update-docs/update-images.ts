import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'VSCODE_EXTENSION_DEMO_IMAGE' | 'CLI_DEMO_IMAGE' | 'GITHUB_ACTION_DEMO_IMAGE';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateImages() {
  const baseImageUrl = 'https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image';

  const vscodeExtensionDemoImageContent = `<div align="center">
  <img width="50%" src="${baseImageUrl}/tscanner-vscode-demo.png" alt="VS Code Extension Demo">
  <br>
  <em>issues detected in real time in the code editor</em>
</div>`;

  const cliDemoImageContent = `<div align="center">
  <img src="${baseImageUrl}/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
  <br>
  <em>scanning codebase via CLI</em>
</div>`;

  const githubActionDemoImageContent = `<div align="center">
  <img width="80%" src="${baseImageUrl}/tscanner-pr-comment-issues-found.png" alt="GitHub Action PR Comment">
  <br>
  <em>issues detected in the latest push in a PR</em>
</div>`;

  const cliImageContent = `<div align="center">
  <img src="${baseImageUrl}/tscanner-cli-demo.png" alt="CLI Scan Screenshot">
  <br>
  <em>scanning codebase via CLI</em>
</div>`;

  const rootReadme = new DynMarkdown<TFields>(path.join(rootDir, 'README.md'));
  rootReadme.updateField('VSCODE_EXTENSION_DEMO_IMAGE', vscodeExtensionDemoImageContent);
  rootReadme.updateField('CLI_DEMO_IMAGE', cliDemoImageContent);
  rootReadme.updateField('GITHUB_ACTION_DEMO_IMAGE', githubActionDemoImageContent);
  rootReadme.saveFile();

  const cliReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField('CLI_DEMO_IMAGE', cliImageContent);
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField('VSCODE_EXTENSION_DEMO_IMAGE', vscodeExtensionDemoImageContent);
  vscodeReadme.saveFile();

  console.log('✓ Updated IMAGES in 3 files (root, cli, vscode)');
}
