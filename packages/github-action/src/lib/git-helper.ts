import * as exec from '@actions/exec';

export async function fetchBranch(targetBranch: string): Promise<void> {
  const branchName = targetBranch.replace('origin/', '');
  await exec.exec('git', ['fetch', 'origin', branchName]);
}

export async function getCommitMessage(commitSha: string): Promise<string> {
  let output = '';
  await exec.exec('git', ['log', '-1', '--pretty=%s', commitSha], {
    silent: true,
    listeners: {
      stdout: (data: Buffer) => {
        output += data.toString();
      },
    },
  });
  return output.trim();
}
