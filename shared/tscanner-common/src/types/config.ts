import z from 'zod';
import { AiMode, AiProvider, severitySchema } from './enums';

const baseRuleConfigSchema = z.object({
  enabled: z.boolean().optional(),
  severity: severitySchema.optional(),
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

const builtinRuleConfigSchema = baseRuleConfigSchema.extend({
  options: z.record(z.string(), z.any()).optional(),
});

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

const aiRuleConfigSchema = baseRuleConfigSchema.extend({
  prompt: z.string(),
  message: z.string(),
  mode: aiModeSchema.optional(),
  timeout: z.number().optional(),
  options: z.any().optional(),
});

const rulesConfigSchema = z.object({
  builtin: z.record(z.string(), builtinRuleConfigSchema).optional(),
  regex: z.record(z.string(), regexRuleConfigSchema).optional(),
  script: z.record(z.string(), scriptRuleConfigSchema).optional(),
});

const aiProviderSchema = z.enum(AiProvider);

const aiConfigSchema = z.object({
  provider: aiProviderSchema.optional(),
  command: z.string().optional(),
});

const codeEditorConfigSchema = z.object({
  highlightErrors: z.boolean().optional(),
  highlightWarnings: z.boolean().optional(),
  highlightInfos: z.boolean().optional(),
  highlightHints: z.boolean().optional(),
  autoScanInterval: z.number().optional(),
  autoAiScanInterval: z.number().optional(),
});

const filesConfigSchema = z.object({
  include: z.array(z.string()),
  exclude: z.array(z.string()),
});

export const tscannerConfigSchema = z.object({
  $schema: z.string().optional(),
  rules: rulesConfigSchema,
  aiRules: z.record(z.string(), aiRuleConfigSchema),
  files: filesConfigSchema,
  ai: aiConfigSchema.optional(),
  codeEditor: codeEditorConfigSchema.optional(),
});

export type TscannerConfig = z.infer<typeof tscannerConfigSchema>;

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;
  const hasBuiltin = config.rules.builtin && Object.keys(config.rules.builtin).length > 0;
  const hasRegex = config.rules.regex && Object.keys(config.rules.regex).length > 0;
  const hasScript = config.rules.script && Object.keys(config.rules.script).length > 0;
  const hasAiRules = Object.keys(config.aiRules).length > 0;
  return Boolean(hasBuiltin || hasRegex || hasScript || hasAiRules);
}
