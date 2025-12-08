import { z } from 'zod';

const cliSummarySchema = z.object({
  total_files: z.number(),
  total_issues: z.number(),
  errors: z.number(),
  warnings: z.number(),
  total_enabled_rules: z.number(),
});

const cliIssueByFileSchema = z.object({
  rule: z.string(),
  severity: z.string(),
  line: z.number(),
  column: z.number(),
  message: z.string(),
  line_text: z.string(),
});

const cliFileEntrySchema = z.object({
  file: z.string(),
  issues: z.array(cliIssueByFileSchema),
});

const cliOutputByFileSchema = z.object({
  files: z.array(cliFileEntrySchema),
  summary: cliSummarySchema,
});

const cliIssueByRuleSchema = z.object({
  file: z.string(),
  line: z.number(),
  column: z.number(),
  message: z.string(),
  severity: z.string(),
  line_text: z.string(),
});

const cliRuleEntrySchema = z.object({
  rule: z.string(),
  count: z.number(),
  issues: z.array(cliIssueByRuleSchema),
});

const cliOutputByRuleSchema = z.object({
  rules: z.array(cliRuleEntrySchema),
  summary: cliSummarySchema,
});

export type CliOutputByFile = z.infer<typeof cliOutputByFileSchema>;
export type CliOutputByRule = z.infer<typeof cliOutputByRuleSchema>;
