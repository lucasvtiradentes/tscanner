import { z } from 'zod';
import { GroupMode, PACKAGE_NAME, ScanMode } from '../constants';
import { githubHelper } from '../lib/actions-helper';

const baseInputsSchema = z.object({
  token: z.string(),
  timezone: z.string(),
  configPath: z.string(),
  tscannerVersion: z.string(),
  devMode: z.boolean(),
  groupBy: z.enum(GroupMode),
  continueOnError: z.boolean(),
});

const branchScannerSchema = baseInputsSchema.extend({
  mode: z.literal(ScanMode.Branch),
  targetBranch: z.string(),
});

const codebaseScannerSchema = baseInputsSchema.extend({
  mode: z.literal(ScanMode.Codebase),
});

const actionInputsSchema = z.discriminatedUnion('mode', [branchScannerSchema, codebaseScannerSchema]);

export type ActionInputs = z.infer<typeof actionInputsSchema>;

const DEFAULT_INPUTS = {
  timezone: 'UTC',
  configPath: '.tscanner/rules.json',
  tscannerVersion: 'latest',
  groupBy: GroupMode.File,
  targetBranch: 'origin/main',
} as const;

export function getActionInputs(): ActionInputs {
  const token = githubHelper.getInput('github-token', { required: true });
  const timezone = githubHelper.getInput('timezone') || DEFAULT_INPUTS.timezone;
  const configPath = githubHelper.getInput('config-path') || DEFAULT_INPUTS.configPath;
  const tscannerVersion = githubHelper.getInput('tscanner-version') || DEFAULT_INPUTS.tscannerVersion;
  const devMode = githubHelper.getInput('dev-mode') === 'true';
  const groupByInput = githubHelper.getInput('group-by') || DEFAULT_INPUTS.groupBy;
  const targetBranch = githubHelper.getInput('target-branch');
  const continueOnError = githubHelper.getInput('continue-on-error') === 'true';

  const groupBy = groupByInput === GroupMode.Rule ? GroupMode.Rule : GroupMode.File;

  if (configPath !== DEFAULT_INPUTS.configPath) {
    githubHelper.logWarning(
      `config-path is currently ignored. ${PACKAGE_NAME} CLI always uses ${DEFAULT_INPUTS.configPath} from project root.`,
    );
  }

  const mode = targetBranch ? ScanMode.Branch : ScanMode.Codebase;

  const rawInputs = {
    token,
    timezone,
    configPath,
    tscannerVersion,
    devMode,
    groupBy,
    continueOnError,
    mode,
    ...(mode === ScanMode.Branch && { targetBranch: targetBranch || DEFAULT_INPUTS.targetBranch }),
  };

  return actionInputsSchema.parse(rawInputs);
}
