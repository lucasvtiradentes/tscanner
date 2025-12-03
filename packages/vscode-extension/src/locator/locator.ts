import { constants, accessSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import * as vscode from 'vscode';
import { findInGlobalModules } from './global-modules';
import { findInNodeModules } from './node-modules';
import { findInPath } from './path';

export type LocatorResult = {
  path: string;
  source: 'settings' | 'node_modules' | 'global' | 'path';
} | null;

export class Locator {
  constructor(private workspaceRoot: string | undefined) {}

  async locate(): Promise<LocatorResult> {
    const settingsPath = this.getSettingsPath();
    if (settingsPath) {
      return { path: settingsPath, source: 'settings' };
    }

    if (this.workspaceRoot) {
      const localPath = await findInNodeModules(this.workspaceRoot);
      if (localPath) {
        return { path: localPath, source: 'node_modules' };
      }
    }

    const globalPath = await findInGlobalModules();
    if (globalPath) {
      return { path: globalPath, source: 'global' };
    }

    const pathBinary = await findInPath();
    if (pathBinary) {
      return { path: pathBinary, source: 'path' };
    }

    return null;
  }

  private getSettingsPath(): string | null {
    const config = vscode.workspace.getConfiguration('tscanner');
    const binPath = config.get<string>('lsp.bin');

    if (!binPath || binPath.trim() === '') {
      return null;
    }

    const resolvedPath = this.resolvePath(binPath);

    if (!existsSync(resolvedPath)) {
      vscode.window.showWarningMessage(
        `TScanner: Configured binary path does not exist: ${resolvedPath}\n\nFalling back to auto-detection.`,
      );
      return null;
    }

    try {
      accessSync(resolvedPath, constants.X_OK);
    } catch {
      vscode.window.showWarningMessage(
        `TScanner: Configured binary is not executable: ${resolvedPath}\n\nTry: chmod +x "${resolvedPath}"`,
      );
    }

    return resolvedPath;
  }

  private resolvePath(binPath: string): string {
    if (binPath.startsWith('~')) {
      return binPath.replace('~', process.env.HOME ?? '');
    }
    if (!binPath.startsWith('/') && this.workspaceRoot) {
      return join(this.workspaceRoot, binPath);
    }
    return binPath;
  }
}
