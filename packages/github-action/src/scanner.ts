import * as exec from '@actions/exec';
import * as core from '@actions/core';

export type ScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  totalRules: number;
  groupBy: 'rule' | 'file';
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
  ruleName?: string;
};

type CliJsonOutputByRule = {
  rules: Array<{
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

type CliJsonOutputByFile = {
  files: Array<{
    file: string;
    issues: Array<{
      rule: string;
      severity: string;
      line: number;
      column: number;
      message: string;
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

type CliJsonOutput = CliJsonOutputByRule | CliJsonOutputByFile;

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

  if (devMode) {
    const workspaceRoot = process.env.GITHUB_WORKSPACE || process.cwd();
    command = 'node';
    args = [`${workspaceRoot}/packages/cli/dist/main.js`, 'check', '--json', '--branch', targetBranch, '--exit-zero'];
    if (groupBy === 'rule') {
      args.splice(3, 0, '--by-rule');
    }
    core.info(`Using local CLI: ${workspaceRoot}/packages/cli/dist/main.js`);
  } else {
    const packageSpec = `tscanner@${tscannerVersion}`;
    command = 'npx';
    args = [packageSpec, 'check', '--json', '--branch', targetBranch, '--exit-zero'];
    if (groupBy === 'rule') {
      args.splice(3, 0, '--by-rule');
    }
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

  const hasIssues =
    ('rules' in scanData && scanData.rules.length > 0) || ('files' in scanData && scanData.files.length > 0);

  if (!hasIssues) {
    core.info('No issues found');
    return {
      totalIssues: 0,
      totalErrors: 0,
      totalWarnings: 0,
      totalFiles: 0,
      totalRules: 0,
      groupBy: groupBy as 'rule' | 'file',
      ruleGroups: [],
    };
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

  let ruleGroups: RuleGroup[];

  if ('rules' in scanData) {
    ruleGroups = scanData.rules.map((ruleData) => {
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
  } else {
    const fileGroups: Array<{ file: string; issues: Issue[]; severity: 'error' | 'warning' }> = scanData.files.map(
      (fileData) => ({
        file: fileData.file,
        issues: fileData.issues.map((issue) => ({
          line: issue.line,
          column: issue.column,
          message: issue.message,
          lineText: issue.line_text,
          ruleName: issue.rule,
        })),
        severity: fileData.issues.some((i) => i.severity === 'error') ? 'error' : 'warning',
      }),
    );

    fileGroups.sort((a, b) => {
      if (a.severity !== b.severity) {
        return a.severity === 'error' ? -1 : 1;
      }
      return b.issues.length - a.issues.length;
    });

    ruleGroups = fileGroups.map((fileGroup) => ({
      ruleName: fileGroup.file,
      severity: fileGroup.severity,
      issueCount: fileGroup.issues.length,
      fileCount: 1,
      files: [
        {
          filePath: fileGroup.file,
          issues: fileGroup.issues,
        },
      ],
    }));
  }

  return {
    totalIssues: scanData.summary.total_issues,
    totalErrors: scanData.summary.errors,
    totalWarnings: scanData.summary.warnings,
    totalFiles: scanData.summary.total_files,
    totalRules: ruleGroups.length,
    groupBy: groupBy as 'rule' | 'file',
    ruleGroups,
  };
}
