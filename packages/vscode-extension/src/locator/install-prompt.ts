import { EXTENSION_DISPLAY_NAME } from 'src/common/scripts-constants';
import { REPO_URL } from 'tscanner-common';
import * as vscode from 'vscode';

const QUICK_START_URL = `${REPO_URL}?tab=readme-ov-file#-quick-start`;

export async function promptInstall(): Promise<boolean> {
  const choice = await vscode.window.showWarningMessage(
    `${EXTENSION_DISPLAY_NAME} binary not found. Please install tscanner CLI to use this extension.`,
    'Learn more',
  );

  if (choice === 'Learn more') {
    vscode.env.openExternal(vscode.Uri.parse(QUICK_START_URL));
  }

  return false;
}
