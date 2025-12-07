import type { CliOutputByFile, CliOutputByRule } from 'tscanner-common';
import { githubHelper } from '../../lib/actions-helper';

export function logFormattedResults(byFile: CliOutputByFile, byRule: CliOutputByRule): void {
  githubHelper.logInfo('Rules triggered:');
  githubHelper.logInfo('');
  for (const rule of byRule.rules ?? []) {
    const firstMessage = rule.issues?.[0]?.message ?? '';
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
        const rawLineText = issue.line_text ?? '';
        const lineText = rawLineText.length > 60 ? `${rawLineText.substring(0, 57)}...` : rawLineText;
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
