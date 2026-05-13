import * as vscode from 'vscode';
import { getSettingsViewId } from '../../common/constants';
import { Command, registerCommand } from '../../common/lib/vscode-utils';

export function createOpenSettingsViewCommand(): vscode.Disposable {
  return registerCommand(Command.OpenSettingsView, async () => {
    await vscode.commands.executeCommand(`${getSettingsViewId()}.focus`);
  });
}
