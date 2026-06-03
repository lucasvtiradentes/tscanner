import z from 'zod';
import { AiExecutionMode } from './enums';

const previousAiIssueSchema = z.object({
  rule: z.string(),
  file: z.string(),
  line: z.number(),
  column: z.number(),
  message: z.string(),
  line_text: z.string().optional(),
});

const scanParamsSchema = z.object({
  root: z.string(),
  config: z.any().optional(),
  config_dir: z.string().optional(),
  branch: z.string().optional(),
  staged: z.boolean().optional(),
  ai_mode: z.nativeEnum(AiExecutionMode).optional(),
  no_cache: z.boolean().optional(),
  previous_ai_issues: z.array(previousAiIssueSchema).optional(),
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
  config_dir: z.string().optional(),
  branch: z.string().optional(),
  uncommitted: z.boolean().optional(),
});

const modifiedLineRangeSchema = z.object({
  startLine: z.number(),
  lineCount: z.number(),
});

export type ScanParams = z.infer<typeof scanParamsSchema>;
export type PreviousAiIssue = z.infer<typeof previousAiIssueSchema>;
export type ScanFileParams = z.infer<typeof scanFileParamsSchema>;
export type ScanContentParams = z.infer<typeof scanContentParamsSchema>;
export type ModifiedLineRange = z.infer<typeof modifiedLineRangeSchema>;
