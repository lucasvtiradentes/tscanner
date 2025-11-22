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
    }>;
  }>;
  summary: {
    total_files: number;
    total_issues: number;
    errors: number;
    warnings: number;
  };
};

export async function scanChangedFiles(targetBranch: string, configPath: string): Promise<ScanResult> {
  core.info(`Scanning changed files vs ${targetBranch}`);

  let scanOutput = '';
  let scanError = '';

  await exec.exec('npx', ['tscanner', 'check', '--json', '--by-rule', '--branch', targetBranch], {
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

  if (scanError && !scanError.includes('Scanning')) {
    core.warning(`Scan stderr: ${scanError}`);
  }

  let scanData: CliJsonOutput;
  try {
    scanData = JSON.parse(scanOutput);
  } catch (e) {
    core.error(`Failed to parse scan output: ${scanOutput}`);
    throw new Error('Invalid scan output format');
  }

  if (!scanData.rules) {
    core.info('No issues found');
    return { totalIssues: 0, totalErrors: 0, totalWarnings: 0, ruleGroups: [] };
  }

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
