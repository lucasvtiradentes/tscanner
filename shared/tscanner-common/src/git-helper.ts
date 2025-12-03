import { execSync } from 'node:child_process';
import type { ModifiedLineRange } from './schemas';

export class GitHelper {
  private static toGitPath(filePath: string): string {
    return filePath.replace(/\\/g, '/');
  }

  private static execGit(cmd: string, cwd: string): [string | null, Error | null] {
    try {
      const output = execSync(cmd, { cwd, encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] });
      return [output, null];
    } catch (error) {
      return [null, error as Error];
    }
  }

  private static parseDiffToLines(diff: string): Set<number> {
    const addedLines = new Set<number>();
    const lines = diff.split('\n');
    let currentLine = 0;

    for (const line of lines) {
      const hunkMatch = line.match(/^@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@/);
      if (hunkMatch) {
        currentLine = Number.parseInt(hunkMatch[1], 10);
        continue;
      }

      if (line.startsWith('+') && !line.startsWith('+++')) {
        addedLines.add(currentLine);
        currentLine++;
      } else if (!line.startsWith('-')) {
        currentLine++;
      }
    }

    return addedLines;
  }

  private static linesToRanges(lines: Set<number>): ModifiedLineRange[] {
    const sortedLines = Array.from(lines).sort((a, b) => a - b);
    const ranges: ModifiedLineRange[] = [];

    if (sortedLines.length > 0) {
      let rangeStart = sortedLines[0];
      let rangeCount = 1;

      for (let i = 1; i < sortedLines.length; i++) {
        if (sortedLines[i] === sortedLines[i - 1] + 1) {
          rangeCount++;
        } else {
          ranges.push({ startLine: rangeStart, lineCount: rangeCount });
          rangeStart = sortedLines[i];
          rangeCount = 1;
        }
      }
      ranges.push({ startLine: rangeStart, lineCount: rangeCount });
    }

    return ranges;
  }

  static fetchBranch(targetBranch: string, cwd: string): void {
    const branchName = targetBranch.replace('origin/', '');
    GitHelper.execGit(`git fetch origin ${branchName}`, cwd);
  }

  static branchExists(workspaceRoot: string, branchName: string): boolean {
    const [, error] = GitHelper.execGit(`git rev-parse --verify "${branchName}"`, workspaceRoot);
    return error === null;
  }

  static getAllBranches(workspaceRoot: string): string[] {
    const [output, error] = GitHelper.execGit('git branch -a', workspaceRoot);
    if (error || !output) {
      return [];
    }

    return output
      .split('\n')
      .map((line: string) => line.trim())
      .filter((line: string) => line && !line.includes('HEAD'))
      .map((line: string) => line.replace(/^\*\s+/, ''))
      .map((line: string) => {
        if (line.startsWith('remotes/origin/')) {
          return `origin/${line.replace('remotes/origin/', '')}`;
        }
        return line;
      });
  }

  static getModifiedLineRanges(workspaceRoot: string, filePath: string, compareBranch: string): ModifiedLineRange[] {
    const gitPath = GitHelper.toGitPath(filePath);
    const [diff, error] = GitHelper.execGit(`git diff -w "${compareBranch}" -- "${gitPath}"`, workspaceRoot);
    if (error || !diff) {
      return [];
    }
    return GitHelper.linesToRanges(GitHelper.parseDiffToLines(diff));
  }
}
