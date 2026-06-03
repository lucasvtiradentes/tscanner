import { z } from 'zod';

const envSchema = z.object({
  CI: z.string().optional(),
  GITHUB_ACTIONS: z.string().optional(),
  TSCANNER_DEV_BIN_DIR: z.string().optional(),
});

const parsed = envSchema.parse(process.env);

export const scriptEnv = {
  devBinDir: parsed.TSCANNER_DEV_BIN_DIR,
  isCi: Boolean(parsed.CI) || Boolean(parsed.GITHUB_ACTIONS),
};
