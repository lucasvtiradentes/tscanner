import * as vscode from 'vscode';
import { getConfigState } from '../common/lib/config-manager';
import {
  Command,
  type QuickPickItemWithId,
  type ScanMode,
  ToastKind,
  executeCommand,
  getCurrentWorkspaceFolder,
  registerCommand,
  showToastMessage,
} from '../common/lib/vscode-utils';
import { logger } from '../common/utils/logger';
import type { IssuesPanelContent } from '../issues-panel/panel-content';
import { getCurrentLocationLabel, openConfigFile, showConfigLocationMenu } from './config-location';
import { showScanModeMenu } from './scan-mode';

enum SettingsMenuOption {
  ManageRules = 'manage-rules',
  ManageScanMode = 'manage-scan-mode',
  ManageConfigLocation = 'manage-config-location',
  OpenConfigFile = 'open-config-file',
}

export function createOpenSettingsMenuCommand(
  updateStatusBar: () => Promise<void>,
  updateBadge: () => void,
  currentScanModeRef: { current: ScanMode },
  currentCompareBranchRef: { current: string },
  currentCustomConfigDirRef: { current: string | null },
  context: vscode.ExtensionContext,
  panelContent: IssuesPanelContent,
) {
  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      showToastMessage(ToastKind.Error, 'No workspace folder open');
      return;
    }

    const workspacePath = workspaceFolder.uri.fsPath;
    const customConfigDir = currentCustomConfigDirRef.current;
    const configState = await getConfigState(context, workspacePath, customConfigDir);
    const currentLocationLabel = getCurrentLocationLabel(
      configState.hasCustom,
      configState.hasLocal,
      configState.hasGlobal,
      customConfigDir,
    );

    const mainMenuItems: QuickPickItemWithId<SettingsMenuOption>[] = [
      {
        id: SettingsMenuOption.ManageRules,
        label: '$(checklist) Manage Rules',
        detail: 'Enable/disable rules',
      },
    ];

    if (configState.hasAny) {
      mainMenuItems.push({
        id: SettingsMenuOption.ManageScanMode,
        label: '$(gear) Manage Scan Mode',
        detail: 'Choose between Codebase or Branch scan mode',
      });
    }

    mainMenuItems.push({
      id: SettingsMenuOption.ManageConfigLocation,
      label: '$(folder) Manage Config Location',
      detail: currentLocationLabel,
    });

    if (configState.hasAny) {
      mainMenuItems.push({
        id: SettingsMenuOption.OpenConfigFile,
        label: '$(edit) Open Config File',
        detail: 'Edit current config file',
      });
    }

    const selected = await vscode.window.showQuickPick(mainMenuItems, {
      placeHolder: 'TScanner Settings',
      ignoreFocusOut: false,
    });

    if (!selected) {
      return;
    }

    switch (selected.id) {
      case SettingsMenuOption.ManageRules:
        logger.info('User selected: Manage Rules');
        await executeCommand(Command.ManageRules);
        break;
      case SettingsMenuOption.ManageScanMode:
        logger.info('User selected: Manage Scan Mode');
        await showScanModeMenu(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, panelContent);
        break;
      case SettingsMenuOption.ManageConfigLocation:
        logger.info('User selected: Manage Config Location');
        await showConfigLocationMenu(updateStatusBar, updateBadge, currentCustomConfigDirRef, context, panelContent);
        break;
      case SettingsMenuOption.OpenConfigFile:
        logger.info('User selected: Open Config File');
        await openConfigFile(context, currentCustomConfigDirRef.current);
        break;
    }
  });
}
