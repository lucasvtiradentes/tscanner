import { type GroupMode, Severity } from '../constants';
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
  ruleGroupsByRule: RuleGroup[];
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

export async function scanChangedFiles(
  targetBranch: string | undefined,
  devMode: boolean,
  tscannerVersion: string,
  groupBy: GroupMode,
): Promise<ScanResult> {
  const scanMode = targetBranch ? `changed files vs ${targetBranch}` : 'entire codebase';
  githubHelper.logInfo(`Scanning ${scanMode} (dev mode: ${devMode}, group by: ${groupBy})`);

  const executor: CliExecutor = devMode ? createDevModeExecutor() : createProdModeExecutor(tscannerVersion);

  const argsFile = ['check', '--json', '--exit-zero', ...(targetBranch ? ['--branch', targetBranch] : [])];
  const argsRule = [...argsFile, '--by-rule'];

  const [scanOutputFile, scanOutputRule] = await Promise.all([executor.execute(argsFile), executor.execute(argsRule)]);

  let scanDataFile: CliJsonOutputByFile;
  let scanDataRule: CliJsonOutputByRule;

  try {
    scanDataFile = JSON.parse(scanOutputFile) as CliJsonOutputByFile;
    scanDataRule = JSON.parse(scanOutputRule) as CliJsonOutputByRule;
  } catch {
    githubHelper.logError('Failed to parse scan output');
    githubHelper.logDebug(`Raw output: ${scanOutputFile.substring(0, 500)}`);
    throw new Error('Invalid scan output format');
  }

  githubHelper.logInfo(`Scan completed: ${scanDataFile.summary?.total_issues || 0} issues found`);

  const hasIssues = scanDataFile.files.length > 0;

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
      ruleGroupsByRule: [],
    };
  }

  githubHelper.logInfo('');
  githubHelper.logInfo('📊 Scan Results:');
  githubHelper.logInfo('');
  await executor.displayResults(argsFile.map((arg) => (arg === '--json' ? '--pretty' : arg)));

  const fileGroups: Array<{ file: string; issues: Issue[]; severity: Severity }> = scanDataFile.files.map(
    (fileData) => ({
      file: fileData.file,
      issues: fileData.issues.map((issue) => ({
        line: issue.line,
        column: issue.column,
        message: issue.message,
        lineText: issue.line_text,
        ruleName: issue.rule,
      })),
      severity: fileData.issues.some((i) => i.severity === 'error') ? Severity.Error : Severity.Warning,
    }),
  );

  fileGroups.sort((a, b) => {
    if (a.severity !== b.severity) {
      return a.severity === Severity.Error ? -1 : 1;
    }
    return b.issues.length - a.issues.length;
  });

  const ruleGroups: RuleGroup[] = fileGroups.map((fileGroup) => ({
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

  const ruleGroupsByRule: RuleGroup[] = scanDataRule.rules.map((ruleData) => {
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

  ruleGroupsByRule.sort((a, b) => {
    if (a.severity !== b.severity) {
      return a.severity === 'error' ? -1 : 1;
    }
    return b.issueCount - a.issueCount;
  });

  return {
    totalIssues: scanDataFile.summary.total_issues,
    totalErrors: scanDataFile.summary.errors,
    totalWarnings: scanDataFile.summary.warnings,
    totalFiles: scanDataFile.summary.total_files,
    totalRules: scanDataRule.rules.length,
    groupBy,
    ruleGroups,
    ruleGroupsByRule,
  };
}
