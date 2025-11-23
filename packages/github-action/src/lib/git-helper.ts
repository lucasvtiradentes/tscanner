import { githubHelper } from './actions-helper';

class GitHelper {
  async fetchBranch(targetBranch: string): Promise<void> {
    const branchName = targetBranch.replace('origin/', '');
    await githubHelper.execCommand('git', ['fetch', 'origin', branchName]);
  }

  async getCommitMessage(commitSha: string): Promise<string> {
    return githubHelper.execCommandWithOutput('git', ['log', '-1', '--pretty=%s', commitSha]);
  }
}

export const gitHelper = new GitHelper();
