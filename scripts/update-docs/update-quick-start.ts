import fs from 'node:fs';
import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { CONFIG_DIR_NAME, CONFIG_FILE_NAME, PACKAGE_DISPLAY_NAME, PACKAGE_NAME } from 'tscanner-common';

type TFields = 'QUICK_START_CLI' | 'QUICK_START_VSCODE_EXTENSION' | 'QUICK_START_GITHUB_ACTION';

const rootDir = path.resolve(__dirname, '..', '..');

function getGithubActionVersion(): string {
  const pkgPath = path.join(rootDir, 'packages/github-action/package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
  return pkg.version;
}

function getGithubActionSection() {
  const version = getGithubActionVersion();
  const quickStartGithubActionYaml = `\`\`\`yaml
name: Code Quality

on:
  pull_request:
    branches: [main]

jobs:
  tscanner:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: lucasvtiradentes/tscanner-action@v${version}
        with:
          github-token: \${{ secrets.GITHUB_TOKEN }}
\`\`\``;

  const quickStartGithubAction = `1. Create \`.github/workflows/tscanner.yml\`:

${quickStartGithubActionYaml}

2. Add ${PACKAGE_DISPLAY_NAME} config to your repo (run \`${PACKAGE_NAME} init\` or create \`${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}\`)
3. Open a PR and watch the magic happen!`;

  return quickStartGithubAction;
}

function getCliSection() {
  const quickStartContentCli = `1. Install globally

\`\`\`bash
npm install -g ${PACKAGE_NAME}
\`\`\`

2. Initialize configuration

\`\`\`bash
${PACKAGE_NAME} init
\`\`\`

3. Use it

\`\`\`bash
# Scan workspace
${PACKAGE_NAME} check

# Scan only changed files vs branch
${PACKAGE_NAME} check --branch origin/main
\`\`\``;

  return quickStartContentCli;
}

function getVscodeExtensionSection() {
  const quickStartVscodeExtension = `1. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "${PACKAGE_DISPLAY_NAME}" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="${PACKAGE_DISPLAY_NAME} installation"></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

2. Click ${PACKAGE_DISPLAY_NAME} icon in activity bar
3. Go to Settings Menu → "Manage Rules" → enable desired rules → click "Save"
4. Issues appear automatically in the sidebar (if any)
5. Click any issue to jump to its location`;

  return quickStartVscodeExtension;
}

export function updateQuickStart() {
  const githubActionQuickStart = getGithubActionSection();
  const cliQuickStart = getCliSection();
  const vscodeQuickStart = getVscodeExtensionSection();

  const mainReadme = new DynMarkdown<TFields>(path.join(rootDir, 'README.md'));
  mainReadme.updateField('QUICK_START_VSCODE_EXTENSION', vscodeQuickStart);
  mainReadme.updateField('QUICK_START_CLI', cliQuickStart);
  mainReadme.updateField('QUICK_START_GITHUB_ACTION', githubActionQuickStart);
  mainReadme.saveFile();

  const cliReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField('QUICK_START_CLI', cliQuickStart);
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField('QUICK_START_VSCODE_EXTENSION', vscodeQuickStart);
  vscodeReadme.saveFile();

  const githubActionReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/github-action/README.md'));
  githubActionReadme.updateField('QUICK_START_GITHUB_ACTION', githubActionQuickStart);
  githubActionReadme.saveFile();

  console.log('✓ Updated QUICK_START sections');
}
