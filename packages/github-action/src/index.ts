import { GitHelper, ScanMode } from 'tscanner-common';
import { writeAnnotations } from './core/annotation-writer';
import { updateOrCreateComment } from './core/comment-updater';
import { type ActionInputs, getActionInputs } from './core/input-validator';
import { type ActionScanResult, type ScanOptions, scanChangedFiles } from './core/scanner';
import { writeSummary } from './core/summary-writer';
import { type Octokit, githubHelper, tmpLog } from './lib/actions-helper';
import { validateConfigFiles } from './utils/config-validator';
import { formatTimestamp } from './utils/format-timestamp';

class ActionRunner {
  async run() {
    try {
      tmpLog('ActionRunner.run() started');
      const inputs = getActionInputs();
      tmpLog('getActionInputs() done');

      validateConfigFiles(inputs.configPath);
      tmpLog('validateConfigFiles() done');

      if (inputs.mode === ScanMode.Branch) {
        const prInfo = githubHelper.getContext().payload.pull_request;

        if (!prInfo) {
          githubHelper.setFailed('Branch mode requires pull_request events');
          return;
        }
      }

      tmpLog('executeScan() starting');
      const scanResults = await this.executeScan(inputs);
      tmpLog(`executeScan() done - ${scanResults.totalIssues} issues found`);

      const octokit = githubHelper.getOctokit(inputs.githubToken);

      if (inputs.prComment) {
        tmpLog('handlePRComment() starting');
        await this.handlePRComment(inputs, octokit, scanResults);
        tmpLog('handlePRComment() done');
      }

      if (inputs.annotations && scanResults.totalIssues > 0) {
        tmpLog('writeAnnotations() starting');
        await writeAnnotations(octokit, scanResults);
        tmpLog('writeAnnotations() done');
      }

      if (inputs.summary) {
        tmpLog('writeSummary() starting');
        const context = githubHelper.getContext();
        const prInfo = context.payload.pull_request;
        const { owner, repo } = context.repo;
        const targetBranch = inputs.mode === ScanMode.Branch ? inputs.targetBranch : undefined;
        const commitSha = prInfo?.head?.sha?.substring(0, 7);
        const commitMessage = prInfo
          ? await this.getCommitMessageFromApi(octokit, owner, repo, prInfo.head.sha)
          : undefined;
        const timestamp = formatTimestamp(inputs.timezone);

        writeSummary({
          scanResult: scanResults,
          targetBranch,
          owner,
          repo,
          prNumber: prInfo?.number ?? 0,
          commitSha,
          commitMessage,
          timestamp,
        });
        tmpLog('writeSummary() done');
      }

      this.handleScanResults(scanResults, inputs);
      tmpLog('ActionRunner.run() completed');
    } catch (error) {
      const errorData = {
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
      };

      githubHelper.logInfo(`error caught: ${JSON.stringify(errorData, null, 2)}`);
      githubHelper.setFailed(`Action failed: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  private async executeScan(inputs: ActionInputs): Promise<ActionScanResult> {
    const commonParams = {
      devMode: inputs.devMode,
      tscannerVersion: inputs.tscannerVersion,
      groupBy: inputs.groupBy,
      configPath: inputs.configPath,
      aiMode: inputs.aiMode,
    } satisfies ScanOptions;

    if (inputs.mode === ScanMode.Branch) {
      GitHelper.fetchBranch(inputs.targetBranch, process.cwd());
      return scanChangedFiles({
        ...commonParams,
        targetBranch: inputs.targetBranch,
      });
    }

    return scanChangedFiles({
      ...commonParams,
    });
  }

  private async handlePRComment(inputs: ActionInputs, octokit: Octokit, scanResult: ActionScanResult) {
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

  private handleScanResults(scanResult: ActionScanResult, inputs: ActionInputs): void {
    if (scanResult.totalErrors > 0) {
      const loggerMethod = inputs.continueOnError ? githubHelper.logInfo : githubHelper.setFailed;
      loggerMethod(`Found ${scanResult.totalErrors} error(s)`);
    } else {
      githubHelper.logInfo('No errors found');
    }
  }
}

new ActionRunner().run();
