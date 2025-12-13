export enum ScanTrigger {
  ManualCommand = 'manual-command',
  FileSave = 'file-save',
  Interval = 'interval',
  Startup = 'startup',
  ConfigChange = 'config-change',
  GitCommit = 'git-commit',
  GitCheckout = 'git-checkout',
  ConfigLocationChange = 'config-location-change',
  ScanModeChange = 'scan-mode-change',
}

export function shouldUseCache(trigger: ScanTrigger): boolean {
  switch (trigger) {
    case ScanTrigger.ManualCommand:
      return false;
    case ScanTrigger.FileSave:
      return true;
    case ScanTrigger.Interval:
      return true;
    case ScanTrigger.Startup:
      return false;
    case ScanTrigger.ConfigChange:
      return false;
    case ScanTrigger.GitCommit:
      return false;
    case ScanTrigger.GitCheckout:
      return false;
    case ScanTrigger.ConfigLocationChange:
      return false;
    case ScanTrigger.ScanModeChange:
      return false;
  }
}
