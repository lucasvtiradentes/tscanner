import { AiExecutionMode, type GroupMode, PACKAGE_NAME } from './constants';

export type CliCheckOptions = {
  branch?: string;
  groupBy?: GroupMode;
  filter?: { type: string; value: string };
  aiMode?: AiExecutionMode;
  jsonOutput?: string;
  configPath?: string;
  continueOnError?: boolean;
  noCache?: boolean;
};

export function buildCheckArgs(options: CliCheckOptions = {}): string[] {
  const args: string[] = ['check'];

  if (options.filter) {
    args.push(`--${options.filter.type}`, options.filter.value);
  }

  if (options.groupBy) {
    args.push('--group-by', options.groupBy);
  }

  if (options.branch) {
    args.push('--branch', options.branch);
  }

  if (options.aiMode === AiExecutionMode.Include) {
    args.push('--include-ai');
  } else if (options.aiMode === AiExecutionMode.Only) {
    args.push('--only-ai');
  }

  if (options.jsonOutput) {
    args.push('--json-output', options.jsonOutput);
  }

  if (options.configPath) {
    args.push('--config-path', options.configPath);
  }

  if (options.continueOnError) {
    args.push('--continue-on-error');
  }

  if (options.noCache) {
    args.push('--no-cache');
  }

  return args;
}

export function buildCheckCommand(options: CliCheckOptions = {}): string {
  const args = buildCheckArgs(options);
  return `${PACKAGE_NAME} ${args.join(' ')}`;
}
