import z from 'zod';
import { severitySchema } from './enums';

const displayIssueSchema = z.object({
  line: z.number(),
  column: z.number(),
  message: z.string(),
  lineText: z.string().optional(),
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
