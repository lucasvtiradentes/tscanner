import {
  AiExecutionMode,
  type CliOutputByFile,
  type CliOutputByRule,
  type GroupMode,
  type RuleGroup,
} from 'tscanner-common';
import { githubHelper } from '../../lib/actions-helper';
import { type CliExecutor, createDevModeExecutor, createProdModeExecutor } from '../cli-executor';
import { logFormattedResults } from './scanner-logger';
import { deriveOutputByRule, transformToRuleGroupsByFile, transformToRuleGroupsByRule } from './scanner-transforms';

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

  const ruleGroups = transformToRuleGroupsByFile(scanDataFile);
  const ruleGroupsByRule = transformToRuleGroupsByRule(scanDataRule);

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
