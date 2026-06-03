#!/usr/bin/env npx tsx

import { existsSync, readFileSync } from 'node:fs';
import { isAbsolute, join, relative, resolve } from 'node:path';

const maxLines = 300;

type Issue = {
  file: string;
  line: number;
  message: string;
};

function parseArgs(argv = process.argv.slice(2)): { files: string[] | null; root: string } {
  let rootArg: string | undefined;
  let sawFiles = false;
  const files: string[] = [];

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--files') {
      sawFiles = true;
      files.push(...parseFiles(argv[index + 1] ?? ''));
      index += 1;
      continue;
    }
    if (arg.startsWith('--files=')) {
      sawFiles = true;
      files.push(...parseFiles(arg.slice('--files='.length)));
      continue;
    }
    if (arg.startsWith('--')) continue;
    rootArg ??= arg;
  }

  return { files: sawFiles ? files : null, root: resolve(rootArg ?? '.') };
}

function parseFiles(value: string): string[] {
  try {
    const parsed = JSON.parse(value);
    if (Array.isArray(parsed)) return parsed.filter((item): item is string => typeof item === 'string');
  } catch {
    // Fall through to comma-separated parsing.
  }
  return value
    .split(',')
    .map((part) => part.trim())
    .filter(Boolean);
}

function countLines(path: string): number | undefined {
  try {
    const content = readFileSync(path, 'utf8');
    return content.length === 0 ? 0 : content.split(/\r?\n/).length;
  } catch {
    return undefined;
  }
}

function main(): void {
  const { files, root } = parseArgs();
  const issues: Issue[] = [];

  for (const file of files ?? []) {
    const absolutePath = isAbsolute(file) ? file : join(root, file);
    if (!existsSync(absolutePath)) continue;

    const lineCount = countLines(absolutePath);
    if (lineCount === undefined || lineCount <= maxLines) continue;

    const relativePath = relative(root, absolutePath);
    issues.push({
      file: relativePath,
      line: maxLines + 1,
      message: `File has ${lineCount} lines, exceeds maximum of ${maxLines} lines`,
    });
  }

  console.log(JSON.stringify({ issues }));
}

main();
