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

const MAX_LINES = 300;

async function main() {
  let data = '';

  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    const lineCount = file.lines.length;

    if (lineCount > MAX_LINES) {
      issues.push({
        file: file.path,
        line: 1,
        message: `File has ${lineCount} lines, exceeds maximum of ${MAX_LINES} lines`,
      });
    }
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
