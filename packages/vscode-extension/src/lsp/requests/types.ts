import type { GroupMode, RuleMetadata, ScanResult, TscannerConfig } from '../../common/types';

export type ScanParams = {
  root: string;
  config?: TscannerConfig;
  branch?: string;
};

export type ScanFileParams = {
  root: string;
  file: string;
};

export type ScanContentParams = {
  root: string;
  file: string;
  content: string;
  config?: TscannerConfig;
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

export type { RuleMetadata };
