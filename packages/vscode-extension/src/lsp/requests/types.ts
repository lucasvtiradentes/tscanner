import type {
  AiExecutionMode,
  GroupMode,
  PreviousAiIssue,
  ScanContentParams,
  ScanFileParams,
  ScanParams,
  ScanResult,
  TscannerConfig,
} from 'tscanner-common';

export type { ScanContentParams, ScanFileParams, ScanParams };

export type ScanRequestOptions = {
  config?: TscannerConfig;
  branch?: string;
  staged?: boolean;
  aiMode?: AiExecutionMode;
  noCache?: boolean;
  previousAiIssues?: PreviousAiIssue[];
};

export type ScanContentRequestOptions = {
  content: string;
  config?: TscannerConfig;
  branch?: string;
  uncommitted?: boolean;
};

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

export type ValidateConfigParams = {
  config_path: string;
};

export type ValidateConfigResult = {
  valid: boolean;
  errors: string[];
  warnings: string[];
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
