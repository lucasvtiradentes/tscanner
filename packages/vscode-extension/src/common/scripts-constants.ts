import {
  CONFIG_DIR_NAME,
  CONFIG_FILE_NAME,
  DEFAULT_TARGET_BRANCH,
  LOG_BASENAME,
  PACKAGE_DISPLAY_NAME,
} from 'tscanner-common';

export { CONFIG_DIR_NAME, CONFIG_FILE_NAME, DEFAULT_TARGET_BRANCH };

export const EXTENSION_PUBLISHER = 'lucasvtiradentes';
export const EXTENSION_NAME = 'tscanner-vscode';
export const EXTENSION_DISPLAY_NAME = PACKAGE_DISPLAY_NAME;

export const CONTEXT_PREFIX = 'tscanner';
export const VIEW_ID = 'tscannerExplorer';
export const DEV_SUFFIX = 'Dev';

export function addDevSuffix(str: string): string {
  return `${str}${DEV_SUFFIX}`;
}

export function addDevLabel(str: string): string {
  return `${str} (${DEV_SUFFIX})`;
}

export function buildLogFilename(isDev: boolean): string {
  return isDev ? `${LOG_BASENAME}-${DEV_SUFFIX.toLowerCase()}.txt` : `${LOG_BASENAME}.txt`;
}
