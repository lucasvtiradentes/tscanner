import { ScanMode, type TscannerConfig, VSCODE_EXTENSION, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId } from '../common/constants';
import { getCachedConfig, getOrLoadConfig } from '../common/lib/config-manager';
import { getBinaryVersionLabel } from '../common/lib/version-checker';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { type BinaryInfo, loadBinaryInfo } from '../locator';
import { buildConfiguredTooltip, getSchemaVersionWarning } from './status-bar-tooltip';

export class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;
  private cachedBinaryInfo: BinaryInfo | null = null;
  private isSearching = false;
  private isAiSearching = false;

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

    extensionStore.subscribe(StoreKey.CachedConfig, () => {
      this.updateDisplay();
    });

    extensionStore.subscribe(StoreKey.VersionWarning, () => {
      this.updateDisplay();
    });

    extensionStore.subscribe(StoreKey.InvalidConfigFields, () => {
      this.updateDisplay();
    });

    extensionStore.subscribe(StoreKey.ConfigError, () => {
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

    await getOrLoadConfig(workspaceFolder.uri.fsPath);

    if (!this.cachedBinaryInfo) {
      this.cachedBinaryInfo = await loadBinaryInfo(workspaceFolder.uri.fsPath);
    }

    this.updateDisplay();
    this.statusBarItem.show();
  }

  private updateDisplay(): void {
    const configError = extensionStore.get(StoreKey.ConfigError);

    if (configError) {
      this.showConfigError(configError);
      return;
    }

    const config = getCachedConfig();
    const hasConfig = hasConfiguredRules(config);

    if (hasConfig && this.cachedBinaryInfo) {
      this.showConfigured(config, this.cachedBinaryInfo);
    } else {
      this.showUnconfigured();
    }
  }

  private showConfigured(config: TscannerConfig | null, binaryInfo: BinaryInfo): void {
    const configDir = extensionStore.get(StoreKey.ConfigDir);
    const versionWarning = extensionStore.get(StoreKey.VersionWarning);
    const invalidConfigFields = extensionStore.get(StoreKey.InvalidConfigFields);
    const schemaWarning = getSchemaVersionWarning(config, getBinaryVersionLabel());
    const hasWarning = !!versionWarning || !!schemaWarning || invalidConfigFields.length > 0;

    const getIcon = () => {
      if (this.isScanning) return VSCODE_EXTENSION.statusBar.icons.scanning;
      if (hasWarning) return '$(warning)';
      return VSCODE_EXTENSION.statusBar.icons.configured;
    };

    const icon = getIcon();
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);
    const getModeText = () => {
      if (scanMode === ScanMode.Codebase) return 'Codebase';
      if (scanMode === ScanMode.Uncommitted) return 'Uncommitted';
      return `Branch (${compareBranch})`;
    };
    const modeText = getModeText();
    const statusText = this.isScanning ? 'Scanning...' : modeText;
    const finalText = `${icon} ${statusText}`;

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = buildConfiguredTooltip(
      configDir,
      config,
      binaryInfo,
      versionWarning,
      invalidConfigFields,
    );
  }

  private showUnconfigured(): void {
    const finalText = `${VSCODE_EXTENSION.statusBar.icons.unconfigured} [No rules]`;

    this.statusBarItem.text = finalText;

    const tooltipLines = ['No rules configured.', 'Run "tscanner init" to create config.'];

    this.statusBarItem.tooltip = tooltipLines.join('\n');
  }

  private showConfigError(error: string): void {
    const finalText = '$(error) Config Error';

    this.statusBarItem.text = finalText;
    this.statusBarItem.tooltip = `Configuration error: ${error}\n\nFix the config to restore functionality.`;
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
