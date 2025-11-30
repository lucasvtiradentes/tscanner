import path from 'node:path';
import { DynMarkdown, MarkdownTable, type TRowContent, getJson } from 'markdown-helper';

type CliFlag = {
  name: string;
  short: string | null;
  description: string;
  takesValue: boolean;
  valueName: string | null;
  possibleValues: string[] | null;
  defaultValue: string | null;
  required: boolean;
};

type CliArgument = {
  name: string;
  description: string;
  required: boolean;
  defaultValue: string;
};

type CliCommand = {
  name: string;
  description: string;
  usage: string;
  arguments: CliArgument[];
  flags: CliFlag[];
};

type CliJson = {
  name: string;
  version: string;
  description: string;
  commands: CliCommand[];
};

type TFields = 'CLI_USAGE';

const rootDir = path.resolve(__dirname, '..', '..');

export function updateCliUsage() {
  const cliJson: CliJson = getJson(path.join(rootDir, 'assets/generated/cli.json'));

  const headerContent = [
    { content: 'Command', width: 120 },
    { content: 'Description', width: 280 },
    { content: 'Flag', width: 200 },
    { content: 'Flag description', width: 350 },
  ] as const satisfies TRowContent;

  const table = new MarkdownTable(headerContent);

  for (const cmd of cliJson.commands) {
    const args = cmd.arguments.map((a) => `[${a.name}]`).join(' ');
    const hasFlags = cmd.flags.length > 0;
    const cmdName = `<code>${cmd.name}${hasFlags ? ' [options]' : ''}${args ? ` ${args}` : ''}</code>`;

    if (cmd.flags.length === 0) {
      table.addBodyRow([
        { content: cmdName, align: 'left' },
        { content: cmd.description, align: 'left' },
        { content: '-', align: 'center' },
        { content: '-', align: 'center' },
      ]);
    } else {
      for (const flag of cmd.flags) {
        let flagName: string;
        const isBooleanValues =
          flag.possibleValues &&
          flag.possibleValues.length === 2 &&
          flag.possibleValues.includes('true') &&
          flag.possibleValues.includes('false');

        if (flag.possibleValues && flag.possibleValues.length > 0 && !isBooleanValues) {
          flagName = `--${flag.name} [${flag.possibleValues.join('/')}]`;
        } else if (flag.takesValue) {
          flagName = `--${flag.name} <${flag.valueName}>`;
        } else {
          flagName = `--${flag.name}`;
        }
        table.addBodyRow([
          { content: cmdName, align: 'left' },
          { content: cmd.description, align: 'left' },
          { content: `<code>${flagName}</code>`, align: 'left' },
          { content: flag.description, align: 'left' },
        ]);
      }
    }
  }

  const tableContent = table.getTable(['Command', 'Description']);

  const content = `## 📖 Usage<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

<div align="center">

${tableContent}

</div>`;

  const readmePath = path.join(rootDir, 'packages/cli/README.md');
  const readme = new DynMarkdown<TFields>(readmePath);
  readme.updateField('CLI_USAGE', content);
  readme.saveFile();

  console.log(`✓ Updated CLI_USAGE in cli readme (${cliJson.commands.length} commands)`);
}
