import * as exec from '@actions/exec';
import * as core from '@actions/core';

export type ScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  ruleGroups: RuleGroup[];
};

export type RuleGroup = {
  ruleName: string;
  severity: 'error' | 'warning';
  issueCount: number;
  fileCount: number;
  files: FileIssues[];
};

export type FileIssues = {
  filePath: string;
  issues: Issue[];
};

export type Issue = {
  line: number;
  column: number;
  message: string;
  lineText: string;
};

type CliJsonOutput = {
  rules?: Array<{
    rule: string;
    count: number;
    issues: Array<{
      file: string;
      line: number;
      column: number;
      message: string;
      severity: string;
      line_text: string;
    }>;
  }>;
  summary: {
    total_files: number;
    total_issues: number;
    errors: number;
    warnings: number;
  };
};

export async function scanChangedFiles(
  targetBranch: string,
  devMode: boolean,
  tscannerVersion: string,
  groupBy: string,
): Promise<ScanResult> {
  core.info(`Scanning changed files vs ${targetBranch} (dev mode: ${devMode}, group by: ${groupBy})`);

  let scanOutput = '';
  let scanError = '';

  let command: string;
  let args: string[];

  const groupFlag = groupBy === 'file' ? '--by-file' : '--by-rule';

  if (devMode) {
    const workspaceRoot = process.env.GITHUB_WORKSPACE || process.cwd();
    command = 'node';
    args = [
      `${workspaceRoot}/packages/cli/dist/main.js`,
      'check',
      '--json',
      groupFlag,
      '--branch',
      targetBranch,
      '--exit-zero',
    ];
    core.info(`Using local CLI: ${workspaceRoot}/packages/cli/dist/main.js`);
  } else {
    const packageSpec = `tscanner@${tscannerVersion}`;
    command = 'npx';
    args = [packageSpec, 'check', '--json', groupFlag, '--branch', targetBranch, '--exit-zero'];
    core.info(`Using published tscanner from npm: ${packageSpec}`);
  }

  await exec.exec(command, args, {
    silent: true,
    listeners: {
      stdout: (data: Buffer) => {
        scanOutput += data.toString();
      },
      stderr: (data: Buffer) => {
        scanError += data.toString();
      },
    },
    ignoreReturnCode: true,
  });

  if (scanError && !scanError.includes('Scanning') && !scanError.includes('Comparing')) {
    core.warning(`Scan stderr: ${scanError}`);
  }

  let scanData: CliJsonOutput;
  try {
    scanData = JSON.parse(scanOutput);
  } catch {
    core.error('Failed to parse scan output');
    core.debug(`Raw output: ${scanOutput.substring(0, 500)}`);
    throw new Error('Invalid scan output format');
  }

  core.info(`Scan completed: ${scanData.summary?.total_issues || 0} issues found`);

  if (!scanData.rules || scanData.rules.length === 0) {
    core.info('No issues found');
    return { totalIssues: 0, totalErrors: 0, totalWarnings: 0, ruleGroups: [] };
  }

  core.info('');
  core.info('📊 Scan Results:');
  core.info('');
  await exec.exec(
    command,
    args.map((arg) => (arg === '--json' ? '--pretty' : arg)),
    {
      ignoreReturnCode: true,
    },
  );

  const ruleGroups: RuleGroup[] = scanData.rules.map((ruleData) => {
    const fileMap = new Map<string, Issue[]>();

    for (const issue of ruleData.issues) {
      if (!fileMap.has(issue.file)) {
        fileMap.set(issue.file, []);
      }
      fileMap.get(issue.file)!.push({
        line: issue.line,
        column: issue.column,
        message: issue.message,
        lineText: issue.line_text,
      });
    }

    const files: FileIssues[] = Array.from(fileMap.entries()).map(([filePath, issues]) => ({
      filePath,
      issues,
    }));

    const severity = ruleData.issues[0]?.severity === 'error' ? 'error' : 'warning';

    return {
      ruleName: ruleData.rule,
      severity,
      issueCount: ruleData.count,
      fileCount: fileMap.size,
      files,
    };
  });

  ruleGroups.sort((a, b) => {
    if (a.severity !== b.severity) {
      return a.severity === 'error' ? -1 : 1;
    }
    return b.issueCount - a.issueCount;
  });

  return {
    totalIssues: scanData.summary.total_issues,
    totalErrors: scanData.summary.errors,
    totalWarnings: scanData.summary.warnings,
    ruleGroups,
  };
}
