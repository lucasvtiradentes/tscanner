import { CONFIG_DIR_NAME, CONFIG_FILE_NAME, PLATFORM_TARGET_MAP, getBinaryName } from 'tscanner-common';
import {
  CONTEXT_PREFIX,
  DEV_SUFFIX,
  EXTENSION_DISPLAY_NAME,
  EXTENSION_NAME,
  EXTENSION_PUBLISHER,
  VIEW_ID,
  addDevLabel,
  buildLogFilename,
} from './scripts-constants';

export { CONFIG_DIR_NAME, CONFIG_FILE_NAME, PLATFORM_TARGET_MAP };

declare const __IS_DEV_BUILD__: boolean;
const IS_DEV = typeof __IS_DEV_BUILD__ !== 'undefined' && __IS_DEV_BUILD__;

export const EXTENSION_ID_PROD = `${EXTENSION_PUBLISHER}.${EXTENSION_NAME}`;
export const EXTENSION_ID_DEV = `${EXTENSION_PUBLISHER}.${EXTENSION_NAME}-dev`;

export function getCommandId(command: string): string {
  return IS_DEV ? `${CONTEXT_PREFIX}${DEV_SUFFIX}.${command}` : `${CONTEXT_PREFIX}.${command}`;
}

export function getContextKey(key: string): string {
  return IS_DEV ? `${key}${DEV_SUFFIX}` : key;
}

export function getViewId(): string {
  return IS_DEV ? `${VIEW_ID}${DEV_SUFFIX}` : VIEW_ID;
}

export function getLogFilename(): string {
  return buildLogFilename(IS_DEV);
}

export function getStatusBarName(): string {
  return IS_DEV ? addDevLabel(EXTENSION_DISPLAY_NAME) : EXTENSION_DISPLAY_NAME;
}

export const BINARY_BASE_NAME = 'tscanner-server';

export { getBinaryName };

export function getServerBinaryName(): string {
  return getBinaryName(BINARY_BASE_NAME);
}
