import z from 'zod';
import { AiMode, severitySchema } from './enums';

const baseRuleConfigSchema = z.object({
  severity: severitySchema.optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

const builtinRuleConfigSchema = baseRuleConfigSchema;

const regexRuleConfigSchema = baseRuleConfigSchema.extend({
  pattern: z.string(),
  message: z.string(),
});

const scriptRuleConfigSchema = baseRuleConfigSchema.extend({
  command: z.string(),
  message: z.string(),
  timeout: z.number().optional(),
  options: z.any().optional(),
});

const aiModeSchema = z.enum(AiMode);

const aiRuleClassificationSchema = z.enum(['code-checkable', 'guidance-only', 'unsupported']);

const aiRuleSourceConfigSchema = baseRuleConfigSchema.extend({
  path: z.string(),
  ignore: z.array(z.string()).optional(),
  id: z.string().optional(),
  message: z.string().optional(),
  mode: aiModeSchema.optional(),
  classification: aiRuleClassificationSchema.optional(),
  timeout: z.number().optional(),
  options: z.any().optional(),
});

const rulesConfigSchema = z.object({
  builtin: z.record(z.string(), builtinRuleConfigSchema).optional(),
  regex: z.record(z.string(), regexRuleConfigSchema).optional(),
  script: z.record(z.string(), scriptRuleConfigSchema).optional(),
});

const filesConfigSchema = z.object({
  include: z.array(z.string()),
  exclude: z.array(z.string()),
});

export const tscannerConfigSchema = z.object({
  $schema: z.string().optional(),
  rules: rulesConfigSchema,
  aiRules: z.array(aiRuleSourceConfigSchema),
  files: filesConfigSchema,
});

export type TscannerConfig = z.infer<typeof tscannerConfigSchema>;

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;
  const hasBuiltin = config.rules.builtin && Object.keys(config.rules.builtin).length > 0;
  const hasRegex = config.rules.regex && Object.keys(config.rules.regex).length > 0;
  const hasScript = config.rules.script && Object.keys(config.rules.script).length > 0;
  const hasAiRules = config.aiRules.length > 0;
  return Boolean(hasBuiltin || hasRegex || hasScript || hasAiRules);
}
