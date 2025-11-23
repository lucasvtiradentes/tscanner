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
    const prInfo = context.payload.pull_request;

    let scanResults;

    if (inputs.mode === ScanMode.Branch) {
      if (!prInfo) {
        githubHelper.setFailed('Branch mode requires pull_request events');
        return;
      }

      const prNumber = prInfo.number;

      githubHelper.logInfo(`Scanning PR #${prNumber} against ${inputs.targetBranch}`);

      await gitHelper.fetchBranch(inputs.targetBranch);

      scanResults = await scanChangedFiles(inputs.targetBranch, inputs.devMode, inputs.tscannerVersion, inputs.groupBy);
    } else {
      githubHelper.logInfo('Scanning entire codebase');

      scanResults = await scanChangedFiles(undefined, inputs.devMode, inputs.tscannerVersion, inputs.groupBy);
    }

    if (prInfo) {
      const prNumber = prInfo.number;
      const { owner, repo } = context.repo;
      const latestCommitSha = prInfo.head.sha.substring(0, 7);
      const commitMessage = await gitHelper.getCommitMessage(prInfo.head.sha);

      await updateOrCreateComment({
        octokit,
        owner,
        repo,
        prNumber,
        scanResult: scanResults,
        timezone: inputs.timezone,
        commitSha: latestCommitSha,
        commitMessage,
        targetBranch: inputs.mode === ScanMode.Branch ? inputs.targetBranch : undefined,
      });
    }

    if (scanResults.totalErrors > 0) {
      console.log(inputs.continueOnError);
      const loggerMethod = inputs.continueOnError ? githubHelper.setFailed : githubHelper.logInfo;
      loggerMethod(`Found ${scanResults.totalErrors} error(s)`);
    } else {
      githubHelper.logInfo('No errors found');
    }
  } catch (error) {
    githubHelper.setFailed(`Action failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

run();
