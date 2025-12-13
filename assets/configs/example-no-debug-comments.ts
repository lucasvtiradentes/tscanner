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
      if (/\/\/\s*(DEBUG|HACK|XXX|TEMP)\b/i.test(line)) {
        issues.push({
          file: file.path,
          line: i + 1,
          message: `Debug comment found: "${line.trim().substring(0, 50)}"`,
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
