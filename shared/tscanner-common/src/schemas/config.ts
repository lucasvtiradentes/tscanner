import { z } from 'zod';
import { AiExecutionMode, AiMode, AiProvider, GroupMode, severitySchema } from '../constants';

const baseRuleConfigSchema = z.object({
  enabled: z.boolean().optional(),
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

const aiRuleConfigSchema = baseRuleConfigSchema.extend({
  prompt: z.string(),
  message: z.string(),
  mode: aiModeSchema.optional(),
  timeout: z.number().optional(),
});

const rulesConfigSchema = z.object({
  builtin: z.record(z.string(), builtinRuleConfigSchema).optional(),
  regex: z.record(z.string(), regexRuleConfigSchema).optional(),
  script: z.record(z.string(), scriptRuleConfigSchema).optional(),
});

const aiProviderSchema = z.enum(AiProvider);

const aiConfigSchema = z.object({
  provider: aiProviderSchema.optional(),
  timeout: z.number().optional(),
  command: z.string().optional(),
});

const codeEditorConfigSchema = z.object({
  highlightErrors: z.boolean().optional(),
  highlightWarnings: z.boolean().optional(),
  scanIntervalSeconds: z.number().optional(),
  aiScanIntervalSeconds: z.number().optional(),
});

const aiExecutionModeSchema = z.enum(AiExecutionMode);

const cliConfigSchema = z.object({
  groupBy: z.enum(GroupMode).optional(),
  noCache: z.boolean().optional(),
  showSettings: z.boolean().optional(),
  showIssueSeverity: z.boolean().optional(),
  showIssueSourceLine: z.boolean().optional(),
  showIssueRuleName: z.boolean().optional(),
  showIssueDescription: z.boolean().optional(),
  showSummary: z.boolean().optional(),
  aiMode: aiExecutionModeSchema.optional(),
});

const filesConfigSchema = z.object({
  include: z.array(z.string()).optional(),
  exclude: z.array(z.string()).optional(),
});

export const tscannerConfigSchema = z.object({
  rules: rulesConfigSchema.optional(),
  aiRules: z.record(z.string(), aiRuleConfigSchema).optional(),
  files: filesConfigSchema.optional(),
  ai: aiConfigSchema.optional(),
  codeEditor: codeEditorConfigSchema.optional(),
  cli: cliConfigSchema.optional(),
});

export type TscannerConfig = z.infer<typeof tscannerConfigSchema>;

export function hasConfiguredRules(config: TscannerConfig | null): boolean {
  if (!config) return false;
  const hasBuiltin = config.rules?.builtin && Object.keys(config.rules.builtin).length > 0;
  const hasRegex = config.rules?.regex && Object.keys(config.rules.regex).length > 0;
  const hasScript = config.rules?.script && Object.keys(config.rules.script).length > 0;
  const hasAiRules = config.aiRules && Object.keys(config.aiRules).length > 0;
  return !!(hasBuiltin || hasRegex || hasScript || hasAiRules);
}
