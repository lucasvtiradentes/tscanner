import {
  type CliOutputByFile,
  type CliOutputByRule,
  type DisplayIssue,
  type FileIssues,
  type RuleGroup,
  Severity,
} from 'tscanner-common';

export function deriveOutputByRule(byFile: CliOutputByFile): CliOutputByRule {
  const ruleMap = new Map<string, { count: number; issues: CliOutputByRule['rules'][0]['issues'] }>();

  for (const fileEntry of byFile.files) {
    for (const issue of fileEntry.issues) {
      let ruleData = ruleMap.get(issue.rule);
      if (ruleData === undefined) {
        ruleData = { count: 0, issues: [] };
        ruleMap.set(issue.rule, ruleData);
      }
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

export function transformToRuleGroupsByFile(byFile: CliOutputByFile): RuleGroup[] {
  const fileGroups: Array<{ file: string; issues: DisplayIssue[]; severity: Severity }> = byFile.files.map(
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

  return fileGroups.map((fileGroup) => ({
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

export function transformToRuleGroupsByRule(byRule: CliOutputByRule): RuleGroup[] {
  const ruleGroups: RuleGroup[] = byRule.rules.map((ruleData) => {
    const fileMap = new Map<string, DisplayIssue[]>();

    for (const issue of ruleData.issues) {
      let fileIssues = fileMap.get(issue.file);
      if (fileIssues === undefined) {
        fileIssues = [];
        fileMap.set(issue.file, fileIssues);
      }
      fileIssues.push({
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

  return ruleGroups;
}
