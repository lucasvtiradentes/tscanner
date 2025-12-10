import z from 'zod';
import constants from '../../../assets/constants.json';

export const PACKAGE_NAME = constants.packageName;
export const PACKAGE_DISPLAY_NAME = constants.packageDisplayName;
export const PACKAGE_DESCRIPTION = constants.packageDescription;
export const CONFIG_DIR_NAME = constants.configDirName;
export const CONFIG_FILE_NAME = constants.configFileName;
export const DEFAULT_TARGET_BRANCH = constants.defaultTargetBranch;
export const LOG_BASENAME = constants.logBasename;
export const LOG_TIMEZONE_OFFSET_HOURS = constants.logTimezoneOffsetHours;
export const IGNORE_COMMENT = constants.ignoreComment;
export const IGNORE_NEXT_LINE_COMMENT = constants.ignoreNextLineComment;
export const JS_EXTENSIONS = constants.extensions.javascript;

export enum Severity {
  Error = 'error',
  Warning = 'warning',
  Info = 'info',
  Hint = 'hint',
}

export enum ScanMode {
  Codebase = 'codebase',
  Branch = 'branch',
}

export enum GroupMode {
  File = 'file',
  Rule = 'rule',
}

export enum ViewMode {
  List = 'list',
  Tree = 'tree',
}

export enum AiProvider {
  Claude = 'claude',
  Gemini = 'gemini',
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
}

export enum IssueRuleType {
  Builtin = 'builtin',
  CustomRegex = 'custom_regex',
  CustomScript = 'custom_script',
  Ai = 'ai',
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

export const severitySchema = z.enum(Severity);
export const issueRuleTypeSchema = z.enum(IssueRuleType);
