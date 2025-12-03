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

function addIssue(issues: ScriptIssue[], file: string, line: number, message: string): void {
  issues.push({ file, line, message });
}

function analyzeModRs(file: ScriptFile, issues: ScriptIssue[]): void {
  const lines = file.content.split('\n');
  let inMultilineUse = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trim();

    if (trimmed === '' || trimmed.startsWith('//')) {
      continue;
    }

    if (inMultilineUse) {
      if (trimmed.includes('};') || trimmed === '};' || (trimmed.endsWith(';') && !trimmed.includes('{'))) {
        inMultilineUse = false;
      }
      continue;
    }

    if (/^(pub\s+)?use\s+.*\{[^}]*$/.test(trimmed)) {
      inMultilineUse = true;
      continue;
    }

    if (/^(pub\s+)?mod\s+\w+;$/.test(trimmed)) {
      continue;
    }

    if (/^(pub\s+)?use\s+.*;\s*$/.test(trimmed)) {
      continue;
    }

    const msg = `mod.rs contains logic beyond imports/exports: "${trimmed.substring(0, 50)}${trimmed.length > 50 ? '...' : ''}"`;
    addIssue(issues, file.path, i + 1, msg);
    return;
  }
}

async function main() {
  let data = '';

  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    if (!file.path.endsWith('mod.rs')) {
      continue;
    }

    analyzeModRs(file, issues);
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
