import fs from 'node:fs';
import path from 'node:path';
import { DynMarkdown } from 'markdown-helper';
import {
  CONFIG_DIR_NAME,
  CONFIG_FILE_NAME,
  IGNORE_COMMENT,
  IGNORE_NEXT_LINE_COMMENT,
  PACKAGE_DISPLAY_NAME,
  PACKAGE_NAME,
} from 'tscanner-common';

type TFields = 'COMMON_SECTION_CONFIG';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateConfigSection() {
  const getConfigSectionContent = () => {
    const fullConfigContent = fs.readFileSync(path.join(rootDir, 'assets/configs/full.json'), 'utf-8').trim();
    const minimalConfigContent = fs.readFileSync(path.join(rootDir, 'assets/configs/minimal.json'), 'utf-8').trim();

    return `## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the ${PACKAGE_DISPLAY_NAME} config folder. Here's how to get started:

1. **CLI**: Run \`${PACKAGE_NAME} init\` in your project root (**Recommended**)
2. **Manual**: Copy one of the configs below to \`${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}\`

<div align="center">
<details>
<summary><strong>Full configuration</strong></summary>

<br/>

<div align="left">

\`\`\`json
${fullConfigContent}
\`\`\`

</div>
</details>

<details>
<summary><strong>Minimal configuration</strong></summary>

<br/>

<div align="left">

\`\`\`json
${minimalConfigContent}
\`\`\`

</div>
</details>

<details>
<summary><strong>Additional info</strong></summary>

<br/>

<div align="left">

**Required fields:** The \`files.include\` and \`files.exclude\` fields are required.

**Per-rule file patterns:** Each rule can have its own \`include\`/\`exclude\` patterns:

\`\`\`json
{
  "rules": {
    "builtin": {
      "no-console": { "exclude": ["src/logger.ts"] },
      "max-function-length": { "include": ["src/core/**/*.ts"] }
    }
  }
}
\`\`\`

**Inline disables:**

\`\`\`typescript
// ${IGNORE_NEXT_LINE_COMMENT} no-explicit-any
const data: any = fetchData();

// ${IGNORE_COMMENT}
// Entire file is skipped
\`\`\`

</div>
</details>

</div>`;
  };

  const readmeConfigs = [
    { path: 'README.md' },
    { path: 'packages/cli/README.md' },
    { path: 'packages/vscode-extension/README.md' },
    { path: 'packages/github-action/README.md' },
  ];

  readmeConfigs.forEach(({ path: filePath }) => {
    const readme = new DynMarkdown<TFields>(path.join(rootDir, filePath));
    readme.updateField('COMMON_SECTION_CONFIG', getConfigSectionContent());
    readme.saveFile();
  });

  console.log('✓ Updated COMMON_SECTION_CONFIG section');
}
