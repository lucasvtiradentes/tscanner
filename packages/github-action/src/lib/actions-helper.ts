import * as fs from 'node:fs';
import * as core from '@actions/core';
import * as exec from '@actions/exec';
import * as github from '@actions/github';

export type Octokit = ReturnType<typeof github.getOctokit>;
type GithubContext = typeof github.context;

type AnnotationProperties = {
  title?: string;
  file?: string;
  startLine?: number;
  endLine?: number;
  startColumn?: number;
  endColumn?: number;
};

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

  addAnnotationError(message: string, properties?: AnnotationProperties): void {
    core.error(message, properties);
  }

  addAnnotationWarning(message: string, properties?: AnnotationProperties): void {
    core.warning(message, properties);
  }

  writeSummary(content: string): void {
    const summaryFile = process.env.GITHUB_STEP_SUMMARY;
    if (summaryFile) {
      fs.appendFileSync(summaryFile, `${content}\n`);
    }
  }
}

export const githubHelper = new ActionsHelper();

const tmpLogStartTime = Date.now();
export function tmpLog(message: string): void {
  const elapsed = ((Date.now() - tmpLogStartTime) / 1000).toFixed(2);
  core.info(`[TMP +${elapsed}s] ${message}`);
}
