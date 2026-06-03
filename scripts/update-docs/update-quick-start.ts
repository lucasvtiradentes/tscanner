import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME, PACKAGE_NAME } from 'tscanner-common';

enum QuickStartField {
  Cli = 'QUICK_START_CLI',
  VscodeExtension = 'QUICK_START_VSCODE_EXTENSION',
  GithubAction = 'QUICK_START_GITHUB_ACTION',
  Install = 'QUICK_START_INSTALL',
}

const rootDir = resolve(__dirname, '..', '..');

function getGithubActionVersion(): string {
  const pkgPath = join(rootDir, 'packages/github-action/package.json');
  const pkg = JSON.parse(readFileSync(pkgPath, 'utf-8'));
  return pkg.version;
}

function getInstallSection() {
  return `1. Install locally

\`\`\`bash
npm install -D ${PACKAGE_NAME}
\`\`\`

2. Initialize configuration

\`\`\`bash
npx ${PACKAGE_NAME} init
\`\`\`

> TIP: Use \`npx ${PACKAGE_NAME} init --minimal\` for a smaller starter config with only one built-in rule.

`;
}

function getGithubActionSection(startStep = 1) {
  const version = getGithubActionVersion();
  const quickStartGithubAction = `${startStep}. Create \`.github/workflows/tscanner.yml\`:

\`\`\`yaml
name: Code Quality
on: [pull_request]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: lucasvtiradentes/tscanner-action@v${version}
        with:
          github-token: \${{ secrets.GITHUB_TOKEN }}
\`\`\`

${startStep + 1}. Open a PR to see it in action`;

  return quickStartGithubAction;
}

function getCliSection(startStep = 1) {
  const quickStartContentCli = `${startStep}. Check via terminal

\`\`\`bash
# Scan workspace
npx ${PACKAGE_NAME} check

# Scan uncommitted changes (staged + unstaged)
npx ${PACKAGE_NAME} check --uncommitted

# Scan only changed files vs branch
npx ${PACKAGE_NAME} check --branch origin/main
\`\`\`

${startStep + 1}. Integrate with [lint-staged](https://github.com/lint-staged/lint-staged) (optional)

\`\`\`jsonc
// .lintstagedrc.json
{
  "*": ["npx ${PACKAGE_NAME} check --staged"]
}
\`\`\``;

  return quickStartContentCli;
}

function getVscodeExtensionSection(startStep = 1) {
  const quickStartVscodeExtension = `${startStep}. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "${PACKAGE_DISPLAY_NAME}" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><a href="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" target="_blank"><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="${PACKAGE_DISPLAY_NAME} installation"></a></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

${startStep + 1}. Click ${PACKAGE_DISPLAY_NAME} icon in activity bar
${startStep + 2}. Issues appear automatically in the sidebar (if any)
${startStep + 3}. Click any issue to jump to its location`;

  return quickStartVscodeExtension;
}

export function updateQuickStart() {
  const installQuickStart = getInstallSection();
  const githubActionQuickStart = getGithubActionSection();
  const cliQuickStart = getCliSection();
  const vscodeQuickStart = getVscodeExtensionSection();

  const mainReadme = new DynMarkdown<QuickStartField>(join(rootDir, 'README.md'));
  mainReadme.updateField(QuickStartField.Install, installQuickStart);
  mainReadme.updateField(QuickStartField.VscodeExtension, vscodeQuickStart);
  mainReadme.updateField(QuickStartField.Cli, cliQuickStart);
  mainReadme.updateField(QuickStartField.GithubAction, githubActionQuickStart);
  mainReadme.saveFile();

  const cliReadme = new DynMarkdown<QuickStartField>(join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField(QuickStartField.Install, installQuickStart);
  cliReadme.updateField(QuickStartField.Cli, getCliSection(3));
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<QuickStartField>(join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField(QuickStartField.Install, installQuickStart);
  vscodeReadme.updateField(QuickStartField.VscodeExtension, getVscodeExtensionSection(3));
  vscodeReadme.saveFile();

  const githubActionReadme = new DynMarkdown<QuickStartField>(join(rootDir, 'packages/github-action/README.md'));
  githubActionReadme.updateField(QuickStartField.Install, installQuickStart);
  githubActionReadme.updateField(QuickStartField.GithubAction, getGithubActionSection(3));
  githubActionReadme.saveFile();

  console.log('✓ Updated QUICK_START sections');
}
