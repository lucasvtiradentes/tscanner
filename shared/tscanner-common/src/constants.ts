import constants from '../../../assets/constants.json';

export const PACKAGE_NAME = constants.shared.packageName;
export const PACKAGE_DISPLAY_NAME = constants.shared.packageDisplayName;
export const PACKAGE_DESCRIPTION = constants.shared.packageDescription;
export const CONFIG_DIR_NAME = constants.shared.configDirName;
export const CONFIG_FILE_NAME = constants.shared.configFileName;
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
export const CODE_EDITOR_DEFAULTS = constants.coreRust.defaults.codeEditor;

export const LspMethod = {
  Scan: constants.shared.lsp.methods.scan,
  ScanFile: constants.shared.lsp.methods.scanFile,
  ScanContent: constants.shared.lsp.methods.scanContent,
  ClearCache: constants.shared.lsp.methods.clearCache,
  GetRulesMetadata: constants.shared.lsp.methods.getRulesMetadata,
  FormatResults: constants.shared.lsp.methods.formatResults,
  AiProgress: constants.shared.lsp.methods.aiProgress,
} as const;

export const PLATFORM_TARGET_MAP: Record<string, string> = {
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc',
};

export const PLATFORM_PACKAGE_MAP: Record<string, string> = {
  'linux-x64': '@tscanner/cli-linux-x64',
  'linux-arm64': '@tscanner/cli-linux-arm64',
  'darwin-x64': '@tscanner/cli-darwin-x64',
  'darwin-arm64': '@tscanner/cli-darwin-arm64',
  'win32-x64': '@tscanner/cli-win32-x64',
};
