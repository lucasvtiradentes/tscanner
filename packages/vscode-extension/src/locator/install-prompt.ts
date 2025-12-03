import * as vscode from 'vscode';
import { logger } from '../common/lib/logger';

export async function promptInstall(): Promise<boolean> {
  const choice = await vscode.window.showWarningMessage(
    'TScanner binary not found. Would you like to install it?',
    'Install globally',
    'Install in project',
    'Configure path',
    'Cancel',
  );

  if (choice === 'Install globally') {
    const terminal = vscode.window.createTerminal('TScanner Install');
    terminal.show();
    terminal.sendText('npm install -g tscanner');

    await vscode.window.showInformationMessage(
      'Installing tscanner globally. Please wait for installation to complete, then restart VSCode.',
      'OK',
    );
    return true;
  }

  if (choice === 'Install in project') {
    const terminal = vscode.window.createTerminal('TScanner Install');
    terminal.show();
    terminal.sendText('npm install --save-dev tscanner');

    await vscode.window.showInformationMessage(
      'Installing tscanner in project. Please wait for installation to complete, then restart VSCode.',
      'OK',
    );
    return true;
  }

  if (choice === 'Configure path') {
    await vscode.commands.executeCommand('workbench.action.openSettings', 'tscanner.lsp.bin');
    return false;
  }

  logger.info('User cancelled tscanner installation');
  return false;
}
