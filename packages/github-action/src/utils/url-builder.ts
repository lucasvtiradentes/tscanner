import { createHash } from 'node:crypto';

function createFileHash(filePath: string): string {
  return createHash('sha256').update(filePath).digest('hex');
}

export function buildPrFileUrl(owner: string, repo: string, prNumber: number, filePath: string, line: number): string {
  const fileHash = createFileHash(filePath);
  return `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${fileHash}R${line}`;
}
