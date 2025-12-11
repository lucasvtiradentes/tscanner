import { execSync } from 'node:child_process';
import { ScanMode, type TscannerConfig, VSCODE_EXTENSION, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
import { getConfigDirLabel, loadConfig } from '../common/lib/config-manager';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { StoreKey, extensionStore } from '../common/state/extension-store';
import { LOCATOR_SOURCE_LABELS, Locator, LocatorSource } from '../locator';

type BinaryInfo = {
  source: LocatorSource;
  version: string | null;
};

function getAiProviderLabel(config: TscannerConfig | null): string {
  if (!config?.ai?.provider) {
    return 'None';
  }
  const provider = config.ai.provider;
  return provider.charAt(0).toUpperCase() + provider.slice(1);
}

function getBinaryVersion(binaryPath: string): string | null {
  try {
    const output = execSync(`"${binaryPath}" --version`, {
      encoding: 'utf8',
      timeout: VSCODE_EXTENSION.timeouts.versionCheckSeconds * 1000,
    });
    const match = output.match(/(\d+\.\d+\.\d+)/);
    return match ? match[1] : null;
  } catch {
    return null;
  }
}

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
      this.cachedBinaryInfo = await this.loadBinaryInfo(workspaceFolder.uri.fsPath);
    }

    if (hasConfig) {
      this.showConfigured(configDir, config, this.cachedBinaryInfo);
    } else {
      this.showUnconfigured();
    }

    this.statusBarItem.show();
  }

  private async loadBinaryInfo(workspacePath: string): Promise<BinaryInfo> {
    const locator = new Locator(workspacePath);
    const result = await locator.locate();

    if (!result) {
      return { source: LocatorSource.Global, version: null };
    }

    const version = getBinaryVersion(result.path);
    return { source: result.source, version };
  }

  private showConfigured(configDir: string | null, config: TscannerConfig | null, binaryInfo: BinaryInfo): void {
    const icon = VSCODE_EXTENSION.statusBar.icons.configured;
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);
    const modeText = scanMode === ScanMode.Codebase ? 'Codebase' : `Branch (${compareBranch})`;
    const finalText = `${icon} ${modeText}`;

    this.statusBarItem.text = finalText;

    const displayName = getStatusBarName();
    const configLabel = getConfigDirLabel(configDir);
    const binaryLabel = binaryInfo.version
      ? `${LOCATOR_SOURCE_LABELS[binaryInfo.source]} (v${binaryInfo.version})`
      : LOCATOR_SOURCE_LABELS[binaryInfo.source];
    const aiProviderLabel = getAiProviderLabel(config);

    const tooltipLines = [
      displayName,
      '',
      `Config: ${configLabel}`,
      `Binary: ${binaryLabel}`,
      `AI Provider: ${aiProviderLabel}`,
    ];

    this.statusBarItem.tooltip = tooltipLines.join('\n');
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
