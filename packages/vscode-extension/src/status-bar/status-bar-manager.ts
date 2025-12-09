import { execSync } from 'node:child_process';
import { ScanMode, type TscannerConfig, hasConfiguredRules } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId, getStatusBarName } from '../common/constants';
import { getConfigDirLabel, loadConfig } from '../common/lib/config-manager';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { Locator, LocatorSource } from '../locator';

type BinaryInfo = {
  source: LocatorSource;
  version: string | null;
};

const SOURCE_LABELS: Record<LocatorSource, string> = {
  [LocatorSource.Dev]: 'dev',
  [LocatorSource.Settings]: 'settings',
  [LocatorSource.NodeModules]: 'local',
  [LocatorSource.Global]: 'global',
  [LocatorSource.Path]: 'PATH',
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
    const output = execSync(`"${binaryPath}" --version`, { encoding: 'utf8', timeout: 5000 });
    const match = output.match(/(\d+\.\d+\.\d+)/);
    return match ? match[1] : null;
  } catch {
    return null;
  }
}

export class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;
  private cachedBinaryInfo: BinaryInfo | null = null;

  constructor(
    private currentScanModeRef: { current: ScanMode },
    private currentCompareBranchRef: { current: string },
    private currentConfigDirRef: { current: string | null },
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

    const configDir = this.currentConfigDirRef.current;
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
    const icon = '$(shield)';
    const modeText =
      this.currentScanModeRef.current === ScanMode.Codebase
        ? 'Codebase'
        : `Branch (${this.currentCompareBranchRef.current})`;
    const finalText = `${icon} ${modeText}`;

    this.statusBarItem.text = finalText;

    const displayName = getStatusBarName();
    const configLabel = getConfigDirLabel(configDir);
    const binaryLabel = binaryInfo.version
      ? `${SOURCE_LABELS[binaryInfo.source]} (v${binaryInfo.version})`
      : SOURCE_LABELS[binaryInfo.source];
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
    const finalText = '$(warning) [No rules]';

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
