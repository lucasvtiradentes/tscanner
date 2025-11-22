import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
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
  ) {
    this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.statusBarItem.command = getCommandId(Command.OpenSettingsMenu);
  }

  async update(): Promise<void> {
    logger.debug('StatusBarManager.update() called');
    const workspaceFolder = getCurrentWorkspaceFolder();

    if (!workspaceFolder) {
      logger.debug('No workspace folder, hiding status bar');
      this.statusBarItem.hide();
      return;
    }

    logger.debug(`Current scan mode ref: ${this.currentScanModeRef.current}`);
    logger.debug(`Current compare branch ref: ${this.currentCompareBranchRef.current}`);

    const config = await loadEffectiveConfig(this.context, workspaceFolder.uri.fsPath);
    const hasConfig = hasConfiguredRules(config);

    const icon = hasConfig ? '$(gear)' : '$(warning)';
    const modeText = this.currentScanModeRef.current === ScanMode.Codebase ? 'Codebase' : 'Branch';
    const branchText =
      this.currentScanModeRef.current === ScanMode.Branch ? ` (${this.currentCompareBranchRef.current})` : '';
    const configWarning = hasConfig ? '' : ' [No rules configured]';

    const finalText = `${icon} ${getStatusBarName()}: ${modeText}${branchText}${configWarning}`;
    logger.info(`Status bar text updated to: "${finalText}"`);

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = hasConfig
      ? 'Click to change scan settings'
      : 'No rules configured. Click to set up rules.';

    this.statusBarItem.show();
    logger.debug('Status bar shown');
  }

  getDisposable(): vscode.Disposable {
    return this.statusBarItem;
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
