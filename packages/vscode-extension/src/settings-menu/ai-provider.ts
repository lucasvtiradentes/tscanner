import { AiProvider } from 'tscanner-common';
import * as vscode from 'vscode';
import { getLocalConfigPath, readLocalConfig, setLocalAiConfig, unsetLocalAiConfig } from '../common/lib/local-config';
import { type QuickPickItemWithId, ToastKind, showToastMessage } from '../common/lib/vscode-utils';

type ShowAiProviderMenuParams = {
  workspacePath: string;
  updateStatusBar: () => Promise<void>;
};

enum AiProviderMenuOption {
  Claude = 'claude',
  Codex = 'codex',
  Gemini = 'gemini',
  Unset = 'unset',
}

const PROVIDER_BY_OPTION: Record<Exclude<AiProviderMenuOption, AiProviderMenuOption.Unset>, AiProvider> = {
  [AiProviderMenuOption.Claude]: AiProvider.Claude,
  [AiProviderMenuOption.Codex]: AiProvider.Codex,
  [AiProviderMenuOption.Gemini]: AiProvider.Gemini,
};

export async function showAiProviderMenu(params: ShowAiProviderMenuParams): Promise<void> {
  const { workspacePath, updateStatusBar } = params;
  const localConfig = await readLocalConfig(workspacePath);
  const current = localConfig.ai?.provider
    ? `${localConfig.ai.provider}${localConfig.ai.model ? ` (${localConfig.ai.model})` : ''}`
    : 'unset';

  const items: QuickPickItemWithId<AiProviderMenuOption>[] = [
    { id: AiProviderMenuOption.Claude, label: 'Claude', detail: 'Use claude CLI for AI rules' },
    { id: AiProviderMenuOption.Codex, label: 'Codex', detail: 'Use codex CLI for AI rules' },
    { id: AiProviderMenuOption.Gemini, label: 'Gemini', detail: 'Use gemini CLI for AI rules' },
    { id: AiProviderMenuOption.Unset, label: 'Unset', detail: 'Remove project-local AI provider' },
  ];

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: `AI provider (${current})`,
    ignoreFocusOut: false,
  });
  if (!selected) return;

  if (selected.id === AiProviderMenuOption.Unset) {
    await unsetLocalAiConfig(workspacePath);
    await updateStatusBar();
    showToastMessage(ToastKind.Info, `AI provider unset in ${getLocalConfigPath(workspacePath)}`);
    return;
  }

  const model = await vscode.window.showInputBox({
    prompt: 'AI model (optional)',
    value: localConfig.ai?.provider === PROVIDER_BY_OPTION[selected.id] ? (localConfig.ai.model ?? '') : '',
    ignoreFocusOut: false,
  });
  if (model === undefined) return;

  await setLocalAiConfig(workspacePath, PROVIDER_BY_OPTION[selected.id], model.trim() || undefined);
  await updateStatusBar();
  showToastMessage(ToastKind.Info, `AI provider saved to ${getLocalConfigPath(workspacePath)}`);
}
