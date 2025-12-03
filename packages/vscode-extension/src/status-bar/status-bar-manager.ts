import { ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import { logger } from '../common/lib/logger';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { hasConfiguredRules } from '../common/types';

export class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;

  constructor(
    private context: vscode.ExtensionContext,
    private currentScanModeRef: { current: ScanMode },
    private currentCompareBranchRef: { current: string },
    private currentCustomConfigDirRef: { current: string | null },
  ) {
    this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.statusBarItem.command = getCommandId(Command.OpenSettingsMenu);
  }

  async update(): Promise<void> {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      this.statusBarItem.hide();
      return;
    }

    const customConfigDir = this.currentCustomConfigDirRef.current;
    const config = await loadEffectiveConfig(this.context, workspaceFolder.uri.fsPath, customConfigDir);
    const hasConfig = hasConfiguredRules(config);

    let finalText: string;
    if (hasConfig) {
      const icon = '$(shield)';
      const modeText =
        this.currentScanModeRef.current === ScanMode.Codebase
          ? 'Codebase'
          : `Branch (${this.currentCompareBranchRef.current})`;
      finalText = `${icon} ${modeText}`;
    } else {
      finalText = '$(warning) [No rules]';
    }
    logger.info(`Status bar text updated to: "${finalText}"`);

    this.statusBarItem.text = finalText;
    const displayName = getStatusBarName();
    this.statusBarItem.tooltip = hasConfig
      ? `${displayName} - Click to change settings${customConfigDir ? `\nConfig: ${customConfigDir}` : ''}`
      : `${displayName} - No rules configured. Click to set up.`;

    this.statusBarItem.show();
  }

  getDisposable(): vscode.Disposable {
    return this.statusBarItem;
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
