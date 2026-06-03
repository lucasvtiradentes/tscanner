import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';

enum ImageField {
  VscodeExtensionDemo = 'VSCODE_EXTENSION_DEMO_IMAGE',
  CliDemo = 'CLI_DEMO_IMAGE',
  GithubActionDemo = 'GITHUB_ACTION_DEMO_IMAGE',
}

const rootDir = resolve(__dirname, '..', '..');

export function updateImages() {
  const baseImageUrl = 'https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image';

  const vscodeExtensionDemoImageContent = `<div align="center">
  <a href="${baseImageUrl}/tscanner-vscode-demo.png" target="_blank"><img src="${baseImageUrl}/tscanner-vscode-demo.png" alt="VS Code Extension Demo"></a>
  <br>
  <em>issues detected in real time in the code editor</em>
</div>`;

  const cliDemoImageContent = `<div align="center">
  <a href="${baseImageUrl}/tscanner-cli-demo.png" target="_blank"><img src="${baseImageUrl}/tscanner-cli-demo.png" alt="CLI Scan Screenshot"></a>
  <br>
  <em>scanning the codebase via CLI</em>
</div>`;

  const githubActionDemoImageContent = `<div align="center">
  <a href="${baseImageUrl}/tscanner-pr-comment-warnings-found.png" target="_blank"><img width="80%" src="${baseImageUrl}/tscanner-pr-comment-warnings-found.png" alt="GitHub Action PR Comment"></a>
  <br>
  <em>issues detected in the latest commit pushed to a PR</em>
</div>`;

  const rootReadme = new DynMarkdown<ImageField>(join(rootDir, 'README.md'));
  rootReadme.updateField(ImageField.VscodeExtensionDemo, vscodeExtensionDemoImageContent);
  rootReadme.updateField(ImageField.CliDemo, cliDemoImageContent);
  rootReadme.updateField(ImageField.GithubActionDemo, githubActionDemoImageContent);
  rootReadme.saveFile();

  const cliReadme = new DynMarkdown<ImageField>(join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField(ImageField.CliDemo, cliDemoImageContent);
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<ImageField>(join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField(ImageField.VscodeExtensionDemo, vscodeExtensionDemoImageContent);
  vscodeReadme.saveFile();

  const gihubReadme = new DynMarkdown<ImageField>(join(rootDir, 'packages/github-action/README.md'));
  gihubReadme.updateField(ImageField.GithubActionDemo, githubActionDemoImageContent);
  gihubReadme.saveFile();

  console.log('✓ Updated IMAGES in 3 files (root, cli, vscode)');
}
