import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME } from 'tscanner-common';

type TFields = 'WAYS_TO_USE_TSCANNER';
type TPackage = 'cli' | 'vscode-extension' | 'github-action';

type TPackageInfo = {
  id: TPackage;
  name: string;
  description: string;
  downloadBadges: string;
};

export function updateWaysToUseTscanner() {
  const PACKAGES: TPackageInfo[] = [
    {
      id: 'vscode-extension',
      name: 'VSCode Extension',
      description: 'Real-time sidebar integration with Git-aware branch scanning',
      downloadBadges: `<a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Extension-blue.svg" alt="VS Marketplace"></a><br /><a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/open-vsx/v/lucasvtiradentes/tscanner-vscode?label=Open%20VSX&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=a60ee5&color=374151" alt="Open VSX"></a>`,
    },
    {
      id: 'cli',
      name: 'CLI',
      description: 'Terminal scanning, CI/CD integration, pre-commit hooks',
      downloadBadges: `<a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/npm/v/tscanner?label=npm&logo=npm&logoColor=white&labelColor=CB3837&color=374151" alt="npm"></a>`,
    },
    {
      id: 'github-action',
      name: 'GitHub Action',
      description: 'CICD integration with analysis summary attached to PR comments',
      downloadBadges: `<a href="https://github.com/marketplace/actions/tscanner-action"><img src="https://img.shields.io/badge/Marketplace-black.svg?logo=github&logoColor=white&labelColor=181717&color=374151" alt="GitHub Marketplace"></a>`,
    },
  ];

  const rootDir = path.resolve(__dirname, '..', '..');

  type TOptions = {
    useFullGithubLink: boolean;
    hiddenPackages: TPackage[];
  };

  const getPackageLink = (pkg: TPackageInfo, useFullGithubLink: boolean) => {
    if (useFullGithubLink) {
      return `https://github.com/lucasvtiradentes/tscanner/tree/main/packages/${pkg.id}#readme`;
    }
    return `packages/${pkg.id}#readme`;
  };

  const getWaysToUseTscannerContent = (options: TOptions) => {
    const visiblePackages = PACKAGES.filter((pkg) => !options.hiddenPackages.includes(pkg.id));

    const rows = visiblePackages
      .map(
        (pkg) => `  <tr>
    <td>
      <div align="center">
        <b><a href="${getPackageLink(pkg, options.useFullGithubLink)}">${pkg.name}</a></b>
        <br />
        <br />
        ${pkg.downloadBadges}
      </div>
    </td>
    <td>${pkg.description}</td>
  </tr>`,
      )
      .join('\n');

    return `<table>
  <tr>
    <th>Package</th>
    <th>Description</th>
  </tr>
${rows}
</table>`;
  };

  type TReadmeConfig = {
    path: string;
    useFullGithubLink: boolean;
    hiddenPackages: TPackage[];
    wrapInDetails: boolean;
  };

  const readmeConfigs: TReadmeConfig[] = [
    { path: 'packages/cli/README.md', useFullGithubLink: true, hiddenPackages: ['cli'], wrapInDetails: true },
    {
      path: 'packages/github-action/README.md',
      useFullGithubLink: true,
      hiddenPackages: ['github-action'],
      wrapInDetails: true,
    },
    {
      path: 'packages/vscode-extension/README.md',
      useFullGithubLink: true,
      hiddenPackages: ['vscode-extension'],
      wrapInDetails: true,
    },
  ];

  readmeConfigs.forEach(({ path: filePath, useFullGithubLink, hiddenPackages, wrapInDetails }) => {
    const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
    let content = getWaysToUseTscannerContent({ useFullGithubLink, hiddenPackages });

    if (wrapInDetails) {
      content = `<details>
<summary>Other ways to use ${PACKAGE_DISPLAY_NAME}</summary>
<br />

${content}

</details>`;
    }

    readme.updateField('WAYS_TO_USE_TSCANNER', content);
    readme.saveFile();
  });

  console.log('✓ Updated WAYS_TO_USE_TSCANNER section');
}
