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
  githubHelper.logInfo('');
  githubHelper.logInfo('📝 Writing annotations via Checks API...');
  githubHelper.logInfo(`ruleGroups count: ${scanResult.ruleGroups.length}`);
  githubHelper.logInfo(`ruleGroupsByRule count: ${scanResult.ruleGroupsByRule.length}`);

  const annotations: CheckAnnotation[] = [];

  for (const group of scanResult.ruleGroupsByRule) {
    githubHelper.logInfo(
      `Processing rule: ${group.ruleName} (${group.issueCount} issues, ${group.files.length} files)`,
    );

    for (const file of group.files) {
      githubHelper.logInfo(`  File: ${file.filePath} (${file.issues.length} issues)`);

      for (const issue of file.issues) {
        const ruleName = issue.ruleName ?? group.ruleName;

        githubHelper.logInfo(`    -> Line ${issue.line}: ${ruleName}`);

        annotations.push({
          path: file.filePath,
          start_line: issue.line,
          end_line: issue.line,
          annotation_level: group.severity === Severity.Error ? 'failure' : 'warning',
          message: issue.message,
          title: ruleName,
        });
      }
    }
  }

  githubHelper.logInfo(`Total annotations to write: ${annotations.length}`);

  if (annotations.length === 0) {
    githubHelper.logInfo('No annotations to write');
    return;
  }

  const context = githubHelper.getContext();
  const { owner, repo } = context.repo;

  const headSha = context.payload.pull_request?.head.sha ?? context.sha;

  githubHelper.logInfo(`Creating check run for SHA: ${headSha}`);
  githubHelper.logInfo(`Owner: ${owner}, Repo: ${repo}`);

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

  githubHelper.logInfo(`Splitting ${annotations.length} annotations into ${chunks.length} chunk(s)`);

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

    githubHelper.logInfo(`Check run created with ID: ${checkRun.id}`);

    for (let i = 1; i < chunks.length; i++) {
      githubHelper.logInfo(`Updating check run with chunk ${i + 1}/${chunks.length} (${chunks[i].length} annotations)`);

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

    githubHelper.logInfo(`✅ All ${annotations.length} annotations written successfully`);
  } catch (error) {
    githubHelper.logError(`Failed to create check run: ${error instanceof Error ? error.message : String(error)}`);
    if (error instanceof Error && 'status' in error) {
      githubHelper.logError(`Status: ${(error as { status: number }).status}`);
    }
    throw error;
  }
}
