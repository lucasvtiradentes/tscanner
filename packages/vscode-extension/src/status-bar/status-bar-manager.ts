import { ScanMode, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
import { loadEffectiveConfig } from '../common/lib/config-manager';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';

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

    if (hasConfig) {
      this.showConfigured(customConfigDir);
    } else {
      this.showUnconfigured();
    }

    this.statusBarItem.show();
  }

  private showConfigured(customConfigDir: string | null): void {
    const icon = '$(shield)';
    const modeText =
      this.currentScanModeRef.current === ScanMode.Codebase
        ? 'Codebase'
        : `Branch (${this.currentCompareBranchRef.current})`;
    const finalText = `${icon} ${modeText}`;

    this.statusBarItem.text = finalText;

    const displayName = getStatusBarName();
    this.statusBarItem.tooltip = `${displayName} - Click to change settings${customConfigDir ? `\nConfig: ${customConfigDir}` : ''}`;
  }

  private showUnconfigured(): void {
    const finalText = '$(warning) [No rules]';

    this.statusBarItem.text = finalText;

    const displayName = getStatusBarName();
    this.statusBarItem.tooltip = `${displayName} - No rules configured. Click to set up.`;
  }

  getDisposable(): vscode.Disposable {
    return this.statusBarItem;
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
