export const DEFAULT_PAGE_SIZE = 20;

export const PACKAGE_NAME = 'tscanner';
export const CONFIG_DIR_NAME = '.tscanner';
export const CONFIG_FILE_NAME = 'config.jsonc';
export const DEFAULT_TARGET_BRANCH = 'origin/main';

export enum Severity {
  Error = 'error',
  Warning = 'warning',
}

export enum ScanMode {
  Codebase = 'codebase',
  Branch = 'branch',
}

export enum GroupMode {
  Rule = 'rule',
  File = 'file',
}

export enum CustomRuleType {
  Regex = 'regex',
  Script = 'script',
  Ai = 'ai',
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
}

export const PLATFORM_TARGET_MAP: Record<string, string> = {
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

export const PLATFORM_PACKAGE_MAP: Record<string, string> = {
  'linux-x64': '@tscanner/cli-linux-x64',
  'linux-arm64': '@tscanner/cli-linux-arm64',
  'darwin-x64': '@tscanner/cli-darwin-x64',
  'darwin-arm64': '@tscanner/cli-darwin-arm64',
  'win32-x64': '@tscanner/cli-win32-x64',
};
