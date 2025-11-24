export const COMMENT_MARKER = '<!-- tscanner-pr-comment -->';

export enum GroupMode {
  Rule = 'rule',
  File = 'file',
}

export enum ScanMode {
  Codebase = 'codebase',
  Branch = 'branch',
}

export enum Severity {
  Error = 'error',
  Warning = 'warning',
}

export const PACKAGE_NAME = 'tscanner';

export const CONFIG_DIR_NAME = '.tscanner';
export const CONFIG_FILE_NAME = 'config.jsonc';
