import z from 'zod';
import {
  AiExecutionMode,
  IssueRuleType,
  RuleCategory,
  RuleType,
  issueRuleTypeSchema,
  severitySchema,
} from '../constants';

const scanParamsSchema = z.object({
  root: z.string(),
  config: z.any().optional(),
  branch: z.string().optional(),
  ai_mode: z.nativeEnum(AiExecutionMode).optional(),
});

const scanFileParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
});

const scanContentParamsSchema = z.object({
  root: z.string(),
  file: z.string(),
  content: z.string(),
  config: z.any().optional(),
});

const issueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  end_column: z.number(),
  message: z.string(),
  severity: severitySchema,
  line_text: z.string().optional(),
  is_ai: z.boolean().optional(),
  rule_type: issueRuleTypeSchema.optional().default(IssueRuleType.Builtin),
});

const fileResultSchema = z.object({
  file: z.string(),
  issues: z.array(issueSchema),
});

const contentScanResultSchema = z.object({
  file: z.string(),
  issues: z.array(issueSchema),
  related_files: z.array(z.string()).optional().default([]),
});

const scanResultSchema = z.object({
  files: z.array(fileResultSchema),
  total_issues: z.number(),
  duration_ms: z.number(),
  total_files: z.number(),
  cached_files: z.number(),
  scanned_files: z.number(),
});

const ruleMetadataSchema = z.object({
  name: z.string(),
  displayName: z.string(),
  description: z.string(),
  ruleType: z.nativeEnum(RuleType),
  defaultSeverity: severitySchema,
  defaultEnabled: z.boolean(),
  category: z.nativeEnum(RuleCategory),
  typescriptOnly: z.boolean().optional(),
  equivalentEslintRule: z.string().optional(),
  equivalentBiomeRule: z.string().optional(),
});

const modifiedLineRangeSchema = z.object({
  startLine: z.number(),
  lineCount: z.number(),
});

export type ScanParams = z.infer<typeof scanParamsSchema>;
export type ScanFileParams = z.infer<typeof scanFileParamsSchema>;
export type ScanContentParams = z.infer<typeof scanContentParamsSchema>;
export type Issue = z.infer<typeof issueSchema>;
export type FileResult = z.infer<typeof fileResultSchema>;
export type ContentScanResult = z.infer<typeof contentScanResultSchema>;
export type ScanResult = z.infer<typeof scanResultSchema>;
export type RuleMetadata = z.infer<typeof ruleMetadataSchema>;
export type ModifiedLineRange = z.infer<typeof modifiedLineRangeSchema>;

const displayIssueSchema = z.object({
  line: z.number(),
  column: z.number(),
  message: z.string(),
  lineText: z.string(),
  ruleName: z.string().optional(),
});

const fileIssuesSchema = z.object({
  filePath: z.string(),
  issues: z.array(displayIssueSchema),
});

const ruleGroupSchema = z.object({
  ruleName: z.string(),
  severity: severitySchema,
  issueCount: z.number(),
  fileCount: z.number(),
  files: z.array(fileIssuesSchema),
});

export type DisplayIssue = z.infer<typeof displayIssueSchema>;
export type FileIssues = z.infer<typeof fileIssuesSchema>;
export type RuleGroup = z.infer<typeof ruleGroupSchema>;
