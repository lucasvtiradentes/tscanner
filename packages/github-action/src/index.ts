import * as core from '@actions/core';
import * as github from '@actions/github';
import { updateOrCreateComment } from './core/comment-updater';
import { getActionInputs } from './core/input-validator';
import { scanChangedFiles } from './core/scanner';
import { fetchBranch, getCommitMessage } from './lib/git-helper';

async function run(): Promise<void> {
  try {
    const inputs = getActionInputs();
    const octokit = github.getOctokit(inputs.token);
    const context = github.context;

    if (!context.payload.pull_request) {
      core.setFailed('This action only works on pull_request events');
      return;
    }

    const prNumber = context.payload.pull_request.number;
    const { owner, repo } = context.repo;

    core.info(`Scanning PR #${prNumber} against ${inputs.targetBranch}`);

    await fetchBranch(inputs.targetBranch);

    const scanResults = await scanChangedFiles(
      inputs.targetBranch,
      inputs.devMode,
      inputs.tscannerVersion,
      inputs.groupBy,
    );

    const latestCommitSha = context.payload.pull_request.head.sha.substring(0, 7);
    const commitMessage = await getCommitMessage(context.payload.pull_request.head.sha);

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
      core.setFailed(`Found ${scanResults.totalErrors} error(s)`);
    } else {
      core.info('No errors found');
    }
  } catch (error) {
    core.setFailed(`Action failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

run();
