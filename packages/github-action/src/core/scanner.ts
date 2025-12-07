import { AiExecutionMode, type CliOutputByFile, type CliOutputByRule, type GroupMode, Severity } from 'tscanner-common';
import { githubHelper, tmpLog } from '../lib/actions-helper';
import { type CliExecutor, createDevModeExecutor, createProdModeExecutor } from './cli-executor';

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
  tmpLog('scanChangedFiles() started');
  const { targetBranch, devMode, tscannerVersion, groupBy, configPath, aiMode } = options;
  const scanMode = targetBranch ? `changed files vs ${targetBranch}` : 'entire codebase';
  githubHelper.logInfo(`Scanning [${scanMode}] group by: [${groupBy}]${getAiModeLabel(aiMode)}`);

  tmpLog(`creating executor (devMode=${devMode})`);
  const executor: CliExecutor = devMode ? createDevModeExecutor() : createProdModeExecutor(tscannerVersion);
  tmpLog('executor created');

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

  tmpLog('starting CLI execution (single run)');
  const scanOutputFile = await executor.execute(baseArgs);
  tmpLog('CLI execution done');

  let scanDataFile: CliOutputByFile;
  let scanDataRule: CliOutputByRule;

  try {
    scanDataFile = JSON.parse(scanOutputFile) as CliOutputByFile;
    tmpLog('deriving ByRule from ByFile');
    scanDataRule = deriveOutputByRule(scanDataFile);
  } catch (err) {
    githubHelper.logError(`Failed to parse scan output: ${err instanceof Error ? err.message : String(err)}`);
    githubHelper.logDebug(`Raw output: ${scanOutputFile.substring(0, 500)}`);
    throw new Error('Invalid scan output format');
  }

  githubHelper.logInfo(`Scan completed: ${scanDataFile.summary?.total_issues || 0} issues found`);
  tmpLog('JSON parsing done');

  const hasIssues = scanDataFile.files.length > 0;

  if (!hasIssues) {
    githubHelper.logInfo('No issues found');
    tmpLog('scanChangedFiles() returning (no issues)');
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
  tmpLog('displayResults() starting');
  const displayArgs = baseArgs
    .map((arg) => (arg === '--format=json' ? '--format=pretty' : arg))
    .filter((arg) => arg !== '--include-ai' && arg !== '--only-ai');
  await executor.displayResults(displayArgs);
  tmpLog('displayResults() done');

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

  tmpLog('scanChangedFiles() returning');
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
