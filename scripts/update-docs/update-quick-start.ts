import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'QUICK_START_CLI' | 'QUICK_START_VSCODE_EXTENSION' | 'QUICK_START_GITHUB_ACTION';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateQuickStart() {
  const quickStartVscodeExtension = `### [VSCode Extension](packages/vscode-extension#readme)

1. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "TScanner" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="TScanner installation"></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

2. Click TScanner icon in activity bar
3. Go to Settings Menu → "Manage Rules" → enable desired rules -> click "Save"
4. Issues appear automatically in the sidebar (if any)
5. Click any issue to jump to its location`;

  const quickStartContentMain = `### [CLI](packages/cli#readme)

1. Install globally

\`\`\`bash
npm install -g tscanner
\`\`\`

2. Initialize configuration

\`\`\`bash
tscanner init
\`\`\`

3. Use it

\`\`\`bash
# Scan workspace
tscanner check

# Scan only changed files vs branch
tscanner check --branch origin/main
\`\`\``;

  const quickStartContentCli = `## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

1. Install globally

\`\`\`bash
npm install -g tscanner
\`\`\`

2. Initialize configuration

\`\`\`bash
tscanner init
\`\`\`

3. Use it

\`\`\`bash
# Scan workspace
tscanner check

# Scan only changed files vs branch
tscanner check --branch origin/main
\`\`\``;

  const quickStartVscodeExtensionReadme = `## 🚀 Quick Start<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

1. Install the extension:

<div align="center">

<table>
  <tr>
    <th>Search "TScanner" in Extensions</th>
    <th>Install from marketplace</th>
  </tr>
  <tr>
    <td><img width="300" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-install.png" alt="TScanner installation"></td>
    <td>
      <div align="center">
      <a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Marketplace-007ACC?logo=visual-studio-code&logoColor=white" alt="VS Code"></a><br/>
      <a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/badge/Open%20VSX-Registry-a60ee5?logo=eclipse-ide&logoColor=white" alt="Open VSX"></a>
      </div>
    </td>
  </tr>
</table>
</div>

2. Click TScanner icon in activity bar
3. Go to Settings Menu → "Manage Rules" → enable desired rules -> click "Save"
4. Issues appear automatically in the sidebar (if any)`;

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
      - uses: lucasvtiradentes/tscanner-action@v0.0.17
        with:
          github-token: \${{ secrets.GITHUB_TOKEN }}
\`\`\``;

  const quickStartGithubAction = `1. Create \`.github/workflows/tscanner.yml\`:

${quickStartGithubActionYaml}

2. Add TScanner config to your repo (run \`tscanner init\` or create \`.tscanner/config.jsonc\`)
3. Open a PR and watch the magic happen!`;

  const mainReadme = new DynMarkdown<TFields>(path.join(rootDir, 'README.md'));
  mainReadme.updateField('QUICK_START_VSCODE_EXTENSION', quickStartVscodeExtension);
  mainReadme.updateField('QUICK_START_CLI', quickStartContentMain);
  mainReadme.updateField('QUICK_START_GITHUB_ACTION', quickStartGithubAction);
  mainReadme.saveFile();

  const cliReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/cli/README.md'));
  cliReadme.updateField('QUICK_START_CLI', quickStartContentCli);
  cliReadme.saveFile();

  const vscodeReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField('QUICK_START_VSCODE_EXTENSION', quickStartVscodeExtensionReadme);
  vscodeReadme.saveFile();

  const githubActionReadme = new DynMarkdown<TFields>(path.join(rootDir, 'packages/github-action/README.md'));
  githubActionReadme.updateField('QUICK_START_GITHUB_ACTION', quickStartGithubAction);
  githubActionReadme.saveFile();

  console.log('✓ Updated QUICK_START sections');
}
