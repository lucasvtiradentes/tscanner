import { AiExecutionMode } from 'tscanner-common';
import { IntervalConfigKey, createIntervalWatcher } from './interval-watcher';

export { createConfigWatcher } from './config-watcher';
export { createFileWatcher } from './file/watcher';
export { createGitWatcher } from './git/watcher';

export const scanIntervalWatcher = createIntervalWatcher({
  name: 'regular',
  configKey: IntervalConfigKey.Scan,
  aiMode: AiExecutionMode.Ignore,
});

export const aiScanIntervalWatcher = createIntervalWatcher({
  name: 'AI',
  configKey: IntervalConfigKey.AiScan,
  aiMode: AiExecutionMode.Only,
});
