import * as core from '@actions/core';
import * as github from '@actions/github';
import * as exec from '@actions/exec';
import { formatComment } from './formatter';
import { scanChangedFiles } from './scanner';

async function run(): Promise<void> {
  try {
    const token = core.getInput('github-token', { required: true });
    const targetBranch = core.getInput('target-branch') || 'origin/main';
    const timezone = core.getInput('timezone') || 'UTC';
    const configPath = core.getInput('config-path') || '.tscanner/rules.json';
    const tscannerVersion = core.getInput('tscanner-version') || 'latest';
    const devMode = core.getInput('dev-mode') === 'true';

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

    const scanResults = await scanChangedFiles(targetBranch, devMode, tscannerVersion);

    const latestCommit = context.payload.pull_request.head.sha.substring(0, 7);
    const comment = formatComment(scanResults, timezone, latestCommit);

    const existingComments = await octokit.rest.issues.listComments({
      owner,
      repo,
      issue_number: prNumber,
    });

    const botComment = existingComments.data.find(
      (c) => c.user?.type === 'Bot' && c.body?.includes('<!-- tscanner-pr-comment -->'),
    );

    if (botComment) {
      await octokit.rest.issues.updateComment({
        owner,
        repo,
        comment_id: botComment.id,
        body: comment,
      });
      core.info(`Updated existing comment #${botComment.id}`);
    } else {
      await octokit.rest.issues.createComment({
        owner,
        repo,
        issue_number: prNumber,
        body: comment,
      });
      core.info('Created new comment');
    }

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
