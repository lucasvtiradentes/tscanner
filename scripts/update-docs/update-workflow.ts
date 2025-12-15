import { join, resolve } from 'node:path';
import { DynMarkdown } from 'markdown-helper';

type TFields = 'WORKFLOW';

const rootDir = resolve(__dirname, '..', '..');

export function updateWorkflow() {
  const getMotivationContent = () => {
    return `
## 🔀 Workflow <a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

Here is a diagram that shows how TScanner fits into the average coding workflow:

<div align="center">
  <img src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/tscanner-and-the-coding-workflow.png" alt="TScanner and the coding workflow">
</div>

Legend: 

- TS1: before commit, you can see issues in the code editor; also you can add it to lintstaged so no error will be committed (unless you want)
- TS2: before opening a PR, you can check all the issues in your branch compared to origin/main and fix them all
- TS3: every new commit push to a PR will be checked for issues and you'll be notified about them in a single comment with clickable links to the exact lines

So what? 

- this will allow you to go fast plus knowing exactly what issues you need to fix before committing or merging.
- this will, over time, reduce to zero the rejected pr's due to **styling or poor code quality patterns**, as long as you keep the rules updated.
  - so our job is to detect code patterns to avoid/enforce and add tscanner rules for that 

<br />

<div align="center">

<details>
<summary>How am I using this to improve my code at work?</summary>

<br />

<div align="left">

I basically observe code patterns to enforce/avoid and add custom rules, here are my current rules: 

regex rules: 

\`\`\`jsonc
"regex": {
  "no-nestjs-logger": {
    "pattern": "import\\\\s*\\\\{[^}]*Logger[^}]*\\\\}\\\\s*from\\\\s*['\\"]@nestjs/common['\\"]",
    "message": "Do not use NestJS Logger. Import from custom logger instead"
  },
  "no-typeorm-for-feature": {
    "pattern": "TypeOrmModule\\\\.forFeature\\\\(",
    "message": "Use api/src/way-type-orm.module.ts instead"
  },
  "avoid-typeorm-raw-queries": {
    "pattern": "await this\\\\.([^.]+)\\\\.manager\\\\.query\\\\(",
    "message": "Avoid using RawQueryBuilder. Use the repository instead",
    "severity": "error"
  },
  "no-static-zod-schema": {
    "pattern": "static\\\\s+zodSchema\\\\s*=",
    "message": "Remove 'static zodSchema' from class. The schema is already passed to createZodDto() and this property is redundant"
  }
}
\`\`\`

script rules:

\`\`\`jsonc
"script": {
  "entity-registered-in-typeorm-module": {
    "command": "npx tsx script-rules/entity-registered-in-typeorm-module.ts",
    "message": "Entity must be registered in way-type-orm.module.ts",
    "severity": "error",
    "include": ["api/src/**/*.entity.ts", "api/src/way-type-orm.module.ts"]
  },
  "entity-registered-in-setup-nest": {
    "command": "npx tsx script-rules/entity-registered-in-setup-nest.ts",
    "message": "Entity must be registered in setup-nest.ts for tests",
    "severity": "error",
    "include": ["api/src/**/*.entity.ts", "api/test/helpers/setup-nest.ts"]
  },
  "no-long-files": {
    "command": "npx tsx script-rules/no-long-files.ts",
    "message": "File exceeds 600 lines limit",
    "include": ["**/*.ts"]
  }
}
\`\`\`

ai rules: 

\`\`\`
soon!
\`\`\`

Note: my rules at work are not commited to the codebase, so I basically installed tscanner globally and move the \`.tscanner\` folder into the \`.gitignore\` file

</div>

</details>

</div>

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
    readme.updateField('WORKFLOW', getMotivationContent());
    readme.saveFile();
  });

  console.log('✓ Updated MOTIVATION section');
}
