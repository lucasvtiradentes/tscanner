import { LOG_BASENAME, PACKAGE_DISPLAY_NAME } from 'tscanner-common';

export const EXTENSION_PUBLISHER = 'lucasvtiradentes';
export const EXTENSION_NAME = 'tscanner-vscode';
export const EXTENSION_DISPLAY_NAME = PACKAGE_DISPLAY_NAME;

export const CONTEXT_PREFIX = 'tscanner';
export const CONFIG_SECTION = 'tscanner';
export const VIEW_ID = 'tscannerExplorer';
export const AI_VIEW_ID = 'tscannerAiExplorer';
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

export function buildConfigSection(isDev: boolean): string {
  return isDev ? addDevSuffix(CONFIG_SECTION) : CONFIG_SECTION;
}

export function buildFullConfigKey(isDev: boolean, key: string): string {
  return `${buildConfigSection(isDev)}.${key}`;
}
