import { PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';
import * as vscode from 'vscode';
import packageJson from '../../../package.json';
import { IS_DEV } from '../constants';
import { DEV_SUFFIX } from '../scripts-constants';
import { StoreKey, extensionStore } from '../state/extension-store';
import { WorkspaceStateKey, getWorkspaceState, setWorkspaceState } from '../state/workspace-state';
import { logger } from './logger';
import { getVersionWarningMessage, shouldShowVersionWarning } from './version-compatibility';

function getExtensionVersion(): string {
  return packageJson.version;
}

export function getExtensionVersionLabel(): string {
  return IS_DEV ? DEV_SUFFIX : `v${packageJson.version}`;
}

export function getBinaryVersionLabel(): string {
  if (IS_DEV) {
    return DEV_SUFFIX;
  }
  const binaryVersion = extensionStore.get(StoreKey.BinaryVersion);
  return binaryVersion ? `v${binaryVersion}` : DEV_SUFFIX;
}

export function checkVersionCompatibility(binaryVersion: string | null, context: vscode.ExtensionContext | null): void {
  const extensionVersion = getExtensionVersion();
  const extensionVersionLabel = getExtensionVersionLabel();
  const binaryVersionLabel = binaryVersion ? (IS_DEV ? DEV_SUFFIX : `v${binaryVersion}`) : DEV_SUFFIX;
  logger.info(`Checking version compatibility: extension=${extensionVersionLabel}, binary=${binaryVersionLabel}`);

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

    if (context) {
      const versionKey = `${extensionVersion}-${binaryVersion}`;
      const dismissedWarnings = getWorkspaceState(context, WorkspaceStateKey.DismissedVersionWarnings);

      if (!dismissedWarnings.includes(versionKey)) {
        showVersionWarningNotification(extensionVersion, binaryVersion, context);
      } else {
        logger.info(`Version warning already dismissed for ${versionKey}`);
      }
    } else {
      showVersionWarningNotification(extensionVersion, binaryVersion, context);
    }
  } else {
    logger.info('Version compatibility check passed');
    extensionStore.set(StoreKey.VersionWarning, null);
  }
}

function showVersionWarningNotification(
  extensionVersion: string,
  binaryVersion: string,
  context: vscode.ExtensionContext | null,
): void {
  const message = `${PACKAGE_DISPLAY_NAME}: Extension v${extensionVersion} may not be compatible with binary v${binaryVersion}. Update recommended.`;

  vscode.window.showWarningMessage(message, 'Learn More', 'Dismiss').then((selection) => {
    if (selection === 'Learn More') {
      vscode.env.openExternal(vscode.Uri.parse(`${REPO_URL}/tree/main/packages/vscode-extension#updating`));
    } else if (selection === 'Dismiss' && context) {
      const versionKey = `${extensionVersion}-${binaryVersion}`;
      const dismissedWarnings = getWorkspaceState(context, WorkspaceStateKey.DismissedVersionWarnings);
      const updatedDismissedWarnings = [...dismissedWarnings, versionKey];
      setWorkspaceState(context, WorkspaceStateKey.DismissedVersionWarnings, updatedDismissedWarnings);
      logger.info(`Dismissed version warning for ${versionKey}`);
    }
  });
}
