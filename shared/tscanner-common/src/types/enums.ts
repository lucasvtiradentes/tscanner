import z from 'zod';

export enum Severity {
  Error = 'error',
  Warning = 'warning',
  Info = 'info',
  Hint = 'hint',
}

export enum IssueRuleType {
  Builtin = 'builtin',
  CustomRegex = 'custom_regex',
  CustomScript = 'custom_script',
  Ai = 'ai',
}

export enum AiProvider {
  Claude = 'claude',
  Gemini = 'gemini',
  Custom = 'custom',
}

export enum AiMode {
  Paths = 'paths',
  Content = 'content',
  Agentic = 'agentic',
}

export enum AiExecutionMode {
  Ignore = 'ignore',
  Include = 'include',
  Only = 'only',
}

export enum RuleType {
  Ast = 'ast',
  Regex = 'regex',
}

export enum RuleCategory {
  TypeSafety = 'typesafety',
  CodeQuality = 'codequality',
  Style = 'style',
  Performance = 'performance',
  BugPrevention = 'bugprevention',
  Variables = 'variables',
  Imports = 'imports',
}

export enum ScanMode {
  Codebase = 'codebase',
  Branch = 'branch',
  Staged = 'staged',
  Uncommitted = 'uncommitted',
}

export enum GroupMode {
  File = 'file',
  Rule = 'rule',
}

export enum ViewMode {
  List = 'list',
  Tree = 'tree',
}

export const severitySchema = z.enum(Severity);
export const issueRuleTypeSchema = z.enum(IssueRuleType);
