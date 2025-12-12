import { ScanMode, type TscannerConfig, VSCODE_EXTENSION, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
import { loadConfig } from '../common/lib/config-manager';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { type BinaryInfo, loadBinaryInfo } from '../locator';
import { buildConfiguredTooltip } from './status-bar-tooltip';

export class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;
  private cachedBinaryInfo: BinaryInfo | null = null;

  constructor() {
    this.statusBarItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Left,
      VSCODE_EXTENSION.statusBar.priority,
    );
    this.statusBarItem.command = getCommandId(Command.OpenSettingsMenu);
  }

  async update(): Promise<void> {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      this.statusBarItem.hide();
      return;
    }

    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const config = await loadConfig(workspaceFolder.uri.fsPath, configDir);
    const hasConfig = hasConfiguredRules(config);

    if (!this.cachedBinaryInfo) {
      this.cachedBinaryInfo = await loadBinaryInfo(workspaceFolder.uri.fsPath);
    }

    if (hasConfig) {
      this.showConfigured(configDir, config, this.cachedBinaryInfo);
    } else {
      this.showUnconfigured();
    }

    this.statusBarItem.show();
  }

  private showConfigured(configDir: string | null, config: TscannerConfig | null, binaryInfo: BinaryInfo): void {
    const icon = VSCODE_EXTENSION.statusBar.icons.configured;
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);
    const modeText = scanMode === ScanMode.Codebase ? 'Codebase' : `Branch (${compareBranch})`;
    const finalText = `${icon} ${modeText}`;

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = buildConfiguredTooltip(configDir, config, binaryInfo);
  }

  private showUnconfigured(): void {
    const finalText = `${VSCODE_EXTENSION.statusBar.icons.unconfigured} [No rules]`;

    this.statusBarItem.text = finalText;

    const displayName = getStatusBarName();
    const tooltipLines = [displayName, '', 'No rules configured.', 'Run "tscanner init" to create config.'];

    this.statusBarItem.tooltip = tooltipLines.join('\n');
  }

  clearBinaryCache(): void {
    this.cachedBinaryInfo = null;
  }

  getDisposable(): vscode.Disposable {
    return this.statusBarItem;
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
