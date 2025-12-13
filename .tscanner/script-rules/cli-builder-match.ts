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

type CliFlag = {
  name: string;
  takesValue: boolean;
};

type CliCommand = {
  name: string;
  flags: CliFlag[];
};

type CliJson = {
  commands: CliCommand[];
};

const FLAG_MAPPINGS: Record<string, string | string[]> = {
  branch: 'branch',
  'config-path': 'configPath',
  'continue-on-error': 'continueOnError',
  'group-by': 'groupBy',
  'json-output': 'jsonOutput',
  'no-cache': 'noCache',
  'include-ai': 'aiMode',
  'only-ai': 'aiMode',
  glob: 'filter',
  rule: 'filter',
};

const IGNORED_FLAGS = ['format', 'staged'];

function kebabToCamel(str: string): string {
  return str.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
}

function extractTypeFields(content: string, typeName: string): string[] {
  const startRegex = new RegExp(`(?:export\\s+)?type\\s+${typeName}\\s*=\\s*\\{`);
  const match = content.match(startRegex);
  if (!match) return [];

  const startIndex = (match.index ?? 0) + match[0].length;
  let depth = 1;
  let endIndex = startIndex;

  for (let i = startIndex; i < content.length && depth > 0; i++) {
    if (content[i] === '{') depth++;
    else if (content[i] === '}') depth--;
    endIndex = i;
  }

  const fieldsStr = content.slice(startIndex, endIndex);
  const lines = fieldsStr.split('\n');
  const fields: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    const fieldMatch = trimmed.match(/^(\w+)\??:/);
    if (fieldMatch) {
      fields.push(fieldMatch[1]);
    }
  }

  return fields;
}

function findLineNumber(content: string, searchStr: string): number {
  const lines = content.split('\n');
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(searchStr)) {
      return i + 1;
    }
  }
  return 1;
}

async function main() {
  let data = '';
  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  const cliJsonFile = input.files.find((f) => f.path.endsWith('cli.json'));
  const builderFile = input.files.find((f) => f.path.endsWith('cli-builder.ts'));

  if (!cliJsonFile || !builderFile) {
    console.log(JSON.stringify({ issues }));
    return;
  }

  let cliJson: CliJson;
  try {
    cliJson = JSON.parse(cliJsonFile.content);
  } catch {
    issues.push({
      file: cliJsonFile.path,
      line: 1,
      message: 'Failed to parse cli.json',
    });
    console.log(JSON.stringify({ issues }));
    return;
  }

  const checkCommand = cliJson.commands.find((c) => c.name === 'check');
  if (!checkCommand) {
    issues.push({
      file: cliJsonFile.path,
      line: 1,
      message: 'No "check" command found in cli.json',
    });
    console.log(JSON.stringify({ issues }));
    return;
  }

  const cliFlags = checkCommand.flags.map((f) => f.name).filter((f) => !IGNORED_FLAGS.includes(f));

  const builderFields = extractTypeFields(builderFile.content, 'CliCheckOptions');
  const builderFieldsSet = new Set(builderFields);

  const coveredFlags = new Set<string>();

  for (const flag of cliFlags) {
    const mapping = FLAG_MAPPINGS[flag];
    const expectedField = mapping ?? kebabToCamel(flag);

    if (Array.isArray(expectedField)) {
      const found = expectedField.some((f) => builderFieldsSet.has(f));
      if (found) {
        coveredFlags.add(flag);
      }
    } else if (builderFieldsSet.has(expectedField)) {
      coveredFlags.add(flag);
    }
  }

  for (const flag of cliFlags) {
    if (!coveredFlags.has(flag)) {
      const expectedField = FLAG_MAPPINGS[flag] ?? kebabToCamel(flag);
      issues.push({
        file: builderFile.path,
        line: findLineNumber(builderFile.content, 'CliCheckOptions'),
        message: `CLI flag "--${flag}" is not covered in CliCheckOptions (expected field: "${expectedField}")`,
      });
    }
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
