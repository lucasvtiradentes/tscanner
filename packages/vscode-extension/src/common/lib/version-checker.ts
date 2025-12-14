import { PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';
import * as vscode from 'vscode';
import packageJson from '../../../package.json';
import { StoreKey, extensionStore } from '../state/extension-store';
import { logger } from './logger';
import { getVersionWarningMessage, shouldShowVersionWarning } from './version-compatibility';

export function getExtensionVersion(): string {
  return packageJson.version;
}

export function checkVersionCompatibility(binaryVersion: string | null): void {
  const extensionVersion = getExtensionVersion();
  logger.info(`Checking version compatibility: extension=${extensionVersion}, binary=${binaryVersion}`);

  extensionStore.set(StoreKey.BinaryVersion, binaryVersion);

  if (!binaryVersion) {
    logger.warn('Binary version not available, skipping compatibility check');
    extensionStore.set(StoreKey.VersionWarning, null);
    return;
  }

  const showWarning = shouldShowVersionWarning(extensionVersion, binaryVersion);
  const warningMessage = getVersionWarningMessage(extensionVersion, binaryVersion);

  if (showWarning && warningMessage) {
    logger.warn(`Version compatibility warning: ${warningMessage}`);
    extensionStore.set(StoreKey.VersionWarning, warningMessage);
    showVersionWarningNotification(extensionVersion, binaryVersion);
  } else {
    logger.info('Version compatibility check passed');
    extensionStore.set(StoreKey.VersionWarning, null);
  }
}

function showVersionWarningNotification(extensionVersion: string, binaryVersion: string): void {
  const message = `${PACKAGE_DISPLAY_NAME}: Extension v${extensionVersion} may not be compatible with binary v${binaryVersion}. Update recommended.`;

  vscode.window.showWarningMessage(message, 'Update CLI', 'Learn More', 'Dismiss').then((selection) => {
    if (selection === 'Update CLI') {
      vscode.env.openExternal(vscode.Uri.parse(`${REPO_URL}#-installation`));
    } else if (selection === 'Learn More') {
      vscode.env.openExternal(vscode.Uri.parse(`${REPO_URL}#version-compatibility`));
    }
  });
}
