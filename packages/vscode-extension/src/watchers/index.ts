import { AiExecutionMode } from 'tscanner-common';
import { Command } from '../common/lib/vscode-utils';
import { IntervalConfigKey, createIntervalWatcher } from './interval-watcher';

export { createConfigWatcher } from './config-watcher';
export { createFileWatcher } from './file/watcher';
export { createGitWatcher } from './git/watcher';

export const scanIntervalWatcher = createIntervalWatcher({
  name: 'regular',
  configKey: IntervalConfigKey.Scan,
  command: Command.RefreshIssues,
  aiMode: AiExecutionMode.Ignore,
});

export const aiScanIntervalWatcher = createIntervalWatcher({
  name: 'AI',
  configKey: IntervalConfigKey.AiScan,
  command: Command.RefreshAiIssues,
});
