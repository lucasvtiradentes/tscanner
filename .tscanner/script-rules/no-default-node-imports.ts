#!/usr/bin/env npx tsx

import { type ScriptIssue, addIssue, runScript } from '../../packages/cli/src/types';

const NODE_MODULE_REGEX = /^import\s+(?:(\w+)|(\*\s+as\s+\w+))\s+from\s+['"]node:/;

runScript((input) => {
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    for (let i = 0; i < file.lines.length; i++) {
      const line = file.lines[i];
      const match = line.match(NODE_MODULE_REGEX);

      if (match) {
        const importType = match[1] ? 'default import' : 'namespace import';
        addIssue(issues, {
          file: file.path,
          line: i + 1,
          message: `Avoid ${importType} from node: modules. Use named imports instead: import { ... } from "node:..."`,
        });
      }
    }
  }

  return issues;
});
