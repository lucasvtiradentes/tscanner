#!/usr/bin/env npx tsx

import { type ScriptIssue, addIssue, runScript } from 'tscanner';

const MAX_LINES = 300;

runScript((input) => {
  const issues: ScriptIssue[] = [];

  for (const file of input.files) {
    const lineCount = file.lines.length;

    if (lineCount > MAX_LINES) {
      addIssue(issues, {
        file: file.path,
        line: MAX_LINES + 1,
        message: `File has ${lineCount} lines, exceeds maximum of ${MAX_LINES} lines`,
      });
    }
  }

  return issues;
});
