import type { TscannerConfig } from 'tscanner-common';

type RuleWithInclude = { include?: string[] };

function normalizePattern(pattern: string): string {
  if (pattern.startsWith('**/') || pattern.startsWith('{')) {
    return pattern;
  }
  return `**/${pattern}`;
}

function collectPatternsFromRules(
  rules: Record<string, RuleWithInclude> | RuleWithInclude[] | undefined,
  patterns: Set<string>,
): void {
  if (!rules) return;

  const values = Array.isArray(rules) ? rules : Object.values(rules);
  for (const rule of values) {
    if (rule.include) {
      for (const pattern of rule.include) {
        patterns.add(normalizePattern(pattern));
      }
    }
  }
}

export function buildWatchPattern(config: TscannerConfig | null): string | null {
  if (!config) {
    return null;
  }

  const patterns = new Set<string>();

  for (const pattern of config.files.include) {
    patterns.add(normalizePattern(pattern));
  }

  collectPatternsFromRules(config.rules.regex, patterns);
  collectPatternsFromRules(config.rules.script, patterns);
  collectPatternsFromRules(config.aiRules, patterns);

  if (patterns.size === 0) {
    return null;
  }

  const uniquePatterns = [...patterns];
  if (uniquePatterns.length === 1) {
    return uniquePatterns[0];
  }

  return `{${uniquePatterns.join(',')}}`;
}
