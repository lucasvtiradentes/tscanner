import { Command, executeCommand, registerCommand } from '../../common/lib/vscode-utils';

export function createRefreshCommand() {
  return registerCommand(Command.Refresh, async () => {
    await executeCommand(Command.HardScan);
  });
}
