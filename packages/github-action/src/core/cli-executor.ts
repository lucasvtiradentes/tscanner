import { PACKAGE_NAME } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';

export type CliExecutor = {
  execute: (args: string[]) => Promise<string>;
};

export function createDevModeExecutor(): CliExecutor {
  const workspaceRoot = process.env.GITHUB_WORKSPACE || process.cwd();
  const cliPath = `${workspaceRoot}/packages/cli/dist/main.js`;

  githubHelper.logInfo(`Using local CLI: ${cliPath}`);

  return {
    async execute(args: string[]): Promise<string> {
      let output = '';
      await githubHelper.execCommand('node', [cliPath, ...args], {
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
  };
}

export function createProdModeExecutor(tscannerVersion: string): CliExecutor {
  const packageSpec = `${PACKAGE_NAME}@${tscannerVersion}`;

  githubHelper.logInfo(`Using published ${PACKAGE_NAME} from npm: ${packageSpec}`);

  return {
    async execute(args: string[]): Promise<string> {
      let output = '';
      await githubHelper.execCommand('npx', [packageSpec, ...args], {
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
  };
}
