import { isAbsolute, relative, resolve, sep } from 'node:path';
import { stderr, stdout } from 'node:process';

export type ScriptArgs = {
  files: string[] | null;
  options?: unknown;
  workspaceRoot: string;
};

export type ScriptIssue = {
  column?: number;
  file: string;
  line: number;
  message: string;
} & Record<string, unknown>;

export type ScriptOutput = {
  issues: ScriptIssue[];
};

export function addIssue(issues: ScriptIssue[], issue: ScriptIssue): void {
  issues.push(issue);
}

export function readScriptArgs(argv = process.argv.slice(2)): ScriptArgs {
  let rootArg: string | undefined;
  let options: unknown;
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
    if (arg === '--options') {
      options = parseJson(argv[index + 1] ?? '');
      index += 1;
      continue;
    }
    if (arg.startsWith('--options=')) {
      options = parseJson(arg.slice('--options='.length));
      continue;
    }
    if (arg.startsWith('--')) {
      if (!arg.includes('=') && argv[index + 1] && !argv[index + 1].startsWith('--')) {
        index += 1;
      }
      continue;
    }
    rootArg ??= arg;
  }

  const workspaceRoot = resolve(rootArg ?? '.');
  return {
    files: sawFiles ? files.flatMap((file) => normalizeFile(workspaceRoot, file)) : null,
    options,
    workspaceRoot,
  };
}

export function writeScriptOutput(issues: ScriptIssue[]): void {
  stdout.write(`${JSON.stringify({ issues })}\n`);
}

export function runScript(fn: (input: ScriptArgs) => Promise<ScriptIssue[]> | ScriptIssue[]): void {
  Promise.resolve(fn(readScriptArgs()))
    .then(writeScriptOutput)
    .catch((err) => {
      const message = err instanceof Error ? (err.stack ?? err.message) : String(err);
      stderr.write(`${message}\n`);
      process.exit(1);
    });
}

function parseFiles(value: string): string[] {
  const trimmed = value.trim();
  if (!trimmed) return [];

  try {
    const parsed = parseJson(trimmed);
    if (Array.isArray(parsed)) return parsed.filter((item): item is string => typeof item === 'string');
  } catch {
    // Fall through to comma-separated parsing.
  }

  return trimmed
    .split(',')
    .map((part) => part.trim())
    .filter(Boolean);
}

function parseJson(value: string): unknown {
  return JSON.parse(value);
}

function normalizeFile(workspaceRoot: string, file: string): string[] {
  const relativeFile = isAbsolute(file) ? relative(workspaceRoot, file) : file;
  const normalized = relativeFile.split(sep).join('/').replace(/^\.\//, '');
  if (!normalized || normalized === '.' || normalized.startsWith('../')) return [];
  return [normalized];
}
