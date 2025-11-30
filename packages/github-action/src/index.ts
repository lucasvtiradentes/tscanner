import { ScanMode } from 'tscanner-common';
import { writeAnnotations } from './core/annotation-writer';
import { updateOrCreateComment } from './core/comment-updater';
import { type ActionInputs, getActionInputs } from './core/input-validator';
import { type ScanOptions, type ScanResult, scanChangedFiles } from './core/scanner';
import { writeSummary } from './core/summary-writer';
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

      if (inputs.annotations && scanResults.totalIssues > 0) {
        writeAnnotations(scanResults);
      }

      if (inputs.summary) {
        const targetBranch = inputs.mode === ScanMode.Branch ? inputs.targetBranch : undefined;
        writeSummary(scanResults, targetBranch);
      }

      this.handleScanResults(scanResults, inputs);
    } catch (error) {
      const errorData = {
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
      };

      githubHelper.logInfo(`error caught: ${JSON.stringify(errorData, null, 2)}`);
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
    const commitMessage = await this.getCommitMessageFromApi(octokit, owner, repo, prInfo.head.sha);

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

  private async getCommitMessageFromApi(octokit: Octokit, owner: string, repo: string, sha: string): Promise<string> {
    try {
      const { data } = await octokit.rest.repos.getCommit({ owner, repo, ref: sha });
      return data.commit.message.split('\n')[0];
    } catch {
      githubHelper.logWarning(`Failed to get commit message for ${sha}`);
      return '';
    }
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
