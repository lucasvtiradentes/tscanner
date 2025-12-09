import { AiExecutionMode } from 'tscanner-common';
import { createIntervalWatcher } from './interval-watcher';

export { createConfigWatcher } from './config-watcher';
export { createFileWatcher } from './file-watcher';

export const scanIntervalWatcher = createIntervalWatcher({
  name: 'regular',
  configKey: 'scanInterval',
  aiMode: AiExecutionMode.Ignore,
});

export const aiScanIntervalWatcher = createIntervalWatcher({
  name: 'AI',
  configKey: 'aiScanInterval',
  aiMode: AiExecutionMode.Only,
});
