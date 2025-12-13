import * as vscode from 'vscode';
import { getLspClient } from '../../scanner/client';
import { logger } from './logger';

export async function validateConfigAndNotify(configPath: string): Promise<boolean> {
  try {
    const lspClient = getLspClient();
    if (!lspClient) {
      logger.warn('LSP client not available for config validation');
      return false;
    }

    const result = await lspClient.validateConfig(configPath);

    if (!result.valid) {
      const errorMessage = `Config validation failed:\n${result.errors.join('\n')}`;
      logger.error(errorMessage);
      await vscode.window.showErrorMessage(`TScanner: ${result.errors[0] ?? 'Invalid configuration'}`);
      return false;
    }

    if (result.warnings.length > 0) {
      logger.warn(`Config warnings:\n${result.warnings.join('\n')}`);
      await vscode.window.showWarningMessage(`TScanner: ${result.warnings[0]}`);
    }

    logger.info('Config validation passed');
    return true;
  } catch (error) {
    logger.error(`Config validation error: ${error}`);
    return false;
  }
}
