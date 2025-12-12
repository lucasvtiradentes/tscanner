import { CODE_EDITOR_DEFAULTS, DISPLAY_ICONS, type TscannerConfig } from 'tscanner-common';
import { getConfigDirLabel } from '../common/lib/config-manager';
import { type BinaryInfo, LOCATOR_SOURCE_LABELS, LocatorSource } from '../locator';

function getAiProviderLabel(config: TscannerConfig | null): string {
  if (!config?.ai?.provider) {
    return 'None';
  }
  const provider = config.ai.provider;
  return provider.charAt(0).toUpperCase() + provider.slice(1);
}

function getActiveRulesLabel(config: TscannerConfig | null): string {
  if (!config) return 'None';

  const parts: string[] = [];

  const builtin = config.rules?.builtin ? Object.keys(config.rules.builtin).length : 0;
  const regex = config.rules?.regex ? Object.keys(config.rules.regex).length : 0;
  const script = config.rules?.script ? Object.keys(config.rules.script).length : 0;
  const ai = config.aiRules ? Object.keys(config.aiRules).length : 0;

  if (builtin > 0) parts.push(`${DISPLAY_ICONS.builtin} ${builtin}`);
  if (regex > 0) parts.push(`${DISPLAY_ICONS.regex} ${regex}`);
  if (script > 0) parts.push(`${DISPLAY_ICONS.script} ${script}`);
  if (ai > 0) parts.push(`${DISPLAY_ICONS.ai} ${ai}`);

  return parts.length > 0 ? parts.join('  ') : 'None';
}

function formatAutoInterval(seconds: number): string {
  if (seconds === 0) return 'off';
  if (seconds < 60) return `${seconds}s`;
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m`;
}

function getScanSettingsLabel(config: TscannerConfig | null): string {
  const useScanCache = config?.codeEditor?.useScanCache ?? CODE_EDITOR_DEFAULTS.useScanCache;
  const autoScanInterval = config?.codeEditor?.autoScanInterval ?? CODE_EDITOR_DEFAULTS.autoScanInterval;

  const cacheLabel = useScanCache ? 'on' : 'off';
  const autoLabel = formatAutoInterval(autoScanInterval);

  return `cache ${cacheLabel}, auto ${autoLabel}`;
}

function getAiScanSettingsLabel(config: TscannerConfig | null): string {
  const useAiScanCache = config?.codeEditor?.useAiScanCache ?? CODE_EDITOR_DEFAULTS.useAiScanCache;
  const autoAiScanInterval = config?.codeEditor?.autoAiScanInterval ?? CODE_EDITOR_DEFAULTS.autoAiScanInterval;

  const cacheLabel = useAiScanCache ? 'on' : 'off';
  const autoLabel = formatAutoInterval(autoAiScanInterval);

  return `cache ${cacheLabel}, auto ${autoLabel}`;
}

export function buildConfiguredTooltip(
  configDir: string | null,
  config: TscannerConfig | null,
  binaryInfo: BinaryInfo,
): string {
  const configLabel = getConfigDirLabel(configDir);
  const configSource = LOCATOR_SOURCE_LABELS[binaryInfo.source];
  const binaryLabel =
    binaryInfo.version && binaryInfo.source !== LocatorSource.Dev
      ? `${configSource} (v${binaryInfo.version})`
      : configSource;
  const aiProviderLabel = getAiProviderLabel(config);
  const activeRulesLabel = getActiveRulesLabel(config);
  const scanSettingsLabel = getScanSettingsLabel(config);
  const aiScanSettingsLabel = getAiScanSettingsLabel(config);

  const tooltipLines = [
    `Binary: ${binaryLabel}`,
    `Config: ${configLabel}`,
    `Active Rules: ${activeRulesLabel}`,
    `Scan: ${scanSettingsLabel}`,
    `AI Scan: ${aiScanSettingsLabel}`,
    config?.ai?.provider && `AI Provider: ${aiProviderLabel}`,
  ];

  return tooltipLines.join('\n');
}
