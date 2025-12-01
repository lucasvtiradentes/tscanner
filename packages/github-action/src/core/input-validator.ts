import { CONFIG_DIR_NAME, GroupMode, ScanMode } from 'tscanner-common';
import { z } from 'zod';
import { githubHelper } from '../lib/actions-helper';

const actionYmlInputsSchema = z.object({
  githubToken: z.string(),
  targetBranch: z.string().optional(),
  timezone: z.string(),
  configPath: z.string(),
  tscannerVersion: z.string(),
  groupBy: z.enum(GroupMode),
  continueOnError: z.boolean(),
  annotations: z.boolean(),
  summary: z.boolean(),
  prComment: z.boolean(),
  devMode: z.boolean(),
});

const branchScannerSchema = actionYmlInputsSchema.extend({
  mode: z.literal(ScanMode.Branch),
  targetBranch: z.string(),
});

const codebaseScannerSchema = actionYmlInputsSchema.extend({
  mode: z.literal(ScanMode.Codebase),
});

const actionInputsSchema = z.discriminatedUnion('mode', [branchScannerSchema, codebaseScannerSchema]);

export type ActionInputs = z.infer<typeof actionInputsSchema>;

const DEFAULT_INPUTS = {
  timezone: 'UTC',
  configPath: CONFIG_DIR_NAME,
  tscannerVersion: 'latest',
  groupBy: GroupMode.File,
  targetBranch: 'origin/main',
} as const;

export function getActionInputs(): ActionInputs {
  const githubToken = githubHelper.getInput('github-token', { required: true });
  const timezone = githubHelper.getInput('timezone') || DEFAULT_INPUTS.timezone;
  const configPath = githubHelper.getInput('config-path') || DEFAULT_INPUTS.configPath;
  const tscannerVersion = githubHelper.getInput('tscanner-version') || DEFAULT_INPUTS.tscannerVersion;
  const devMode = githubHelper.getInput('dev-mode') === 'true';
  const groupByInput = githubHelper.getInput('group-by') || DEFAULT_INPUTS.groupBy;
  const targetBranch = githubHelper.getInput('target-branch');
  const continueOnError = githubHelper.getInput('continue-on-error') === 'true';
  const annotations = githubHelper.getInput('annotations') !== 'false';
  const summary = githubHelper.getInput('summary') !== 'false';
  const prComment = githubHelper.getInput('pr-comment') !== 'false';

  const groupBy = groupByInput === GroupMode.Rule ? GroupMode.Rule : GroupMode.File;

  const mode = targetBranch ? ScanMode.Branch : ScanMode.Codebase;

  const rawInputs = {
    githubToken,
    timezone,
    configPath,
    tscannerVersion,
    devMode,
    groupBy,
    continueOnError,
    annotations,
    summary,
    prComment,
    mode,
    ...(mode === ScanMode.Branch && { targetBranch: targetBranch || DEFAULT_INPUTS.targetBranch }),
  };

  return actionInputsSchema.parse(rawInputs);
}
