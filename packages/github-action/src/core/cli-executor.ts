import * as core from '@actions/core';
import * as exec from '@actions/exec';

export type CliExecutor = {
  execute: (args: string[]) => Promise<string>;
  displayResults: (args: string[]) => Promise<void>;
};

export function createDevModeExecutor(): CliExecutor {
  const workspaceRoot = process.env.GITHUB_WORKSPACE || process.cwd();
  const cliPath = `${workspaceRoot}/packages/cli/dist/main.js`;

  core.info(`Using local CLI: ${cliPath}`);

  return {
    async execute(args: string[]): Promise<string> {
      let output = '';
      await exec.exec('node', [cliPath, ...args], {
        silent: true,
        listeners: {
          stdout: (data: Buffer) => {
            output += data.toString();
          },
        },
        ignoreReturnCode: true,
      });
      return output;
    },

    async displayResults(args: string[]): Promise<void> {
      await exec.exec('node', [cliPath, ...args], {
        ignoreReturnCode: true,
      });
    },
  };
}

export function createProdModeExecutor(tscannerVersion: string): CliExecutor {
  const packageSpec = `tscanner@${tscannerVersion}`;

  core.info(`Using published tscanner from npm: ${packageSpec}`);

  return {
    async execute(args: string[]): Promise<string> {
      let output = '';
      await exec.exec('npx', [packageSpec, ...args], {
        silent: true,
        listeners: {
          stdout: (data: Buffer) => {
            output += data.toString();
          },
        },
        ignoreReturnCode: true,
      });
      return output;
    },

    async displayResults(args: string[]): Promise<void> {
      await exec.exec('npx', [packageSpec, ...args], {
        ignoreReturnCode: true,
      });
    },
  };
}
