import { z } from 'zod';

const envSchema = z.object({
  GITHUB_HEAD_REF: z.string().optional(),
  GITHUB_REF_NAME: z.string().optional(),
  GITHUB_SHA: z.string().optional(),
  GITHUB_WORKSPACE: z.string().optional(),
  GITHUB_STEP_SUMMARY: z.string().optional(),
});

const parsed = envSchema.parse(process.env);

export const env = {
  branchName: parsed.GITHUB_HEAD_REF || parsed.GITHUB_REF_NAME || 'unknown',
  commitSha: parsed.GITHUB_SHA || 'unknown',
  workspaceRoot: parsed.GITHUB_WORKSPACE,
  summaryFile: parsed.GITHUB_STEP_SUMMARY,
};
