import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import { PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';

enum TFields {
  WaysToUseTscanner = 'WAYS_TO_USE_TSCANNER',
}

enum TPackage {
  Cli = 'cli',
  VscodeExtension = 'vscode-extension',
  GithubAction = 'github-action',
}

type TPackageInfo = {
  id: TPackage;
  name: string;
  description: string;
  downloadBadges: string;
};

export function updateWaysToUseTscanner() {
  const PACKAGES: TPackageInfo[] = [
    {
      id: TPackage.VscodeExtension,
      name: 'VSCode Extension',
      description: 'Live code issues in sidebar with multiple scan modes and AI clipboard export to fix them',
      downloadBadges: `<a href="https://marketplace.visualstudio.com/items?itemName=lucasvtiradentes.tscanner-vscode"><img src="https://img.shields.io/badge/VS%20Code-Extension-blue.svg" alt="VS Marketplace"></a><br /><a href="https://open-vsx.org/extension/lucasvtiradentes/tscanner-vscode"><img src="https://img.shields.io/open-vsx/v/lucasvtiradentes/tscanner-vscode?label=Open%20VSX&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=a60ee5&color=374151" alt="Open VSX"></a>`,
    },
    {
      id: TPackage.Cli,
      name: 'CLI',
      description: 'Fast terminal scanning with pre-commit hook integration',
      downloadBadges: `<a href="https://www.npmjs.com/package/tscanner"><img src="https://img.shields.io/npm/v/tscanner?label=npm&logo=npm&logoColor=white&labelColor=CB3837&color=374151" alt="npm"></a>`,
    },
    {
      id: TPackage.GithubAction,
      name: 'GitHub Action',
      description: 'CICD integration with analysis summary attached to PR comments',
      downloadBadges: `<a href="https://github.com/marketplace/actions/tscanner-action"><img src="https://img.shields.io/badge/Marketplace-black.svg?logo=github&logoColor=white&labelColor=181717&color=374151" alt="GitHub Marketplace"></a>`,
    },
  ];

  const rootDir = resolve(__dirname, '..', '..');

  type TOptions = {
    useFullGithubLink: boolean;
    hiddenPackages: TPackage[];
  };

  const getPackageLink = (pkg: TPackageInfo, useFullGithubLink: boolean) => {
    if (useFullGithubLink) {
      return `${REPO_URL}/tree/main/packages/${pkg.id}#readme`;
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
    detailsTitle: string | null;
  };

  const readmeConfigs: TReadmeConfig[] = [
    {
      path: 'README.md',
      useFullGithubLink: false,
      hiddenPackages: [],
      detailsTitle: `Ways to use ${PACKAGE_DISPLAY_NAME}`,
    },
    {
      path: 'packages/cli/README.md',
      useFullGithubLink: true,
      hiddenPackages: [TPackage.Cli],
      detailsTitle: `Other ways to use ${PACKAGE_DISPLAY_NAME}`,
    },
    {
      path: 'packages/github-action/README.md',
      useFullGithubLink: true,
      hiddenPackages: [TPackage.GithubAction],
      detailsTitle: `Other ways to use ${PACKAGE_DISPLAY_NAME}`,
    },
    {
      path: 'packages/vscode-extension/README.md',
      useFullGithubLink: true,
      hiddenPackages: [TPackage.VscodeExtension],
      detailsTitle: `Other ways to use ${PACKAGE_DISPLAY_NAME}`,
    },
  ];

  readmeConfigs.forEach(({ path: filePath, useFullGithubLink, hiddenPackages, detailsTitle }) => {
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    let content = getWaysToUseTscannerContent({ useFullGithubLink, hiddenPackages });

    if (detailsTitle) {
      content = `<details>
<summary>${detailsTitle}</summary>
<br />

${content}

</details>`;
    }

    readme.updateField(TFields.WaysToUseTscanner, content);
    readme.saveFile();
  });

  console.log('✓ Updated WAYS_TO_USE_TSCANNER section');
}
