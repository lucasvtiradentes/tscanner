import { GroupMode, Severity } from '../constants';
import { githubHelper } from '../lib/actions-helper';
import { type CliExecutor, createDevModeExecutor, createProdModeExecutor } from './cli-executor';

export type ScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  totalRules: number;
  groupBy: GroupMode;
  ruleGroups: RuleGroup[];
};

export type RuleGroup = {
  ruleName: string;
  severity: Severity;
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
  groupBy: GroupMode,
): Promise<ScanResult> {
  githubHelper.logInfo(`Scanning changed files vs ${targetBranch} (dev mode: ${devMode}, group by: ${groupBy})`);

  const executor: CliExecutor = devMode ? createDevModeExecutor() : createProdModeExecutor(tscannerVersion);

  const args = ['check', '--json', '--branch', targetBranch, '--exit-zero'];
  if (groupBy === GroupMode.Rule) {
    args.splice(1, 0, '--by-rule');
  }

  const scanOutput = await executor.execute(args);

  let scanData: CliJsonOutput;
  try {
    scanData = JSON.parse(scanOutput);
  } catch {
    githubHelper.logError('Failed to parse scan output');
    githubHelper.logDebug(`Raw output: ${scanOutput.substring(0, 500)}`);
    throw new Error('Invalid scan output format');
  }

  githubHelper.logInfo(`Scan completed: ${scanData.summary?.total_issues || 0} issues found`);

  const hasIssues =
    ('rules' in scanData && scanData.rules.length > 0) || ('files' in scanData && scanData.files.length > 0);

  if (!hasIssues) {
    githubHelper.logInfo('No issues found');
    return {
      totalIssues: 0,
      totalErrors: 0,
      totalWarnings: 0,
      totalFiles: 0,
      totalRules: 0,
      groupBy,
      ruleGroups: [],
    };
  }

  githubHelper.logInfo('');
  githubHelper.logInfo('📊 Scan Results:');
  githubHelper.logInfo('');
  await executor.displayResults(args.map((arg) => (arg === '--json' ? '--pretty' : arg)));

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

      const severity = ruleData.issues[0]?.severity === 'error' ? Severity.Error : Severity.Warning;

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
    const fileGroups: Array<{ file: string; issues: Issue[]; severity: Severity }> = scanData.files.map((fileData) => ({
      file: fileData.file,
      issues: fileData.issues.map((issue) => ({
        line: issue.line,
        column: issue.column,
        message: issue.message,
        lineText: issue.line_text,
        ruleName: issue.rule,
      })),
      severity: fileData.issues.some((i) => i.severity === 'error') ? Severity.Error : Severity.Warning,
    }));

    fileGroups.sort((a, b) => {
      if (a.severity !== b.severity) {
        return a.severity === Severity.Error ? -1 : 1;
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
    groupBy,
    ruleGroups,
  };
}
