import { StartupScanMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getLocalConfigPath, readLocalConfig, setLocalAiConfig } from '../../common/lib/local-config';
import {
  Command,
  type QuickPickItemWithId,
  registerCommand,
  requireWorkspaceOrNull,
} from '../../common/lib/vscode-utils';
import { ExtensionConfigKey, getExtensionConfig, updateExtensionConfig } from '../../common/state/extension-config';
import type { CommandContext } from '../../common/state/extension-state';
import type { RegularIssuesView } from '../../issues-panel';
import { showAiProviderMenu } from '../../settings-menu/ai-provider';
import { showScanModeMenu } from '../../settings-menu/scan-mode';
import type { SettingsView } from '../../settings-view';
import { aiScanIntervalWatcher, scanIntervalWatcher } from '../../watchers';

const AUTO_SCAN_ENABLED_INTERVAL_SECONDS = 60;
const LAST_AUTO_SCAN_INTERVAL_KEY = 'tscanner.settings.lastAutoScanInterval';
const LAST_AUTO_AI_SCAN_INTERVAL_KEY = 'tscanner.settings.lastAutoAiScanInterval';

type SettingsActionContext = {
  commandContext: CommandContext;
  regularView: RegularIssuesView;
  settingsView: SettingsView;
};

async function pickStartupScan(
  configKey: ExtensionConfigKey.StartupScan | ExtensionConfigKey.StartupAiScan,
): Promise<void> {
  const current = getExtensionConfig(configKey);
  const items: QuickPickItemWithId<StartupScanMode>[] = [
    {
      id: StartupScanMode.Off,
      label: 'Off',
      description: current === StartupScanMode.Off ? 'Active' : '',
    },
    {
      id: StartupScanMode.Cached,
      label: 'Cached',
      description: current === StartupScanMode.Cached ? 'Active' : '',
    },
    {
      id: StartupScanMode.Fresh,
      label: 'Fresh',
      description: current === StartupScanMode.Fresh ? 'Active' : '',
    },
  ];

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: 'Startup scan',
    ignoreFocusOut: false,
  });
  if (!selected) return;

  await updateExtensionConfig(configKey, selected.id);
}

function getEnabledInterval(context: vscode.ExtensionContext, key: string): number {
  const saved = context.workspaceState.get<number>(key) ?? AUTO_SCAN_ENABLED_INTERVAL_SECONDS;
  return saved > 0 ? saved : AUTO_SCAN_ENABLED_INTERVAL_SECONDS;
}

export function createManageAiProviderCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { commandContext, settingsView } = ctx;

  return registerCommand(Command.ManageAiProvider, async () => {
    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    await showAiProviderMenu({
      workspacePath: workspaceFolder.uri.fsPath,
      updateStatusBar: commandContext.updateStatusBar,
    });
    settingsView.refresh();
  });
}

export function createManageAiModelCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { commandContext, settingsView } = ctx;

  return registerCommand(Command.ManageAiModel, async () => {
    const workspaceFolder = requireWorkspaceOrNull();
    if (!workspaceFolder) return;

    const workspacePath = workspaceFolder.uri.fsPath;
    const localConfig = await readLocalConfig(workspacePath);
    if (!localConfig.ai?.provider) {
      await showAiProviderMenu({
        workspacePath,
        updateStatusBar: commandContext.updateStatusBar,
      });
      settingsView.refresh();
      return;
    }

    const model = await vscode.window.showInputBox({
      prompt: 'AI model (optional)',
      value: localConfig.ai.model ?? '',
      ignoreFocusOut: false,
    });
    if (model === undefined) return;

    await setLocalAiConfig(workspacePath, localConfig.ai.provider, model.trim() || undefined);
    await commandContext.updateStatusBar();
    settingsView.refresh();
    vscode.window.setStatusBarMessage(`AI model saved to ${getLocalConfigPath(workspacePath)}`, 3000);
  });
}

export function createManageScanModeCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { commandContext, regularView, settingsView } = ctx;

  return registerCommand(Command.ManageScanMode, async () => {
    await showScanModeMenu({
      updateStatusBar: commandContext.updateStatusBar,
      regularView,
    });
    settingsView.refresh();
  });
}

export function createManageStartupScanCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { settingsView } = ctx;

  return registerCommand(Command.ManageStartupScan, async () => {
    await pickStartupScan(ExtensionConfigKey.StartupScan);
    settingsView.refresh();
  });
}

export function createManageStartupAiScanCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { settingsView } = ctx;

  return registerCommand(Command.ManageStartupAiScan, async () => {
    await pickStartupScan(ExtensionConfigKey.StartupAiScan);
    settingsView.refresh();
  });
}

export function createToggleAutoScanCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { commandContext, settingsView } = ctx;

  return registerCommand(Command.ToggleAutoScan, async () => {
    const current = getExtensionConfig(ExtensionConfigKey.AutoScanInterval);
    if (current > 0) {
      await commandContext.context.workspaceState.update(LAST_AUTO_SCAN_INTERVAL_KEY, current);
    }
    const nextValue = current > 0 ? 0 : getEnabledInterval(commandContext.context, LAST_AUTO_SCAN_INTERVAL_KEY);
    await updateExtensionConfig(ExtensionConfigKey.AutoScanInterval, nextValue);
    scanIntervalWatcher.setup(true);
    await commandContext.updateStatusBar();
    settingsView.refresh();
  });
}

export function createToggleAutoAiScanCommand(ctx: SettingsActionContext): vscode.Disposable {
  const { commandContext, settingsView } = ctx;

  return registerCommand(Command.ToggleAutoAiScan, async () => {
    const current = getExtensionConfig(ExtensionConfigKey.AutoAiScanInterval);
    if (current > 0) {
      await commandContext.context.workspaceState.update(LAST_AUTO_AI_SCAN_INTERVAL_KEY, current);
    }
    const nextValue = current > 0 ? 0 : getEnabledInterval(commandContext.context, LAST_AUTO_AI_SCAN_INTERVAL_KEY);
    await updateExtensionConfig(ExtensionConfigKey.AutoAiScanInterval, nextValue);
    aiScanIntervalWatcher.setup(true);
    await commandContext.updateStatusBar();
    settingsView.refresh();
  });
}
