import * as vscode from 'vscode';
import { logger } from './logger';

type GitExtension = {
  getAPI(version: 1): GitAPI;
};

type GitAPI = {
  repositories: Repository[];
};

type Repository = {
  state: {
    HEAD: { name?: string; commit?: string } | undefined;
    refs: { name: string; commit?: string }[];
  };
  diffWithHEAD(path?: string): Promise<Change[]>;
  diffBetween(ref1: string, ref2: string): Promise<Change[]>;
};

type Change = {
  uri: vscode.Uri;
  status: number;
};

export class VscodeGit {
  private static gitApi: GitAPI | null = null;

  private static getGitAPI(): GitAPI | null {
    if (!VscodeGit.gitApi) {
      const gitExtension = vscode.extensions.getExtension<GitExtension>('vscode.git');
      if (gitExtension?.isActive) {
        VscodeGit.gitApi = gitExtension.exports.getAPI(1);
      }
    }
    return VscodeGit.gitApi;
  }

  private static getRepository(workspaceRoot: string): Repository | null {
    const api = VscodeGit.getGitAPI();
    if (!api) return null;

    const workspaceUri = vscode.Uri.file(workspaceRoot);
    return (
      api.repositories.find((repo) => {
        return workspaceUri.fsPath.startsWith(repo.state.HEAD?.commit || '');
      }) ||
      api.repositories[0] ||
      null
    );
  }

  static getCurrentBranch(workspaceRoot: string) {
    const repo = VscodeGit.getRepository(workspaceRoot);
    if (!repo) {
      logger.error('Git repository not found');
      return null;
    }

    return repo.state.HEAD?.name || null;
  }

  static async getChangedFiles(workspaceRoot: string, compareBranch: string): Promise<Set<string>> {
    const repo = VscodeGit.getRepository(workspaceRoot);
    if (!repo) {
      logger.error('Git repository not found');
      return new Set();
    }

    try {
      const startTime = Date.now();

      const changedFromHead = await repo.diffWithHEAD();
      const uncommittedFiles = new Set(changedFromHead.map((change) => vscode.workspace.asRelativePath(change.uri)));

      const currentBranch = repo.state.HEAD?.name || 'HEAD';
      const committedChanges = await repo.diffBetween(compareBranch, currentBranch);
      const committedFiles = new Set(committedChanges.map((change) => vscode.workspace.asRelativePath(change.uri)));

      const allFiles = new Set([...uncommittedFiles, ...committedFiles]);

      const elapsed = Date.now() - startTime;
      logger.debug(
        `Git diff via VSCode API: ${uncommittedFiles.size} uncommitted + ${committedFiles.size} committed = ${allFiles.size} total (${elapsed}ms)`,
      );

      return allFiles;
    } catch (error) {
      logger.error(`Failed to get changed files vs ${compareBranch}: ${error}`);
      return new Set();
    }
  }
}
