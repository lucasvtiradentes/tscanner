import * as core from '@actions/core';
import * as exec from '@actions/exec';
import * as github from '@actions/github';

export type Octokit = ReturnType<typeof github.getOctokit>;
export type GithubContext = typeof github.context;

class ActionsHelper {
  getOctokit(token: string): Octokit {
    return github.getOctokit(token);
  }

  getContext(): GithubContext {
    return github.context;
  }

  logInfo(message: string): void {
    core.info(message);
  }

  logWarning(message: string): void {
    core.warning(message);
  }

  logError(message: string): void {
    core.error(message);
  }

  logDebug(message: string): void {
    core.debug(message);
  }

  setFailed(message: string): void {
    core.setFailed(message);
  }

  getInput(name: string, options?: core.InputOptions): string {
    return core.getInput(name, options);
  }

  async execCommand(command: string, args: string[], options?: exec.ExecOptions): Promise<number> {
    return exec.exec(command, args, options);
  }

  async execCommandWithOutput(command: string, args: string[], silent = true): Promise<string> {
    let output = '';
    await exec.exec(command, args, {
      silent,
      listeners: {
        stdout: (data: Buffer) => {
          output += data.toString();
        },
      },
    });
    return output.trim();
  }
}

export const githubHelper = new ActionsHelper();
