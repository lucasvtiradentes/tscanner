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
