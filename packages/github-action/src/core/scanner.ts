import { AiExecutionMode, type CliOutputByFile, type CliOutputByRule, type GroupMode, Severity } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';
import { type CliExecutor, createDevModeExecutor, createProdModeExecutor } from './cli-executor';

function logFormattedResults(byFile: CliOutputByFile, byRule: CliOutputByRule): void {
  githubHelper.logInfo('Rules triggered:');
  githubHelper.logInfo('');
  for (const rule of byRule.rules) {
    const firstMessage = rule.issues[0]?.message || '';
    const truncatedMessage = firstMessage.length > 80 ? `${firstMessage.substring(0, 77)}...` : firstMessage;
    githubHelper.logInfo(`  ${rule.rule.padEnd(25)}: ${truncatedMessage}`);
  }
  githubHelper.logInfo('');
  githubHelper.logInfo('Issues grouped by file:');
  githubHelper.logInfo('');
  for (const file of byFile.files) {
    const ruleCount = new Set(file.issues.map((i) => i.rule)).size;
    githubHelper.logInfo(`${file.file} - ${file.issues.length} issues - ${ruleCount} rules`);
    githubHelper.logInfo('');
    const issuesByRule = new Map<string, typeof file.issues>();
    for (const issue of file.issues) {
      if (!issuesByRule.has(issue.rule)) {
        issuesByRule.set(issue.rule, []);
      }
      issuesByRule.get(issue.rule)!.push(issue);
    }
    for (const [ruleName, issues] of issuesByRule) {
      githubHelper.logInfo(`  ${ruleName} (${issues.length} issues)`);
      for (const issue of issues) {
        const severity = issue.severity === 'error' ? '✖' : '⚠';
        const lineText = issue.line_text.length > 60 ? `${issue.line_text.substring(0, 57)}...` : issue.line_text;
        githubHelper.logInfo(`    ${severity} ${issue.line}:${issue.column} -> ${lineText}`);
      }
      githubHelper.logInfo('');
    }
  }
  githubHelper.logInfo('Check summary:');
  githubHelper.logInfo('');
  githubHelper.logInfo(
    `  Issues: ${byFile.summary.total_issues} (${byFile.summary.errors} errors, ${byFile.summary.warnings} warnings)`,
  );
  githubHelper.logInfo(`  Files with issues: ${byFile.files.length}/${byFile.summary.total_files}`);
  githubHelper.logInfo(`  Triggered rules: ${byRule.rules.length}/${byFile.summary.total_enabled_rules}`);
}

function deriveOutputByRule(byFile: CliOutputByFile): CliOutputByRule {
  const ruleMap = new Map<string, { count: number; issues: CliOutputByRule['rules'][0]['issues'] }>();

  for (const fileEntry of byFile.files) {
    for (const issue of fileEntry.issues) {
      if (!ruleMap.has(issue.rule)) {
        ruleMap.set(issue.rule, { count: 0, issues: [] });
      }
      const ruleData = ruleMap.get(issue.rule)!;
      ruleData.count++;
      ruleData.issues.push({
        file: fileEntry.file,
        line: issue.line,
        column: issue.column,
        message: issue.message,
        severity: issue.severity,
        line_text: issue.line_text,
      });
    }
  }

  const rules: CliOutputByRule['rules'] = Array.from(ruleMap.entries()).map(([rule, data]) => ({
    rule,
    count: data.count,
    issues: data.issues,
  }));

  return {
    rules,
    summary: byFile.summary,
  };
}

export type ActionScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  filesWithIssues: number;
  totalRules: number;
  totalEnabledRules: number;
  groupBy: GroupMode;
  ruleGroups: RuleGroup[];
  ruleGroupsByRule: RuleGroup[];
};

type RuleGroup = {
  ruleName: string;
  severity: Severity;
  issueCount: number;
  fileCount: number;
  files: FileIssues[];
};

type FileIssues = {
  filePath: string;
  issues: Issue[];
};

type Issue = {
  line: number;
  column: number;
  message: string;
  lineText: string;
  ruleName?: string;
};

export type ScanOptions = {
  targetBranch?: string;
  devMode: boolean;
  tscannerVersion: string;
  groupBy: GroupMode;
  configPath: string;
  aiMode: AiExecutionMode;
};

function getAiModeArgs(aiMode: AiExecutionMode): string[] {
  switch (aiMode) {
    case AiExecutionMode.Include:
      return ['--include-ai'];
    case AiExecutionMode.Only:
      return ['--only-ai'];
    default:
      return [];
  }
}

function getAiModeLabel(aiMode: AiExecutionMode): string {
  switch (aiMode) {
    case AiExecutionMode.Include:
      return ' (with AI rules)';
    case AiExecutionMode.Only:
      return ' (AI rules only)';
    default:
      return '';
  }
}

export async function scanChangedFiles(options: ScanOptions): Promise<ActionScanResult> {
  const { targetBranch, devMode, tscannerVersion, groupBy, configPath, aiMode } = options;
  const scanMode = targetBranch ? `changed files vs ${targetBranch}` : 'entire codebase';
  githubHelper.logInfo(`Scanning [${scanMode}] group by: [${groupBy}]${getAiModeLabel(aiMode)}`);

  const executor: CliExecutor = devMode ? createDevModeExecutor() : createProdModeExecutor(tscannerVersion);

  const baseArgs = [
    'check',
    '--format=json',
    '--continue-on-error',
    '--config-path',
    configPath,
    ...(targetBranch ? ['--branch', targetBranch] : []),
    ...getAiModeArgs(aiMode),
    '--group-by=file',
  ];

  const scanOutputFile = await executor.execute(baseArgs);

  let scanDataFile: CliOutputByFile;
  let scanDataRule: CliOutputByRule;

  try {
    scanDataFile = JSON.parse(scanOutputFile) as CliOutputByFile;
    scanDataRule = deriveOutputByRule(scanDataFile);
  } catch (err) {
    githubHelper.logError(`Failed to parse scan output: ${err instanceof Error ? err.message : String(err)}`);
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
      filesWithIssues: 0,
      totalRules: 0,
      totalEnabledRules: scanDataFile.summary.total_enabled_rules,
      groupBy,
      ruleGroups: [],
      ruleGroupsByRule: [],
    };
  }

  githubHelper.logInfo('');
  githubHelper.logInfo('📊 Scan Results:');
  githubHelper.logInfo('');
  logFormattedResults(scanDataFile, scanDataRule);

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
    filesWithIssues: scanDataFile.files.length,
    totalRules: scanDataRule.rules.length,
    totalEnabledRules: scanDataFile.summary.total_enabled_rules,
    groupBy,
    ruleGroups,
    ruleGroupsByRule,
  };
}
