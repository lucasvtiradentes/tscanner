import * as core from '@actions/core';
import {
  DEFAULT_CONFIG_PATH,
  DEFAULT_GROUP_BY,
  DEFAULT_TARGET_BRANCH,
  DEFAULT_TIMEZONE,
  DEFAULT_TSCANNER_VERSION,
  GroupMode,
} from '../constants';

export type ActionInputs = {
  token: string;
  targetBranch: string;
  timezone: string;
  configPath: string;
  tscannerVersion: string;
  devMode: boolean;
  groupBy: GroupMode;
};

export function getActionInputs(): ActionInputs {
  const token = core.getInput('github-token', { required: true });
  const targetBranch = core.getInput('target-branch') || DEFAULT_TARGET_BRANCH;
  const timezone = core.getInput('timezone') || DEFAULT_TIMEZONE;
  const configPath = core.getInput('config-path') || DEFAULT_CONFIG_PATH;
  const tscannerVersion = core.getInput('tscanner-version') || DEFAULT_TSCANNER_VERSION;
  const devMode = core.getInput('dev-mode') === 'true';
  const groupByInput = core.getInput('group-by') || DEFAULT_GROUP_BY;

  const groupBy = groupByInput === GroupMode.Rule ? GroupMode.Rule : GroupMode.File;

  if (configPath !== DEFAULT_CONFIG_PATH) {
    core.warning(
      `config-path is currently ignored. tscanner CLI always uses ${DEFAULT_CONFIG_PATH} from project root.`,
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
