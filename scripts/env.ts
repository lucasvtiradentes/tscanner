import { z } from 'zod';

const envSchema = z.object({
  CI: z.string().optional(),
  GITHUB_ACTIONS: z.string().optional(),
});

const parsed = envSchema.parse(process.env);

export const scriptEnv = {
  isCi: Boolean(parsed.CI) || Boolean(parsed.GITHUB_ACTIONS),
};
