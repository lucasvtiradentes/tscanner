import { Severity } from 'tscanner-common';
import { type Octokit, githubHelper } from '../lib/actions-helper';
import type { ScanResult } from './scanner';

type CheckAnnotation = {
  path: string;
  start_line: number;
  end_line: number;
  annotation_level: 'notice' | 'warning' | 'failure';
  message: string;
  title: string;
};

export async function writeAnnotations(octokit: Octokit, scanResult: ScanResult): Promise<void> {
  const annotations: CheckAnnotation[] = [];

  for (const group of scanResult.ruleGroupsByRule) {
    for (const file of group.files) {
      for (const issue of file.issues) {
        annotations.push({
          path: file.filePath,
          start_line: issue.line,
          end_line: issue.line,
          annotation_level: group.severity === Severity.Error ? 'failure' : 'warning',
          message: issue.message,
          title: issue.ruleName ?? group.ruleName,
        });
      }
    }
  }

  if (annotations.length === 0) {
    return;
  }

  const context = githubHelper.getContext();
  const { owner, repo } = context.repo;
  const headSha = context.payload.pull_request?.head.sha ?? context.sha;

  const totalErrors = scanResult.totalErrors;
  const totalWarnings = scanResult.totalWarnings;
  const conclusion = totalErrors > 0 ? 'failure' : 'neutral';
  const title =
    totalErrors > 0 ? `${totalErrors} error(s), ${totalWarnings} warning(s)` : `${totalWarnings} warning(s)`;

  const MAX_ANNOTATIONS_PER_REQUEST = 50;
  const chunks: CheckAnnotation[][] = [];
  for (let i = 0; i < annotations.length; i += MAX_ANNOTATIONS_PER_REQUEST) {
    chunks.push(annotations.slice(i, i + MAX_ANNOTATIONS_PER_REQUEST));
  }

  try {
    const { data: checkRun } = await octokit.rest.checks.create({
      owner,
      repo,
      name: 'TScanner',
      head_sha: headSha,
      status: 'completed',
      conclusion,
      output: {
        title,
        summary: `Found ${scanResult.totalIssues} issue(s) in ${scanResult.totalFiles} file(s)`,
        annotations: chunks[0],
      },
    });

    for (let i = 1; i < chunks.length; i++) {
      await octokit.rest.checks.update({
        owner,
        repo,
        check_run_id: checkRun.id,
        output: {
          title,
          summary: `Found ${scanResult.totalIssues} issue(s) in ${scanResult.totalFiles} file(s)`,
          annotations: chunks[i],
        },
      });
    }

    githubHelper.logInfo(`Annotations written: ${annotations.length}`);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    const status = error instanceof Error && 'status' in error ? (error as { status: number }).status : undefined;

    githubHelper.logError(`Failed to create check run: ${errorMessage}`);
    if (status) {
      githubHelper.logError(`Status: ${status}`);
    }

    if (status === 403) {
      githubHelper.logError('');
      githubHelper.logError('❌ Checks API permission denied.');
      githubHelper.logError('');
      githubHelper.logError('To enable annotations, add "checks: write" permission to your workflow:');
      githubHelper.logError('');
      githubHelper.logError('  permissions:');
      githubHelper.logError('    contents: read');
      githubHelper.logError('    pull-requests: write');
      githubHelper.logError('    checks: write');
      githubHelper.logError('');
      throw new Error('Missing "checks: write" permission. Add it to your workflow permissions.');
    }

    throw error;
  }
}
