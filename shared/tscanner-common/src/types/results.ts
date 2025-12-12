import z from 'zod';
import { IssueRuleType, issueRuleTypeSchema, severitySchema } from './enums';

const issueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  end_column: z.number(),
  message: z.string(),
  severity: severitySchema,
  line_text: z.string().optional(),
  category: z.string().optional(),
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
  regular_rules_duration_ms: z.number(),
  ai_rules_duration_ms: z.number(),
  total_files: z.number(),
  cached_files: z.number(),
  scanned_files: z.number(),
  warnings: z.array(z.string()).optional().default([]),
});

export type FileResult = z.infer<typeof fileResultSchema>;
export type ContentScanResult = z.infer<typeof contentScanResultSchema>;
export type ScanResult = z.infer<typeof scanResultSchema>;
