import path from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';
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

type Constants = {
  defaults: {
    files: {
      include: string[];
      exclude: string[];
    };
    codeEditor: {
      highlightErrors: boolean;
      highlightWarnings: boolean;
      scanInterval: number;
      aiScanInterval: number;
    };
  };
};

function getSchemaUrl(): string {
  const pkgJson = getJson(path.join(rootDir, 'packages/cli/package.json')) as { version: string };
  return `https://unpkg.com/tscanner@${pkgJson.version}/schema.json`;
}

export function updateConfigSection() {
  const getConfigSectionContent = () => {
    const constants = getJson(path.join(rootDir, 'assets/constants.json')) as Constants;
    const defaultConfigJson = {
      $schema: getSchemaUrl(),
      files: constants.defaults.files,
      codeEditor: constants.defaults.codeEditor,
    };
    const defaultConfigContent = JSON.stringify(defaultConfigJson, null, 2);

    return `## ⚙️ Configuration<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

To scan your code, you need to set up the rules in the ${PACKAGE_DISPLAY_NAME} config folder. Here's how to get started:

1. **CLI**: Run \`${PACKAGE_NAME} init\` in your project root (**Recommended**)
2. **Manual**: Copy the default config below to \`${CONFIG_DIR_NAME}/${CONFIG_FILE_NAME}\`

<div align="center">
<details>
<summary><strong>Default configuration</strong></summary>

<br/>

<div align="left">

\`\`\`json
${defaultConfigContent}
\`\`\`

</div>
</details>

<details>
<summary><strong>Additional info about configuration</strong></summary>

<br/>

<div align="left">

All configuration fields are **optional** with sensible defaults. The minimum required config is just enabling the rules you want:

\`\`\`json
{
  "rules": {
    "builtin": {
      "no-explicit-any": {}
    }
  }
}
\`\`\`

With this minimal config, ${PACKAGE_DISPLAY_NAME} will scan all \`.ts/.tsx/.js/.jsx/.mjs/.cjs\` files, excluding \`node_modules/\`, \`dist/\`, \`build/\`, and \`.git/\` directories.

**Understanding \`files.include\` and \`files.exclude\`:**

- \`files.include\`: Glob patterns for files to scan (default: \`["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"]\`)
- \`files.exclude\`: Glob patterns for files/folders to ignore (default: \`["**/node_modules/**", "**/dist/**", "**/build/**", "**/.git/**"]\`)

Example with per-rule file patterns:

\`\`\`json
{
  "rules": {
    "builtin": {
      "no-explicit-any": {},
      "no-console": {
        "exclude": ["src/utils/logger.ts"]
      },
      "max-function-length": {
        "include": ["src/core/**/*.ts"]
      }
    }
  }
}
\`\`\`

This config:
- Runs \`no-explicit-any\` on all files (uses global \`files\` patterns)
- Runs \`no-console\` on all files except \`src/utils/logger.ts\`
- Runs \`max-function-length\` only on files inside \`src/core/\`

**Inline Disables:**

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
