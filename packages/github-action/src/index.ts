import { ScanMode } from './constants';
import { updateOrCreateComment } from './core/comment-updater';
import { type ActionInputs, getActionInputs } from './core/input-validator';
import { type ScanOptions, type ScanResult, scanChangedFiles } from './core/scanner';
import { type Octokit, githubHelper } from './lib/actions-helper';
import { gitHelper } from './lib/git-helper';
import { validateConfigFiles } from './utils/config-validator';

class ActionRunner {
  async run() {
    try {
      const inputs = getActionInputs();

      validateConfigFiles(inputs.configPath);

      if (inputs.mode === ScanMode.Branch) {
        const prInfo = githubHelper.getContext().payload.pull_request;

        if (!prInfo) {
          githubHelper.setFailed('Branch mode requires pull_request events');
          return;
        }
      }

      const scanResults = await this.executeScan(inputs);
      const octokit = githubHelper.getOctokit(inputs.token);

      await this.handlePRComment(inputs, octokit, scanResults);
      this.handleScanResults(scanResults, inputs);
    } catch (error) {
      githubHelper.setFailed(`Action failed: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  private async executeScan(inputs: ActionInputs): Promise<ScanResult> {
    const commonParams = {
      devMode: inputs.devMode,
      tscannerVersion: inputs.tscannerVersion,
      groupBy: inputs.groupBy,
      configPath: inputs.configPath,
    } satisfies ScanOptions;

    if (inputs.mode === ScanMode.Branch) {
      await gitHelper.fetchBranch(inputs.targetBranch);
      return scanChangedFiles({
        ...commonParams,
        targetBranch: inputs.targetBranch,
      });
    }

    return scanChangedFiles({
      ...commonParams,
    });
  }

  private async handlePRComment(inputs: ActionInputs, octokit: Octokit, scanResult: ScanResult) {
    const context = githubHelper.getContext();
    const prInfo = context.payload.pull_request;

    if (!prInfo) {
      return;
    }

    const prNumber = prInfo.number;
    const { owner, repo } = context.repo;
    const latestCommitSha = prInfo.head.sha.substring(0, 7);
    const commitMessage = await gitHelper.getCommitMessage(prInfo.head.sha);

    await updateOrCreateComment({
      octokit,
      owner,
      repo,
      prNumber,
      scanResult,
      timezone: inputs.timezone,
      commitSha: latestCommitSha,
      commitMessage,
      targetBranch: inputs.mode === ScanMode.Branch ? inputs.targetBranch : undefined,
    });
  }

  private handleScanResults(scanResult: ScanResult, inputs: ActionInputs): void {
    if (scanResult.totalErrors > 0) {
      const loggerMethod = inputs.continueOnError ? githubHelper.logInfo : githubHelper.setFailed;
      loggerMethod(`Found ${scanResult.totalErrors} error(s)`);
    } else {
      githubHelper.logInfo('No errors found');
    }
  }
}

new ActionRunner().run();
