import { stdin } from 'node:process';

export type ScriptFile = {
  path: string;
  content: string;
  lines: string[];
};

export type ScriptInput<TOptions = Record<string, unknown>> = {
  files: ScriptFile[];
  options?: TOptions;
  workspaceRoot: string;
};

export type ScriptIssue = {
  file: string;
  line: number;
  column?: number;
  message: string;
};

export type ScriptOutput = {
  issues: ScriptIssue[];
};

export function addIssue(issues: ScriptIssue[], issue: Omit<ScriptIssue, 'column'> & { column?: number }): void {
  issues.push(issue);
}

export async function readScriptInput<TOptions = Record<string, unknown>>(): Promise<ScriptInput<TOptions>> {
  let data = '';
  for await (const chunk of stdin) {
    data += chunk;
  }
  return JSON.parse(data);
}

export function writeScriptOutput(issues: ScriptIssue[]): void {
  console.log(JSON.stringify({ issues }));
}

export function runScript<TOptions = Record<string, unknown>>(
  fn: (input: ScriptInput<TOptions>) => Promise<ScriptIssue[]> | ScriptIssue[],
): void {
  readScriptInput<TOptions>()
    .then((input) => Promise.resolve(fn(input)))
    .then(writeScriptOutput)
    .catch((err) => {
      console.error(err);
      process.exit(1);
    });
}
