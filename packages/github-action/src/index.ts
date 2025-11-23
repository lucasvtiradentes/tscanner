import { ScanMode } from './constants';
import { updateOrCreateComment } from './core/comment-updater';
import { getActionInputs } from './core/input-validator';
import { scanChangedFiles } from './core/scanner';
import { githubHelper } from './lib/actions-helper';
import { gitHelper } from './lib/git-helper';

async function run() {
  try {
    const inputs = getActionInputs();
    const octokit = githubHelper.getOctokit(inputs.token);
    const context = githubHelper.getContext();

    if (inputs.mode === ScanMode.Branch) {
      if (!context.payload.pull_request) {
        githubHelper.setFailed('Branch mode requires pull_request events');
        return;
      }

      const prNumber = context.payload.pull_request.number;
      const { owner, repo } = context.repo;

      githubHelper.logInfo(`Scanning PR #${prNumber} against ${inputs.targetBranch}`);

      await gitHelper.fetchBranch(inputs.targetBranch);

      const scanResults = await scanChangedFiles(
        inputs.targetBranch,
        inputs.devMode,
        inputs.tscannerVersion,
        inputs.groupBy,
      );

      const latestCommitSha = context.payload.pull_request.head.sha.substring(0, 7);
      const commitMessage = await gitHelper.getCommitMessage(context.payload.pull_request.head.sha);

      await updateOrCreateComment({
        octokit,
        owner,
        repo,
        prNumber,
        scanResult: scanResults,
        timezone: inputs.timezone,
        commitSha: latestCommitSha,
        commitMessage,
      });

      if (scanResults.totalErrors > 0) {
        githubHelper.setFailed(`Found ${scanResults.totalErrors} error(s)`);
      } else {
        githubHelper.logInfo('No errors found');
      }
    } else {
      githubHelper.logInfo('Scanning entire codebase');

      const scanResults = await scanChangedFiles(undefined, inputs.devMode, inputs.tscannerVersion, inputs.groupBy);

      if (scanResults.totalErrors > 0) {
        githubHelper.setFailed(`Found ${scanResults.totalErrors} error(s)`);
      } else {
        githubHelper.logInfo('No errors found');
      }
    }
  } catch (error) {
    githubHelper.setFailed(`Action failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

run();
