#!/usr/bin/env npx tsx

import { stdin } from 'node:process';

type ScriptFile = {
  path: string;
  content: string;
  lines: string[];
};

type ScriptInput = {
  files: ScriptFile[];
  options?: Record<string, unknown>;
  workspaceRoot: string;
};

type ScriptIssue = {
  file: string;
  line: number;
  column?: number;
  message: string;
};

const NODE_MODULE_REGEX = /^import\s+(?:(\w+)|(\*\s+as\s+\w+))\s+from\s+['"]node:/;

async function main() {
  let data = '';

  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    for (let i = 0; i < file.lines.length; i++) {
      const line = file.lines[i];
      const match = line.match(NODE_MODULE_REGEX);

      if (match) {
        const importType = match[1] ? 'default import' : 'namespace import';
        issues.push({
          file: file.path,
          line: i + 1,
          message: `Avoid ${importType} from node: modules. Use named imports instead: import { ... } from "node:..."`,
        });
      }
    }
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
