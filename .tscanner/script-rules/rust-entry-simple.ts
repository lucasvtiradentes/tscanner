#!/usr/bin/env npx tsx

import { type ScriptFile, type ScriptIssue, addIssue, runScript } from '../../packages/cli/src/types'; // from 'tscanner'

function analyzeEntryFile(file: ScriptFile, issues: ScriptIssue[]): void {
  const lines = file.content.split('\n');
  let inMultilineUse = false;
  const fileName = file.path.endsWith('lib.rs') ? 'lib.rs' : 'mod.rs';

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trim();

    if (trimmed === '' || trimmed.startsWith('//') || trimmed.startsWith('#!')) {
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

    const msg = `${fileName} contains logic beyond imports/exports: "${trimmed.substring(0, 50)}${trimmed.length > 50 ? '...' : ''}"`;
    addIssue(issues, { file: file.path, line: i + 1, message: msg });
    return;
  }
}

runScript((input) => {
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    if (!file.path.endsWith('mod.rs') && !file.path.endsWith('lib.rs')) {
      continue;
    }
    analyzeEntryFile(file, issues);
  }

  return issues;
});
