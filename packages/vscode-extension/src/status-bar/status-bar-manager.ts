import { ScanMode, type TscannerConfig, VSCODE_EXTENSION, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId } from '../common/constants';
import { loadConfig } from '../common/lib/config-manager';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { type BinaryInfo, loadBinaryInfo } from '../locator';
import { buildConfiguredTooltip } from './status-bar-tooltip';

export class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;
  private cachedBinaryInfo: BinaryInfo | null = null;
  private isSearching = false;
  private isAiSearching = false;
  private cachedConfigDir: string | null = null;
  private cachedConfig: TscannerConfig | null = null;

  constructor() {
    this.statusBarItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Left,
      VSCODE_EXTENSION.statusBar.priority,
    );
    this.statusBarItem.command = getCommandId(Command.OpenSettingsMenu);

    extensionStore.subscribe(StoreKey.IsSearching, (isSearching) => {
      this.isSearching = isSearching;
      this.updateDisplay();
    });

    extensionStore.subscribe(StoreKey.IsAiSearching, (isAiSearching) => {
      this.isAiSearching = isAiSearching;
      this.updateDisplay();
    });
  }

  private get isScanning(): boolean {
    return this.isSearching || this.isAiSearching;
  }

  async update(): Promise<void> {
    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      this.statusBarItem.hide();
      return;
    }

    this.cachedConfigDir = extensionStore.get(StoreKey.ConfigDir);
    this.cachedConfig = await loadConfig(workspaceFolder.uri.fsPath, this.cachedConfigDir);

    if (!this.cachedBinaryInfo) {
      this.cachedBinaryInfo = await loadBinaryInfo(workspaceFolder.uri.fsPath);
    }

    this.updateDisplay();
    this.statusBarItem.show();
  }

  private updateDisplay(): void {
    const hasConfig = hasConfiguredRules(this.cachedConfig);

    if (hasConfig && this.cachedBinaryInfo) {
      this.showConfigured(this.cachedConfigDir, this.cachedConfig, this.cachedBinaryInfo);
    } else {
      this.showUnconfigured();
    }
  }

  private showConfigured(configDir: string | null, config: TscannerConfig | null, binaryInfo: BinaryInfo): void {
    const icon = this.isScanning
      ? VSCODE_EXTENSION.statusBar.icons.scanning
      : VSCODE_EXTENSION.statusBar.icons.configured;
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);
    const modeText = scanMode === ScanMode.Codebase ? 'Codebase' : `Branch (${compareBranch})`;
    const statusText = this.isScanning ? 'Scanning...' : modeText;
    const finalText = `${icon} ${statusText}`;

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = buildConfiguredTooltip(configDir, config, binaryInfo);
  }

  private showUnconfigured(): void {
    const finalText = `${VSCODE_EXTENSION.statusBar.icons.unconfigured} [No rules]`;

    this.statusBarItem.text = finalText;

    const tooltipLines = ['No rules configured.', 'Run "tscanner init" to create config.'];

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
