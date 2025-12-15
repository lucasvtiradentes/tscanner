import { PACKAGE_DISPLAY_NAME, REPO_URL } from 'tscanner-common';
import * as vscode from 'vscode';
import { getLspClient } from '../../scanner/client';
import { parseConfigError } from '../../scanner/utils';
import { getStatusBarName } from '../constants';
import { StoreKey, extensionStore } from '../state/extension-store';
import { logger } from './logger';
import { ToastKind, showToastMessage } from './vscode-utils';

export async function validateConfigAndNotify(configPath: string): Promise<boolean> {
  try {
    const lspClient = getLspClient();
    if (!lspClient) {
      logger.warn('LSP client not available for config validation');
      return false;
    }

    logger.info(`Calling LSP validateConfig with path: ${configPath}`);
    const result = await lspClient.validateConfig(configPath);
    logger.info(
      `LSP validateConfig result: valid=${result.valid}, errors=${result.errors.length}, warnings=${result.warnings.length}`,
    );

    if (!result.valid) {
      const errorMessage = `Config validation failed:\n${result.errors.join('\n')}`;
      logger.error(errorMessage);

      const firstError = result.errors[0];

      if (firstError?.startsWith('Config file not found:')) {
        logger.info('Config file not found - treating as unconfigured');
        extensionStore.set(StoreKey.InvalidConfigFields, []);
        extensionStore.set(StoreKey.ConfigError, null);
        return true;
      }

      const configError = firstError ? parseConfigError(firstError) : null;

      if (configError) {
        extensionStore.set(StoreKey.InvalidConfigFields, configError.invalidFields);
        extensionStore.set(StoreKey.ConfigError, null);
        logger.warn(`Config has invalid fields: [${configError.invalidFields.join(', ')}]. Entering degraded mode.`);

        const fieldsText = configError.invalidFields.join(', ');
        const message = `${PACKAGE_DISPLAY_NAME}: Config has incompatible fields [${fieldsText}]. Some features may be disabled.`;

        vscode.window.showWarningMessage(message, 'Learn More', 'Dismiss').then((selection) => {
          if (selection === 'Learn More') {
            vscode.env.openExternal(vscode.Uri.parse(`${REPO_URL}/tree/main/packages/vscode-extension#updating`));
          }
        });

        return true;
      }

      extensionStore.set(StoreKey.InvalidConfigFields, []);
      extensionStore.set(StoreKey.ConfigError, result.errors[0] ?? 'Invalid configuration');
      showToastMessage(ToastKind.Error, `TScanner: ${result.errors[0] ?? 'Invalid configuration'}`);
      return false;
    }

    let invalidFields: string[] = [];
    for (const warning of result.warnings) {
      const configError = parseConfigError(warning);
      if (configError) {
        invalidFields = [...invalidFields, ...configError.invalidFields];
      }
    }

    extensionStore.set(StoreKey.InvalidConfigFields, invalidFields);
    extensionStore.set(StoreKey.ConfigError, null);

    if (result.warnings.length > 0) {
      logger.warn(`Config warnings:\n${result.warnings.join('\n')}`);
      vscode.window.showWarningMessage(`${getStatusBarName()}: ${result.warnings[0]}`);
    }

    logger.info('Config validation passed');
    return true;
  } catch (error) {
    logger.error(`Config validation error: ${error}`);
    extensionStore.set(StoreKey.InvalidConfigFields, []);
    extensionStore.set(StoreKey.ConfigError, String(error));
    return false;
  }
}
