import * as fs from 'node:fs';
import * as path from 'node:path';
import { CONFIG_FILE_NAME } from '../constants';
import { githubHelper } from '../lib/actions-helper';

export function validateConfigFiles(configPath: string): void {
  const configFilePath = path.join(configPath, CONFIG_FILE_NAME);

  if (fs.existsSync(configFilePath)) {
    githubHelper.logInfo(`Using ${CONFIG_FILE_NAME} from ${configPath}`);
  } else {
    githubHelper.logWarning(`No config file found in ${configPath}. Using default configuration.`);
  }
}
