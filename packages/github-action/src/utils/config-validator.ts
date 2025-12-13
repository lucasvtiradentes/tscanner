import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { CONFIG_FILE_NAME } from 'tscanner-common';
import { githubHelper } from '../lib/actions-helper';

export function validateConfigFiles(configPath: string): void {
  const configFilePath = join(configPath, CONFIG_FILE_NAME);

  if (existsSync(configFilePath)) {
    githubHelper.logInfo(`Using ${CONFIG_FILE_NAME} from ${configPath}`);
  } else {
    githubHelper.logWarning(`No config file found in ${configPath}. Using default configuration.`);
  }
}
