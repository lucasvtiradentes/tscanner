import { join, resolve } from 'node:path';
import { DynMarkdown, MarkdownTable, type TRowContent, getJson } from 'markdown-helper';

type VscodeCommand = {
  command: string;
  title?: string;
  icon?: string;
};

type VscodeKeybinding = {
  command: string;
  key: string;
  when?: string;
};

type CommandPaletteEntry = {
  command: string;
  when: string;
};

type VscodePackageJson = {
  contributes: {
    commands: VscodeCommand[];
    keybindings: VscodeKeybinding[];
    menus: {
      commandPalette: CommandPaletteEntry[];
    };
  };
};

type TFields = 'COMMANDS';

const rootDir = resolve(__dirname, '..', '..');

export function updateCommands() {
  const vscodePackageJson: VscodePackageJson = getJson(join(rootDir, 'packages/vscode-extension/package.json'));

  const hiddenCommands = new Set(
    vscodePackageJson.contributes.menus.commandPalette
      .filter((entry) => entry.when === 'false')
      .map((entry) => entry.command),
  );

  const keybindingsMap = new Map(vscodePackageJson.contributes.keybindings.map((kb) => [kb.command, kb.key]));

  const visibleCommands = vscodePackageJson.contributes.commands
    .filter((cmd) => !hiddenCommands.has(cmd.command) && cmd.title && cmd.title.startsWith('tscanner:'))
    .sort((a, b) => (a.title || '').localeCompare(b.title || ''));

  const commandsHeaderContent = [
    { content: 'Command', width: 400 },
    { content: 'Keybinding', width: 100 },
  ] as const satisfies TRowContent;

  const commandsTable = new MarkdownTable(commandsHeaderContent);

  for (const cmd of visibleCommands) {
    const keybinding = keybindingsMap.get(cmd.command);
    commandsTable.addBodyRow([
      { content: `${cmd.title}`, align: 'left' },
      { content: keybinding ? `${keybinding}` : '-', align: 'center' },
    ]);
  }

  const commandsContent = `<div align="center">\n\n${commandsTable.getTable()}\n\n</div>`;

  const vscodeReadme = new DynMarkdown<TFields>(join(rootDir, 'packages/vscode-extension/README.md'));
  vscodeReadme.updateField('COMMANDS', commandsContent);
  vscodeReadme.saveFile();

  console.log(`✓ Updated COMMANDS in vscode readme (${visibleCommands.length} commands)`);
}
