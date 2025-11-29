import { z } from 'zod';
import { CustomRuleType, RuleCategory, RuleType } from './constants';

export const severitySchema = z.enum(['error', 'warning']);

export const scanParamsSchema = z.object({
  root: z.string(),
  config: z.any().optional(),
  branch: z.string().optional(),
});

export const scanFileParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
});

export const scanContentParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
  content: z.string(),
  config: z.any().optional(),
});

export const getRulesMetadataParamsSchema = z.object({});

export const clearCacheParamsSchema = z.object({});

export const issueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  end_column: z.number(),
  message: z.string(),
  severity: severitySchema,
  line_text: z.string().optional(),
});

export const fileResultSchema = z.object({
  file: z.string(),
  issues: z.array(issueSchema),
});

export const scanResultSchema = z.object({
  files: z.array(fileResultSchema),
  total_issues: z.number(),
  duration_ms: z.number(),
  total_files: z.number(),
  cached_files: z.number(),
  scanned_files: z.number(),
});

export const ruleMetadataSchema = z.object({
  name: z.string(),
  displayName: z.string(),
  description: z.string(),
  ruleType: z.enum([RuleType.Ast, RuleType.Regex]),
  defaultSeverity: severitySchema,
  defaultEnabled: z.boolean(),
  category: z.enum([RuleCategory.TypeSafety, RuleCategory.CodeQuality, RuleCategory.Style, RuleCategory.Performance]),
  typescriptOnly: z.boolean().optional(),
  equivalentEslintRule: z.string().optional(),
  equivalentBiomeRule: z.string().optional(),
});

export const modifiedLineRangeSchema = z.object({
  startLine: z.number(),
  lineCount: z.number(),
});

export const builtinRuleConfigSchema = z.object({
  enabled: z.boolean().optional(),
  severity: severitySchema.optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

export const customRuleConfigSchema = z.object({
  type: z.enum([CustomRuleType.Regex, CustomRuleType.Script, CustomRuleType.Ai]),
  pattern: z.string().optional(),
  script: z.string().optional(),
  prompt: z.string().optional(),
  message: z.string(),
  severity: severitySchema.optional(),
  enabled: z.boolean().optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

export const lspConfigSchema = z.object({
  errors: z.boolean().optional().default(true),
  warnings: z.boolean().optional().default(true),
});

export const cliGroupBySchema = z.enum(['file', 'rule']);

export const cliConfigSchema = z.object({
  groupBy: cliGroupBySchema.optional().default('file'),
  noCache: z.boolean().optional().default(false),
  showSeverity: z.boolean().optional().default(true),
  showSourceLine: z.boolean().optional().default(true),
  showRuleName: z.boolean().optional().default(true),
  showDescription: z.boolean().optional().default(false),
  showSummaryAtFooter: z.boolean().optional().default(true),
});

export const filesConfigSchema = z.object({
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

export const tscannerConfigSchema = z.object({
  lsp: lspConfigSchema.optional(),
  cli: cliConfigSchema.optional(),
  builtinRules: z.record(z.string(), builtinRuleConfigSchema).optional(),
  customRules: z.record(z.string(), customRuleConfigSchema).optional(),
  files: filesConfigSchema.optional(),
});

export type ScanParams = z.infer<typeof scanParamsSchema>;
export type ScanFileParams = z.infer<typeof scanFileParamsSchema>;
export type ScanContentParams = z.infer<typeof scanContentParamsSchema>;
export type GetRulesMetadataParams = z.infer<typeof getRulesMetadataParamsSchema>;
export type ClearCacheParams = z.infer<typeof clearCacheParamsSchema>;
export type Issue = z.infer<typeof issueSchema>;
export type FileResult = z.infer<typeof fileResultSchema>;
export type ScanResult = z.infer<typeof scanResultSchema>;
export type RuleMetadata = z.infer<typeof ruleMetadataSchema>;
export type ModifiedLineRange = z.infer<typeof modifiedLineRangeSchema>;
export type BuiltinRuleConfig = z.infer<typeof builtinRuleConfigSchema>;
export type CustomRuleConfig = z.infer<typeof customRuleConfigSchema>;
export type LspConfig = z.infer<typeof lspConfigSchema>;
export type CliGroupBy = z.infer<typeof cliGroupBySchema>;
export type CliConfig = z.infer<typeof cliConfigSchema>;
export type FilesConfig = z.infer<typeof filesConfigSchema>;
export type TscannerConfig = z.infer<typeof tscannerConfigSchema>;

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;
  return !!(
    (config.builtinRules && Object.keys(config.builtinRules).length > 0) ||
    (config.customRules && Object.keys(config.customRules).length > 0)
  );
}
