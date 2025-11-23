import { GroupMode } from '../constants';
import { githubHelper } from '../lib/actions-helper';

export type ActionInputs = {
  token: string;
  targetBranch: string;
  timezone: string;
  configPath: string;
  tscannerVersion: string;
  devMode: boolean;
  groupBy: GroupMode;
};

const DEFAULT_INPUTS = {
  targetBranch: 'origin/main',
  timezone: 'UTC',
  configPath: '.tscanner/rules.json',
  tscannerVersion: 'latest',
  groupBy: GroupMode.File,
} as const satisfies Partial<ActionInputs>;

export function getActionInputs(): ActionInputs {
  const token = githubHelper.getInput('github-token', { required: true });
  const targetBranch = githubHelper.getInput('target-branch') || DEFAULT_INPUTS.targetBranch;
  const timezone = githubHelper.getInput('timezone') || DEFAULT_INPUTS.timezone;
  const configPath = githubHelper.getInput('config-path') || DEFAULT_INPUTS.configPath;
  const tscannerVersion = githubHelper.getInput('tscanner-version') || DEFAULT_INPUTS.tscannerVersion;
  const devMode = githubHelper.getInput('dev-mode') === 'true';
  const groupByInput = githubHelper.getInput('group-by') || DEFAULT_INPUTS.groupBy;

  const groupBy = groupByInput === GroupMode.Rule ? GroupMode.Rule : GroupMode.File;

  if (configPath !== DEFAULT_INPUTS.configPath) {
    githubHelper.logWarning(
      `config-path is currently ignored. tscanner CLI always uses ${DEFAULT_INPUTS.configPath} from project root.`,
    );
  }

  return {
    token,
    targetBranch,
    timezone,
    configPath,
    tscannerVersion,
    devMode,
    groupBy,
  };
}
