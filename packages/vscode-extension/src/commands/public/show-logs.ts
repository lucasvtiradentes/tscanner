import * as vscode from 'vscode';
import { LOG_FILE_PATH } from '../../common/lib/logger';
import { Command, ToastKind, openTextDocument, registerCommand, showToastMessage } from '../../common/lib/vscode-utils';

export function createShowLogsCommand() {
  return registerCommand(Command.ShowLogs, async () => {
    try {
      const doc = await openTextDocument(vscode.Uri.file(LOG_FILE_PATH));
      await vscode.window.showTextDocument(doc, { preview: false });
    } catch (error) {
      showToastMessage(ToastKind.Error, `Failed to open logs: ${error}`);
    }
  });
}
