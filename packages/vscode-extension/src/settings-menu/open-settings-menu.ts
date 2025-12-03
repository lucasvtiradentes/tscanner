import * as vscode from 'vscode';
import { getConfigState } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import {
  Command,
  type QuickPickItemWithId,
  executeCommand,
  registerCommand,
  requireWorkspaceOrNull,
} from '../common/lib/vscode-utils';
import type { CommandContext } from '../common/state/extension-state';
import type { IssuesPanelContent } from '../issues-panel/panel-content';
import { getCurrentLocationLabel, openConfigFile, showConfigLocationMenu } from './config-location';
import { showScanModeMenu } from './scan-mode';

enum SettingsMenuOption {
  ManageRules = 'manage-rules',
  ManageScanMode = 'manage-scan-mode',
  ManageConfigLocation = 'manage-config-location',
  OpenConfigFile = 'open-config-file',
}

export function createOpenSettingsMenuCommand(ctx: CommandContext, panelContent: IssuesPanelContent) {
  const { context, stateRefs, updateBadge, updateStatusBar } = ctx;
  const { currentScanModeRef, currentCompareBranchRef, currentCustomConfigDirRef } = stateRefs;

  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');

    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

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
        {
          id: SettingsMenuOption.OpenConfigFile,
          label: '$(edit) Open Config File',
          detail: 'Edit current config file',
        },
      );
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
        await executeCommand(Command.ManageRules);
        break;
      case SettingsMenuOption.ManageScanMode:
        await showScanModeMenu(updateStatusBar, currentScanModeRef, currentCompareBranchRef, context, panelContent);
        break;
      case SettingsMenuOption.ManageConfigLocation:
        await showConfigLocationMenu(updateStatusBar, updateBadge, currentCustomConfigDirRef, context, panelContent);
        break;
      case SettingsMenuOption.OpenConfigFile:
        await openConfigFile(context, currentCustomConfigDirRef.current);
        break;
    }
  });
}
