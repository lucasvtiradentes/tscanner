import { EXTENSION_DISPLAY_NAME } from 'src/common/scripts-constants';
import * as vscode from 'vscode';
import { getConfigState } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, type QuickPickItemWithId, registerCommand, requireWorkspaceOrNull } from '../common/lib/vscode-utils';
import type { CommandContext } from '../common/state/extension-state';
import type { RegularIssuesView } from '../issues-panel';
import { getCurrentLocationLabel, showConfigLocationMenu } from './config-location';
import { showScanModeMenu } from './scan-mode';

enum SettingsMenuOption {
  ManageScanMode = 'manage-scan-mode',
  ManageConfigLocation = 'manage-config-location',
}

export function createOpenSettingsMenuCommand(ctx: CommandContext, regularView: RegularIssuesView) {
  const { context, stateRefs, updateStatusBar } = ctx;
  const { currentScanModeRef, currentCompareBranchRef, currentCustomConfigDirRef } = stateRefs;

  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');

    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    const customConfigDir = currentCustomConfigDirRef.current;
    const configState = await getConfigState(context, workspacePath, customConfigDir);
    const currentLocationLabel = getCurrentLocationLabel(configState.hasCustom, configState.hasLocal, customConfigDir);

    const mainMenuItems: QuickPickItemWithId<SettingsMenuOption>[] = [];

    if (configState.hasAny) {
      mainMenuItems.push(
        {
          id: SettingsMenuOption.ManageScanMode,
          label: '$(gear) Manage Scan Mode',
          detail: 'Choose between Codebase or Branch scan mode',
        },
        {
          id: SettingsMenuOption.ManageConfigLocation,
          label: '$(folder) Manage Config Location',
          detail: currentLocationLabel,
        },
      );
    }

    const selected = await vscode.window.showQuickPick(mainMenuItems, {
      placeHolder: `${EXTENSION_DISPLAY_NAME} Settings`,
      ignoreFocusOut: false,
    });

    if (!selected) {
      return;
    }

    switch (selected.id) {
      case SettingsMenuOption.ManageScanMode:
        await showScanModeMenu(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, regularView);
        break;
      case SettingsMenuOption.ManageConfigLocation:
        await showConfigLocationMenu(updateStatusBar, currentCustomConfigDirRef, context, regularView);
        break;
    }
  });
}
