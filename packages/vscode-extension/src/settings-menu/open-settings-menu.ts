import { EXTENSION_DISPLAY_NAME } from 'src/common/scripts-constants';
import * as vscode from 'vscode';
import { hasConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, type QuickPickItemWithId, registerCommand, requireWorkspaceOrNull } from '../common/lib/vscode-utils';
import type { CommandContext } from '../common/state/extension-state';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import type { RegularIssuesView } from '../issues-panel';
import { showAiProviderMenu } from './ai-provider';
import { getCurrentLocationLabel, showConfigLocationMenu } from './config-location';
import { showScanModeMenu } from './scan-mode';

enum SettingsMenuOption {
  ManageScanMode = 'manage-scan-mode',
  ManageAiProvider = 'manage-ai-provider',
  ManageConfigLocation = 'manage-config-location',
}

export function createOpenSettingsMenuCommand(ctx: CommandContext, regularView: RegularIssuesView) {
  const { updateStatusBar } = ctx;

  return registerCommand(Command.OpenSettingsMenu, async () => {
    logger.info('openSettingsMenu command called');

    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const hasConfigFile = await hasConfig(workspacePath, configDir);
    const currentLocationLabel = getCurrentLocationLabel(configDir, hasConfigFile);

    const mainMenuItems: QuickPickItemWithId<SettingsMenuOption>[] = [];

    if (hasConfigFile) {
      mainMenuItems.push({
        id: SettingsMenuOption.ManageScanMode,
        label: '$(gear) Manage Scan Mode',
        detail: 'Choose which files to scan',
      });
      mainMenuItems.push({
        id: SettingsMenuOption.ManageAiProvider,
        label: '$(sparkle) Manage AI Provider',
        detail: 'Project-local provider in .tscanner/local.jsonc',
      });
    }

    mainMenuItems.push({
      id: SettingsMenuOption.ManageConfigLocation,
      label: '$(folder) Manage Config Location',
      detail: currentLocationLabel,
    });

    const selected = await vscode.window.showQuickPick(mainMenuItems, {
      placeHolder: `${EXTENSION_DISPLAY_NAME} Settings`,
      ignoreFocusOut: false,
    });

    if (!selected) return;

    switch (selected.id) {
      case SettingsMenuOption.ManageScanMode:
        await showScanModeMenu({ updateStatusBar, regularView });
        break;
      case SettingsMenuOption.ManageAiProvider:
        await showAiProviderMenu({ workspacePath, updateStatusBar });
        break;
      case SettingsMenuOption.ManageConfigLocation:
        await showConfigLocationMenu({ updateStatusBar, regularView });
        break;
    }
  });
}
