import { ScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCommandId } from '../common/constants';
import { hasConfig } from '../common/lib/config-manager';
import { readLocalConfig } from '../common/lib/local-config';
import { Command, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { ExtensionConfigKey, getExtensionConfig } from '../common/state/extension-config';
import { StoreKey, extensionStore } from '../common/state/extension-store';

type SettingsRow = {
  label: string;
  description?: string;
  icon: string;
  command?: Command;
  tooltip?: string;
  children?: SettingsRow[];
};

class SettingsTreeItem extends vscode.TreeItem {
  constructor(row: SettingsRow) {
    super(row.label, row.children ? vscode.TreeItemCollapsibleState.Expanded : vscode.TreeItemCollapsibleState.None);
    this.description = row.description;
    this.iconPath = new vscode.ThemeIcon(row.icon);
    this.tooltip = row.tooltip;
    if (row.command) {
      this.command = {
        command: getCommandId(row.command),
        title: row.label,
      };
    }
  }
}

function formatInterval(seconds: number): string {
  return seconds > 0 ? `on (${seconds}s)` : 'off';
}

function formatScanMode(scanMode: ScanMode, compareBranch: string): string {
  if (scanMode === ScanMode.Branch) return `${scanMode} (${compareBranch})`;
  return scanMode;
}

function formatValue(value: string): string {
  return value.charAt(0).toUpperCase() + value.slice(1);
}

export class SettingsView implements vscode.TreeDataProvider<SettingsTreeItem> {
  private readonly changeEmitter = new vscode.EventEmitter<SettingsTreeItem | undefined>();
  private readonly childrenByItem = new WeakMap<SettingsTreeItem, SettingsRow[]>();

  readonly onDidChangeTreeData = this.changeEmitter.event;

  refresh(): void {
    this.changeEmitter.fire(undefined);
  }

  getTreeItem(element: SettingsTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: SettingsTreeItem): Promise<SettingsTreeItem[]> {
    if (element) {
      return (this.childrenByItem.get(element) ?? []).map((row) => this.createItem(row));
    }

    const workspaceFolder = getCurrentWorkspaceFolder();
    if (!workspaceFolder) {
      return [
        this.createItem({
          label: 'Open a folder',
          description: 'required',
          icon: 'folder-opened',
        }),
      ];
    }

    const workspacePath = workspaceFolder.uri.fsPath;
    const configExists = await hasConfig(workspacePath);
    if (!configExists) {
      return [
        this.createItem({
          label: 'Project config',
          description: 'missing',
          icon: 'warning',
        }),
      ];
    }

    const localConfig = await readLocalConfig(workspacePath);
    const aiProvider = localConfig.ai?.provider ?? 'unset';
    const aiModel = localConfig.ai?.model ?? 'default';
    const hasAiProvider = !!localConfig.ai?.provider;
    const scanMode = extensionStore.get(StoreKey.ScanMode);
    const compareBranch = extensionStore.get(StoreKey.CompareBranch);

    const rows: SettingsRow[] = [
      {
        label: 'Regular scan',
        icon: 'search',
        children: [
          {
            label: 'Mode',
            description: formatValue(formatScanMode(scanMode, compareBranch)),
            icon: 'list-selection',
            command: Command.ManageScanMode,
          },
          {
            label: 'Startup',
            description: formatValue(getExtensionConfig(ExtensionConfigKey.StartupScan)),
            icon: 'history',
            command: Command.ManageStartupScan,
          },
          {
            label: 'Auto',
            description: formatInterval(getExtensionConfig(ExtensionConfigKey.AutoScanInterval)),
            icon: 'watch',
            command: Command.ToggleAutoScan,
          },
        ],
      },
      {
        label: 'AI scan',
        icon: 'sparkle',
        children: [
          {
            label: 'Provider',
            description: aiProvider,
            icon: 'server-environment',
            command: Command.ManageAiProvider,
          },
          {
            label: 'Model',
            description: aiModel,
            icon: 'symbol-string',
            command: hasAiProvider ? Command.ManageAiModel : undefined,
            tooltip: hasAiProvider ? undefined : 'Set an AI provider before choosing a model.',
          },
          {
            label: 'Startup',
            description: formatValue(getExtensionConfig(ExtensionConfigKey.StartupAiScan)),
            icon: 'history',
            command: Command.ManageStartupAiScan,
          },
          {
            label: 'Auto',
            description: formatInterval(getExtensionConfig(ExtensionConfigKey.AutoAiScanInterval)),
            icon: 'watch',
            command: Command.ToggleAutoAiScan,
          },
        ],
      },
    ];

    return rows.map((row) => this.createItem(row));
  }

  private createItem(row: SettingsRow): SettingsTreeItem {
    const item = new SettingsTreeItem(row);
    if (row.children) {
      this.childrenByItem.set(item, row.children);
    }
    return item;
  }
}
