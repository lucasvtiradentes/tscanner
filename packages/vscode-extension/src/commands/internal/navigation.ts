import type * as vscode from 'vscode';
import { Command, navigateToPosition, registerCommand } from '../../common/lib/vscode-utils';

export function createOpenFileCommand() {
  return registerCommand(Command.OpenFile, (uri: vscode.Uri, line: number, column: number) => {
    navigateToPosition(uri, line, column);
  });
}
