import { constants, accessSync, existsSync } from 'node:fs';
import { homedir } from 'node:os';
import { isAbsolute, join } from 'node:path';
import { PACKAGE_DISPLAY_NAME } from 'tscanner-common';
import * as vscode from 'vscode';
import { IS_DEV } from '../common/constants';
import { ExtensionConfigKey, getExtensionConfig } from '../common/state/extension-config';
import { findInGlobalModules } from './global-modules';
import { findInNodeModules } from './node-modules';
import { findInPath } from './path';

export enum LocatorSource {
  Dev = 'dev',
  Settings = 'settings',
  NodeModules = 'node_modules',
  Global = 'global',
  Path = 'path',
}

type LocatorResult = {
  path: string;
  args?: string[];
  source: LocatorSource;
} | null;

export class Locator {
  constructor(private workspaceRoot: string | undefined) {}

  async locate(): Promise<LocatorResult> {
    const devResult = this.findDevBinary();
    if (devResult) {
      return devResult;
    }

    const settingsPath = this.getSettingsPath();
    if (settingsPath) {
      return { path: settingsPath, source: LocatorSource.Settings };
    }

    if (this.workspaceRoot) {
      const localPath = await findInNodeModules(this.workspaceRoot);
      if (localPath) {
        return { path: localPath, source: LocatorSource.NodeModules };
      }
    }

    const globalPath = await findInGlobalModules();
    if (globalPath) {
      return { path: globalPath, source: LocatorSource.Global };
    }

    const pathBinary = await findInPath();
    if (pathBinary) {
      return { path: pathBinary, source: LocatorSource.Path };
    }

    return null;
  }

  private findDevBinary(): LocatorResult {
    if (!IS_DEV || !this.workspaceRoot) {
      return null;
    }

    const ext = process.platform === 'win32' ? '.exe' : '';
    const binaryName = `tscanner${ext}`;

    const rustBinaryPath = join(this.workspaceRoot, 'packages', 'rust-core', 'target', 'release', binaryName);
    if (existsSync(rustBinaryPath)) {
      return { path: rustBinaryPath, source: LocatorSource.Dev };
    }

    return null;
  }

  private getSettingsPath(): string | null {
    const binPath = getExtensionConfig(ExtensionConfigKey.LspBin);

    if (!binPath || binPath.trim() === '') {
      return null;
    }

    const resolvedPath = this.resolvePath(binPath);

    if (!existsSync(resolvedPath)) {
      vscode.window.showWarningMessage(
        `${PACKAGE_DISPLAY_NAME}: Configured binary path does not exist: ${resolvedPath}\n\nFalling back to auto-detection.`,
      );
      return null;
    }

    try {
      accessSync(resolvedPath, constants.X_OK);
    } catch {
      vscode.window.showWarningMessage(
        `${PACKAGE_DISPLAY_NAME}: Configured binary is not executable: ${resolvedPath}\n\nTry: chmod +x "${resolvedPath}"`,
      );
    }

    return resolvedPath;
  }

  private resolvePath(binPath: string): string {
    if (binPath.startsWith('~')) {
      return binPath.replace('~', homedir());
    }
    if (!isAbsolute(binPath) && this.workspaceRoot) {
      return join(this.workspaceRoot, binPath);
    }
    return binPath;
  }
}
