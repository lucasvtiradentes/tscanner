import { githubHelper } from './actions-helper';

class GitHelper {
  async fetchBranch(targetBranch: string) {
    const branchName = targetBranch.replace('origin/', '');
    await githubHelper.execCommand('git', ['fetch', 'origin', branchName]);
  }

  async getCommitMessage(commitSha: string): Promise<string> {
    try {
      const message = await githubHelper.execCommandWithOutput('git', ['log', '-1', '--pretty=%s', commitSha]);
      return message || '';
    } catch {
      githubHelper.logWarning(`Failed to get commit message for ${commitSha}`);
      return '';
    }
  }
}

export const gitHelper = new GitHelper();
