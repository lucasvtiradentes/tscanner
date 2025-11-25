import * as vscode from 'vscode';
import { getCommandId } from '../common/constants';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import { Command, ScanMode, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { hasConfiguredRules } from '../common/types';
import { logger } from '../common/utils/logger';

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

    const icon = hasConfig ? '$(shield)' : '$(warning)';
    const modeText =
      this.currentScanModeRef.current === ScanMode.Codebase
        ? 'Codebase'
        : `Branch (${this.currentCompareBranchRef.current})`;
    const configWarning = hasConfig ? '' : ' [No rules]';

    const finalText = `${icon} ${modeText}${configWarning}`;
    logger.info(`Status bar text updated to: "${finalText}"`);

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = hasConfig
      ? `TScanner - Click to change settings${customConfigDir ? `\nConfig: ${customConfigDir}` : ''}`
      : 'TScanner - No rules configured. Click to set up.';

    this.statusBarItem.show();
  }

  getDisposable(): vscode.Disposable {
    return this.statusBarItem;
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
