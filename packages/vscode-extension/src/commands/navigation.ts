import * as vscode from 'vscode';
import { Command, openTextDocument, registerCommand } from '../common/lib/vscode-utils';

export function createOpenFileCommand() {
  return registerCommand(Command.OpenFile, (uri: vscode.Uri, line: number, column: number) => {
    openTextDocument(uri).then((doc) => {
      vscode.window.showTextDocument(doc).then((editor) => {
        const position = new vscode.Position(line, column);
        editor.selection = new vscode.Selection(position, position);
        editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
      });
    });
  });
}
