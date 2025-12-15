import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';

enum ImageFields {
  VscodeExtensionDemoImage = 'VSCODE_EXTENSION_DEMO_IMAGE',
  CliDemoImage = 'CLI_DEMO_IMAGE',
  GithubActionDemoImage = 'GITHUB_ACTION_DEMO_IMAGE',
}

const rootDir = resolve(__dirname, '..', '..');

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
  <em>scanning the codebase via CLI</em>
</div>`;

  const githubActionDemoImageContent = `<div align="center">
  <img width="80%" src="${baseImageUrl}/tscanner-pr-comment-warnings-found.png" alt="GitHub Action PR Comment">
  <br>
  <em>issues detected in the latest commit pushed to a PR</em>
</div>`;

  const rootReadme = new DynMarkdown<ImageFields>(join(rootDir, 'README.md'));
  rootReadme.updateField(ImageFields.VscodeExtensionDemoImage, vscodeExtensionDemoImageContent);
  rootReadme.updateField(ImageFields.CliDemoImage, cliDemoImageContent);
  rootReadme.updateField(ImageFields.GithubActionDemoImage, githubActionDemoImageContent);
  rootReadme.saveFile();

  const cliReadme = new DynMarkdown<ImageFields>(join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField(ImageFields.CliDemoImage, cliDemoImageContent);
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<ImageFields>(join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField(ImageFields.VscodeExtensionDemoImage, vscodeExtensionDemoImageContent);
  vscodeReadme.saveFile();

  const gihubReadme = new DynMarkdown<ImageFields>(join(rootDir, 'packages/github-action/README.md'));
  gihubReadme.updateField(ImageFields.GithubActionDemoImage, githubActionDemoImageContent);
  gihubReadme.saveFile();

  console.log('✓ Updated IMAGES in 3 files (root, cli, vscode)');
}
