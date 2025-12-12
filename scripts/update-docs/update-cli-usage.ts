import { join, resolve } from 'node:path';
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
  group?: string;
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

const rootDir = resolve(__dirname, '..', '..');

function formatFlagName(flag: CliFlag): string {
  const isBooleanValues =
    flag.possibleValues &&
    flag.possibleValues.length === 2 &&
    flag.possibleValues.includes('true') &&
    flag.possibleValues.includes('false');

  if (flag.possibleValues && flag.possibleValues.length > 0 && !isBooleanValues) {
    return `--${flag.name} [${flag.possibleValues.join('/')}]`;
  }
  if (flag.takesValue) {
    return `--${flag.name} &lt;${flag.valueName}&gt;`;
  }
  return `--${flag.name}`;
}

export function updateCliUsage() {
  const cliJson: CliJson = getJson(join(rootDir, 'assets/generated/cli.json'));

  const headerContent = [
    { content: 'Command', width: 120 },
    { content: 'Description', width: 280 },
    { content: 'Flag', width: 200 },
    { content: 'Default', width: 100 },
    { content: 'Flag description', width: 300 },
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
        { content: '-', align: 'center' },
      ]);
    } else {
      let currentGroup: string | undefined;

      for (const flag of cmd.flags) {
        const flagName = formatFlagName(flag);
        const defaultValue = flag.defaultValue ?? '-';

        const showGroupHeader = flag.group && flag.group !== currentGroup;
        if (showGroupHeader) {
          currentGroup = flag.group;
        }

        const groupPrefix = showGroupHeader ? `<b>${flag.group}</b><br/>` : '';
        const flagContent = `${groupPrefix}<code>${flagName}</code>`;

        table.addBodyRow([
          { content: cmdName, align: 'left' },
          { content: cmd.description, align: 'left' },
          { content: flagContent, align: 'left' },
          { content: defaultValue, align: 'center' },
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

  const readmePath = join(rootDir, 'packages/cli/README.md');
  const readme = new DynMarkdown<TFields>(readmePath);
  readme.updateField('CLI_USAGE', content);
  readme.saveFile();

  console.log(`✓ Updated CLI_USAGE in cli readme (${cliJson.commands.length} commands)`);
}
