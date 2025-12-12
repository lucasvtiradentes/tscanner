import type { IssueRuleType } from './enums';

export type CliOutputIssue = {
  rule: string;
  severity: string;
  line: number;
  column: number;
  message: string;
  line_text?: string;
  rule_type: IssueRuleType;
};

export type CliOutputFileGroup = {
  file: string;
  issues: CliOutputIssue[];
};

export type RulesBreakdown = {
  builtin: number;
  regex: number;
  script: number;
  ai: number;
};

export type CliOutputSummary = {
  total_files: number;
  files_with_issues: number;
  total_issues: number;
  errors: number;
  warnings: number;
  infos: number;
  hints: number;
  triggered_rules: number;
  triggered_rules_breakdown: RulesBreakdown;
  total_enabled_rules: number;
  enabled_rules_breakdown: RulesBreakdown;
  duration_ms: number;
};

export type CliOutputByFile = {
  files: CliOutputFileGroup[];
  summary: CliOutputSummary;
};

export type CliOutputRuleIssue = {
  file: string;
  line: number;
  column: number;
  message: string;
  severity: string;
  line_text?: string;
};

export type CliOutputRuleGroup = {
  rule: string;
  rule_type?: IssueRuleType;
  message?: string;
  count: number;
  issues: CliOutputRuleIssue[];
};

export type CliOutputByRule = {
  rules: CliOutputRuleGroup[];
  summary: CliOutputSummary;
};
