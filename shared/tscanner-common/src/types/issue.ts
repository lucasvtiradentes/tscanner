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

export type Issue = z.infer<typeof issueSchema>;
