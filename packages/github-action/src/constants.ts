export const COMMENT_MARKER = '<!-- tscanner-pr-comment -->';

export enum GroupMode {
  Rule = 'rule',
  File = 'file',
}

export enum Severity {
  Error = 'error',
  Warning = 'warning',
}

export const DEFAULT_TARGET_BRANCH = 'origin/main';
export const DEFAULT_TIMEZONE = 'UTC';
export const DEFAULT_CONFIG_PATH = '.tscanner/rules.json';
export const DEFAULT_TSCANNER_VERSION = 'latest';
export const DEFAULT_GROUP_BY = GroupMode.File;
