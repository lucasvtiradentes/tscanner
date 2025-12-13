import * as vscode from 'vscode';
import { logger } from '../../common/lib/logger';
import { createCheckoutHandler, createCommitHandler } from './handler';

type GitAPI = {
  repositories: GitRepository[];
  onDidOpenRepository: vscode.Event<GitRepository>;
};

type GitRepository = {
  state: {
    HEAD?: { name?: string };
  };
  onDidCommit: vscode.Event<void>;
  onDidCheckout: vscode.Event<void>;
};

function getGitAPI(): GitAPI | null {
  const gitExtension = vscode.extensions.getExtension('vscode.git');
  if (!gitExtension) {
    logger.debug('Git extension not found');
    return null;
  }

  if (!gitExtension.isActive) {
    logger.debug('Git extension not active');
    return null;
  }

  return gitExtension.exports.getAPI(1);
}

function setupRepositoryListeners(repo: GitRepository, disposables: vscode.Disposable[]): void {
  const handleCommit = createCommitHandler();
  const handleCheckout = createCheckoutHandler();

  disposables.push(repo.onDidCommit(handleCommit));
  disposables.push(repo.onDidCheckout(handleCheckout));

  logger.debug(`Git listeners attached for branch: ${repo.state.HEAD?.name ?? 'unknown'}`);
}

export async function createGitWatcher(): Promise<vscode.Disposable | null> {
  const gitAPI = getGitAPI();
  if (!gitAPI) {
    return null;
  }

  const disposables: vscode.Disposable[] = [];

  for (const repo of gitAPI.repositories) {
    setupRepositoryListeners(repo, disposables);
  }

  disposables.push(
    gitAPI.onDidOpenRepository((newRepo) => {
      logger.debug('New repository opened, attaching git listeners');
      setupRepositoryListeners(newRepo, disposables);
    }),
  );

  return vscode.Disposable.from(...disposables);
}
