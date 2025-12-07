import type { GroupMode, ScanContentParams, ScanFileParams, ScanParams, ScanResult } from 'tscanner-common';

export type { ScanContentParams, ScanFileParams, ScanParams };

export type FormatResultsParams = {
  root: string;
  results: ScanResult;
  group_mode: GroupMode;
};

export type FormatPrettyResult = {
  output: string;
  summary: {
    total_issues: number;
    error_count: number;
    warning_count: number;
    file_count: number;
    rule_count: number;
  };
};

export type ClearCacheResult = {
  cleared: boolean;
};

export type AiRuleStatus =
  | { pending: Record<string, never> }
  | { running: Record<string, never> }
  | { completed: { issues_found: number } }
  | { failed: { error: string } };

export type AiProgressParams = {
  rule_name: string;
  rule_index: number;
  total_rules: number;
  status: AiRuleStatus;
};
