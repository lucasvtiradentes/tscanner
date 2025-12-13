import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import * as cache from '@actions/cache';
import { githubHelper } from '../lib/actions-helper';

const CACHE_DIR = join(process.env.HOME || '~', '.cache', 'tscanner');

function computeConfigHash(configPath: string): string {
  const hash = createHash('sha256');
  const fullPath = join(process.cwd(), configPath);

  if (!existsSync(fullPath)) {
    return 'no-config';
  }

  const files = readdirSync(fullPath, { recursive: true }) as string[];

  for (const file of files.sort()) {
    const filePath = join(fullPath, file);
    const stat = statSync(filePath);

    if (stat.isFile()) {
      const content = readFileSync(filePath);
      hash.update(file);
      hash.update(content);
    }
  }

  return hash.digest('hex').substring(0, 16);
}

export type CacheResult = {
  restored: boolean;
  cacheKey?: string;
};

export async function restoreCache(configPath: string): Promise<CacheResult> {
  const configHash = computeConfigHash(configPath);
  const runnerOs = process.env.RUNNER_OS || 'Linux';

  const primaryKey = `tscanner-${runnerOs}-${configHash}`;
  const restoreKeys = [`tscanner-${runnerOs}-`];

  githubHelper.logInfo(`Cache key: ${primaryKey}`);

  try {
    const cacheKey = await cache.restoreCache([CACHE_DIR], primaryKey, restoreKeys);

    if (cacheKey) {
      githubHelper.logInfo(`Cache restored from key: ${cacheKey}`);
      return { restored: true, cacheKey };
    }

    githubHelper.logInfo('Cache miss - no existing cache found');
    return { restored: false };
  } catch (error) {
    githubHelper.logWarning(`Cache restore failed: ${error instanceof Error ? error.message : String(error)}`);
    return { restored: false };
  }
}

export async function saveCache(configPath: string): Promise<void> {
  const configHash = computeConfigHash(configPath);
  const runnerOs = process.env.RUNNER_OS || 'Linux';
  const key = `tscanner-${runnerOs}-${configHash}`;

  if (!existsSync(CACHE_DIR)) {
    githubHelper.logInfo('No cache directory to save');
    return;
  }

  try {
    await cache.saveCache([CACHE_DIR], key);
    githubHelper.logInfo(`Cache saved with key: ${key}`);
  } catch (error) {
    if (error instanceof Error && error.message.includes('already exists')) {
      githubHelper.logInfo('Cache already exists, skipping save');
      return;
    }
    githubHelper.logWarning(`Cache save failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}
