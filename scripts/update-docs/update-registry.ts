import { join, resolve } from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';
import { PACKAGE_NAME, REPO_URL } from 'tscanner-common';

type TFields = 'REGISTRY';

type RegistryRule = {
  name: string;
  kind: 'ai' | 'script' | 'regex';
  category: string;
  description: string;
  file?: string;
};

type RegistryIndex = {
  rules: RegistryRule[];
};

const rootDir = resolve(__dirname, '..', '..');

export function updateRegistry() {
  const registryIndex: RegistryIndex = getJson(join(rootDir, 'registry/index.json'));
  const rules = registryIndex.rules;

  const aiRules = rules.filter((r) => r.kind === 'ai');
  const scriptRules = rules.filter((r) => r.kind === 'script');
  const regexRules = rules.filter((r) => r.kind === 'regex');

  const kindBadge = (kind: string) => {
    const colors: Record<string, string> = {
      ai: '8B5CF6',
      script: '10B981',
      regex: '6C757D',
    };
    return `<img src="https://img.shields.io/badge/${kind}-${colors[kind] ?? '6C757D'}" alt="${kind}">`;
  };

  const langBadge = (file?: string) => {
    if (!file) return '-';
    const ext = file.split('.').pop() ?? '';
    const langs: Record<string, { label: string; color: string; logo: string }> = {
      ts: { label: 'TypeScript', color: '3178C6', logo: 'typescript' },
      js: { label: 'JavaScript', color: 'F7DF1E', logo: 'javascript' },
      mjs: { label: 'JavaScript', color: 'F7DF1E', logo: 'javascript' },
      cjs: { label: 'JavaScript', color: 'F7DF1E', logo: 'javascript' },
      py: { label: 'Python', color: '3776AB', logo: 'python' },
      rs: { label: 'Rust', color: 'DEA584', logo: 'rust' },
      go: { label: 'Go', color: '00ADD8', logo: 'go' },
      rb: { label: 'Ruby', color: 'CC342D', logo: 'ruby' },
      sh: { label: 'Bash', color: '4EAA25', logo: 'gnubash' },
      bash: { label: 'Bash', color: '4EAA25', logo: 'gnubash' },
      zsh: { label: 'Zsh', color: '4EAA25', logo: 'zsh' },
      lua: { label: 'Lua', color: '2C2D72', logo: 'lua' },
      php: { label: 'PHP', color: '777BB4', logo: 'php' },
      java: { label: 'Java', color: 'ED8B00', logo: 'openjdk' },
      kt: { label: 'Kotlin', color: '7F52FF', logo: 'kotlin' },
      swift: { label: 'Swift', color: 'F05138', logo: 'swift' },
      cs: { label: 'C#', color: '512BD4', logo: 'csharp' },
      cpp: { label: 'C++', color: '00599C', logo: 'cplusplus' },
      c: { label: 'C', color: 'A8B9CC', logo: 'c' },
      md: { label: 'Markdown', color: '083fa1', logo: 'markdown' },
    };
    const lang = langs[ext];
    if (!lang) return '-';
    return `<img src="https://img.shields.io/badge/${lang.label}-${lang.color}?logo=${lang.logo}&logoColor=white" alt="${lang.label}">`;
  };

  const getRuleUrl = (rule: RegistryRule) => {
    const folderMap: Record<string, string> = {
      ai: 'ai-rules',
      script: 'script-rules',
      regex: 'regex-rules',
    };
    const folder = folderMap[rule.kind];
    const defaultFile: Record<string, string> = {
      ai: 'prompt.md',
      script: 'script.ts',
      regex: 'config.jsonc',
    };
    const file = rule.file ?? defaultFile[rule.kind];
    return `${REPO_URL}/blob/main/registry/${folder}/${rule.name}/${file}`;
  };

  const rulesTableRows = rules
    .map(
      (r) =>
        `  <tr>
    <td><a href="${getRuleUrl(r)}"><code>${r.name}</code></a></td>
    <td>${kindBadge(r.kind)}</td>
    <td>${langBadge(r.file)}</td>
    <td>${r.description}</td>
  </tr>`,
    )
    .join('\n');

  const getRegistryContent = () => {
    return `## 📦 Registry<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

The registry is a collection of community rules ready to install with a single command.

\`\`\`bash
npx ${PACKAGE_NAME} registry                     # List all available rules (and you chose the ones you want to install)
npx ${PACKAGE_NAME} registry no-long-files       # Install a specific rule
npx ${PACKAGE_NAME} registry --kind script       # Filter by type (ai, script, regex)
npx ${PACKAGE_NAME} registry --category security # Filter by category
npx ${PACKAGE_NAME} registry --latest            # Use rules from main branch instead of current version
\`\`\`

<div align="center">

**Available rules (${rules.length})**

<table>
  <tr>
    <th width="33%">Rule</th>
    <th width="17%">Type</th>
    <th width="17%">Language</th>
    <th width="33%">Description</th>
  </tr>
${rulesTableRows}
</table>

</div>

<br />

> **Want to share your rule?** Open a PR adding your rule to the [\`registry/\`](${REPO_URL}/tree/main/registry) folder. Once merged, everyone can install it with \`npx ${PACKAGE_NAME} registry your-rule-name\`.
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
    readme.updateField('REGISTRY', getRegistryContent());
    readme.saveFile();
  });

  console.log(
    `✓ Updated REGISTRY section (${rules.length} rules: ${aiRules.length} ai, ${scriptRules.length} script, ${regexRules.length} regex)`,
  );
}
