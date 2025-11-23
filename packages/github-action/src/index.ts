import * as core from '@actions/core';
import * as github from '@actions/github';
import * as exec from '@actions/exec';
import { scanChangedFiles } from './scanner';
import { updateOrCreateComment } from './comment-updater';

async function run(): Promise<void> {
  try {
    const token = core.getInput('github-token', { required: true });
    const targetBranch = core.getInput('target-branch') || 'origin/main';
    const timezone = core.getInput('timezone') || 'UTC';
    const configPath = core.getInput('config-path') || '.tscanner/rules.json';
    const tscannerVersion = core.getInput('tscanner-version') || 'latest';
    const devMode = core.getInput('dev-mode') === 'true';
    const groupBy = core.getInput('group-by') || 'rule';

    if (configPath !== '.tscanner/rules.json') {
      core.warning(
        'config-path is currently ignored. tscanner CLI always uses .tscanner/rules.json from project root.',
      );
    }

    const octokit = github.getOctokit(token);
    const context = github.context;

    if (!context.payload.pull_request) {
      core.setFailed('This action only works on pull_request events');
      return;
    }

    const prNumber = context.payload.pull_request.number;
    const { owner, repo } = context.repo;

    core.info(`Scanning PR #${prNumber} against ${targetBranch}`);

    await exec.exec('git', ['fetch', 'origin', targetBranch.replace('origin/', '')]);

    const scanResults = await scanChangedFiles(targetBranch, devMode, tscannerVersion, groupBy);

    const latestCommitSha = context.payload.pull_request.head.sha.substring(0, 7);
    let commitMessage = '';

    let gitLogOutput = '';
    await exec.exec('git', ['log', '-1', '--pretty=%s', context.payload.pull_request.head.sha], {
      silent: true,
      listeners: {
        stdout: (data: Buffer) => {
          gitLogOutput += data.toString();
        },
      },
    });
    commitMessage = gitLogOutput.trim();

    await updateOrCreateComment({
      octokit,
      owner,
      repo,
      prNumber,
      scanResult: scanResults,
      timezone,
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
