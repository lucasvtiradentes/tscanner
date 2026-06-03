import z from 'zod';
import { RuleCategory, RuleType, ruleOptionTypeSchema, severitySchema } from './enums';

const ruleOptionSchema = z.object({
  name: z.string(),
  description: z.string(),
  type: ruleOptionTypeSchema,
  default: z.any(),
  minimum: z.number().optional(),
  items: z.string().optional(),
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
  options: z.array(ruleOptionSchema).optional(),
});

export type RuleOption = z.infer<typeof ruleOptionSchema>;
export type RuleMetadata = z.infer<typeof ruleMetadataSchema>;
