import { constants, accessSync, existsSync } from 'node:fs';
import { homedir } from 'node:os';
import { isAbsolute, join } from 'node:path';
import { DEV_SUFFIX } from 'src/common/scripts-constants';
import { PACKAGE_DISPLAY_NAME, PACKAGE_NAME, VSCODE_EXTENSION } from 'tscanner-common';
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

export const LOCATOR_SOURCE_LABELS: Record<LocatorSource, string> = {
  [LocatorSource.Dev]: DEV_SUFFIX,
  [LocatorSource.Settings]: 'settings',
  [LocatorSource.NodeModules]: 'local',
  [LocatorSource.Global]: 'global',
  [LocatorSource.Path]: 'PATH',
};

export const LOCATOR_SOURCE_LABELS_VERBOSE: Record<LocatorSource, string> = {
  [LocatorSource.Dev]: `${DEV_SUFFIX} (local rust build)`,
  [LocatorSource.Settings]: 'settings (user configured)',
  [LocatorSource.NodeModules]: 'node_modules (project dependency)',
  [LocatorSource.Global]: 'global (npm -g)',
  [LocatorSource.Path]: 'PATH (system)',
};

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
    const binaryName = `${PACKAGE_NAME}${ext}`;

    const rustBinaryPath = join(this.workspaceRoot, VSCODE_EXTENSION.paths.devBinaryRelative, binaryName);
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
