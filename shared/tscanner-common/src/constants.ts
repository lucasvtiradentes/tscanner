import constants from '../../../assets/constants.json';
import { PlatformKey } from './types';

export const PACKAGE_NAME = constants.shared.packageName;
export const PACKAGE_DEV_NAME = constants.shared.packageDevName;
export const PACKAGE_DISPLAY_NAME = constants.shared.packageDisplayName;
export const PACKAGE_DESCRIPTION = constants.shared.packageDescription;
export const CONFIG_DIR_NAME = constants.shared.configDirName;
export const CONFIG_FILE_NAME = constants.shared.configFileName;
export const LOCAL_CONFIG_FILE_NAME = constants.shared.localConfigFileName;
export const DEFAULT_TARGET_BRANCH = constants.shared.defaultTargetBranch;
export const LOG_BASENAME = constants.shared.logBasename;
export const LOG_TIMEZONE_OFFSET_HOURS = constants.shared.logTimezoneOffsetHours;
export const LOG_CONTEXT_WIDTH = constants.shared.logContextWidth;
export const IGNORE_COMMENT = constants.shared.ignoreComment;
export const IGNORE_NEXT_LINE_COMMENT = constants.shared.ignoreNextLineComment;
export const CONFIG_ERROR_PREFIX = constants.shared.configErrorPrefix;
export const JS_EXTENSIONS = constants.shared.extensions.javascript;
export const VSCODE_EXTENSION = constants.vscodeExtension;
export const DISPLAY_ICONS = constants.shared.icons;
export const LSP_CLIENT_ID = constants.shared.lsp.clientId;
export const REPO_URL = constants.shared.urls.repo;
export const REPO_BLOB_URL = constants.shared.urls.repoBlob;
export const VSCODE_SETTINGS_DEFAULTS = constants.vscodeExtension.defaults;

export const LspMethod = {
  Scan: constants.shared.lsp.methods.scan,
  ScanFile: constants.shared.lsp.methods.scanFile,
  ScanContent: constants.shared.lsp.methods.scanContent,
  ClearCache: constants.shared.lsp.methods.clearCache,
  GetRulesMetadata: constants.shared.lsp.methods.getRulesMetadata,
  FormatResults: constants.shared.lsp.methods.formatResults,
  ValidateConfig: constants.shared.lsp.methods.validateConfig,
  AiProgress: constants.shared.lsp.methods.aiProgress,
} as const;

export const PLATFORM_TARGET_MAP: Record<PlatformKey, string> = {
  [PlatformKey.LinuxX64]: 'x86_64-unknown-linux-gnu',
  [PlatformKey.LinuxArm64]: 'aarch64-unknown-linux-gnu',
  [PlatformKey.DarwinX64]: 'x86_64-apple-darwin',
  [PlatformKey.DarwinArm64]: 'aarch64-apple-darwin',
  [PlatformKey.Win32X64]: 'x86_64-pc-windows-msvc',
};

export const PLATFORM_PACKAGE_MAP: Record<PlatformKey, string> = {
  [PlatformKey.LinuxX64]: '@tscanner/cli-linux-x64',
  [PlatformKey.LinuxArm64]: '@tscanner/cli-linux-arm64',
  [PlatformKey.DarwinX64]: '@tscanner/cli-darwin-x64',
  [PlatformKey.DarwinArm64]: '@tscanner/cli-darwin-arm64',
  [PlatformKey.Win32X64]: '@tscanner/cli-win32-x64',
};
