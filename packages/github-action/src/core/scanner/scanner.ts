import { existsSync, readFileSync, unlinkSync } from 'node:fs';
import { join } from 'node:path';
import {
  type AiExecutionMode,
  type CliOutputByFile,
  type CliOutputByRule,
  GroupMode,
  type RuleGroup,
  type RulesBreakdown,
  buildCheckArgs,
} from 'tscanner-common';
import { githubHelper } from '../../lib/actions-helper';
import { type CliExecutor, createDevModeExecutor, createProdModeExecutor } from '../cli-executor';
import { deriveOutputByRule, transformToRuleGroupsByFile, transformToRuleGroupsByRule } from './scanner-transforms';

export type ActionScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  cachedFiles: number;
  scannedFiles: number;
  filesWithIssues: number;
  totalRules: number;
  totalEnabledRules: number;
  enabledRulesBreakdown: RulesBreakdown;
  groupBy: GroupMode;
  ruleGroups: RuleGroup[];
  ruleGroupsByRule: RuleGroup[];
  scanErrors: string[];
};

export type ScanOptions = {
  targetBranch?: string;
  devMode: boolean;
  tscannerVersion: string;
  groupBy: GroupMode;
  configPath: string;
  aiMode: AiExecutionMode;
  noCache: boolean;
};

export async function scanChangedFiles(options: ScanOptions): Promise<ActionScanResult> {
  const { targetBranch, devMode, tscannerVersion, groupBy, configPath, aiMode, noCache } = options;

  const executor: CliExecutor = devMode ? createDevModeExecutor() : createProdModeExecutor(tscannerVersion);

  const jsonOutputFile = join(process.cwd(), 'tscanner-results.json');

  const baseArgs = buildCheckArgs({
    jsonOutput: jsonOutputFile,
    continueOnError: true,
    configPath,
    branch: targetBranch,
    aiMode,
    groupBy: GroupMode.File,
    noCache,
  });

  await executor.execute(baseArgs);

  let scanDataFile: CliOutputByFile;
  let scanDataRule: CliOutputByRule;

  try {
    const jsonContent = readFileSync(jsonOutputFile, 'utf-8');
    scanDataFile = JSON.parse(jsonContent) as CliOutputByFile;
    scanDataRule = deriveOutputByRule(scanDataFile);
    unlinkSync(jsonOutputFile);
  } catch (err) {
    githubHelper.logError(`Failed to parse scan output: ${err instanceof Error ? err.message : String(err)}`);
    if (existsSync(jsonOutputFile)) {
      try {
        const rawContent = readFileSync(jsonOutputFile, 'utf-8');
        githubHelper.logDebug(`Raw output: ${rawContent.substring(0, 500)}`);
        unlinkSync(jsonOutputFile);
      } catch {}
    }
    throw new Error('Invalid scan output format');
  }

  const hasIssues = scanDataFile.files.length > 0;

  if (!hasIssues) {
    return {
      totalIssues: 0,
      totalErrors: 0,
      totalWarnings: 0,
      totalFiles: scanDataFile.summary.total_files,
      cachedFiles: scanDataFile.summary.cached_files,
      scannedFiles: scanDataFile.summary.scanned_files,
      filesWithIssues: 0,
      totalRules: 0,
      totalEnabledRules: scanDataFile.summary.total_enabled_rules,
      enabledRulesBreakdown: scanDataFile.summary.enabled_rules_breakdown,
      groupBy,
      ruleGroups: [],
      ruleGroupsByRule: [],
      scanErrors: scanDataFile.summary.scan_errors ?? [],
    };
  }

  const ruleGroups = transformToRuleGroupsByFile(scanDataFile);
  const ruleGroupsByRule = transformToRuleGroupsByRule(scanDataRule);

  return {
    totalIssues: scanDataFile.summary.total_issues,
    totalErrors: scanDataFile.summary.errors,
    totalWarnings: scanDataFile.summary.warnings,
    totalFiles: scanDataFile.summary.total_files,
    cachedFiles: scanDataFile.summary.cached_files,
    scannedFiles: scanDataFile.summary.scanned_files,
    filesWithIssues: scanDataFile.files.length,
    totalRules: scanDataRule.rules.length,
    totalEnabledRules: scanDataFile.summary.total_enabled_rules,
    enabledRulesBreakdown: scanDataFile.summary.enabled_rules_breakdown,
    groupBy,
    ruleGroups,
    ruleGroupsByRule,
    scanErrors: scanDataFile.summary.scan_errors ?? [],
  };
}
