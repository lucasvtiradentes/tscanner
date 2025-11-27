import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import { homedir } from 'node:os';
import { join } from 'node:path';
import {
  BINARY_BASE_NAME,
  EXTENSION_ID_DEV,
  PLATFORM_TARGET_MAP,
  getBinaryName,
} from '../packages/vscode-extension/src/common/constants';
import {
  CONTEXT_PREFIX,
  DEV_SUFFIX,
  EXTENSION_DISPLAY_NAME,
  VIEW_ID,
  addDevLabel,
  addDevSuffix,
  buildLogFilename,
} from '../packages/vscode-extension/src/common/scripts-constants';

const logger = console;

const SCRIPT_DIR = __dirname;
const ROOT_DIR = join(SCRIPT_DIR, '..');
const EXTENSION_DIR = join(ROOT_DIR, 'packages', 'vscode-extension');
const CORE_DIR = join(ROOT_DIR, 'packages', 'core');

async function main() {
  if (process.env.CI || process.env.GITHUB_ACTIONS) {
    logger.log('[VSCode] Skipping local installation in CI environment');
    process.exit(0);
  }

  await setupLocalDistDirectory();
  await copyExtensionFiles();
  await copyBinaries();
  await patchExtensionCode();
  await writePackageJson();
  await copyMetaFiles();
  await copyToVSCodeExtensions();
  await printSuccessMessage();
}

main();

async function setupLocalDistDirectory() {
  logger.log('[VSCode] Step 1/7 - Setting up local dist directory...');
  const targetDir = getLocalDistDirectory();

  if (existsSync(targetDir)) {
    rmSync(targetDir, { recursive: true });
  }

  mkdirSync(targetDir, { recursive: true });
}

async function copyExtensionFiles() {
  logger.log('[VSCode] Step 2/7 - Copying extension files...');
  const targetDir = getLocalDistDirectory();

  copyRecursive(join(EXTENSION_DIR, 'out'), join(targetDir, 'out'));
  copyRecursive(join(EXTENSION_DIR, 'resources'), join(targetDir, 'resources'));
}

async function copyBinaries() {
  logger.log('[VSCode] Step 3/7 - Copying Rust binary...');
  const targetDir = getLocalDistDirectory();
  const coreTargetDir = join(CORE_DIR, 'target', 'release');
  const outBinariesDir = join(targetDir, 'out', 'binaries');

  const platformInfo = getPlatformInfo();
  if (!platformInfo) {
    logger.log('[VSCode]    ⚠️  Unsupported platform - skipping');
    return;
  }

  const { platform, npmPlatform } = platformInfo;
  const sourcePath = join(coreTargetDir, getBinaryName(BINARY_BASE_NAME));

  if (!existsSync(sourcePath)) {
    logger.log('[VSCode]    ⚠️  Binary not found - skipping (not built yet)');
    return;
  }

  mkdirSync(outBinariesDir, { recursive: true });

  const targetBinary = join(outBinariesDir, `${BINARY_BASE_NAME}-${npmPlatform}${platform === 'win32' ? '.exe' : ''}`);

  copyFileSync(sourcePath, targetBinary);
  logger.log(`[VSCode]    ✅ Copied binary for ${npmPlatform}`);
}

async function patchExtensionCode() {
  logger.log('[VSCode] Step 4/7 - Patching extension code...');
  const targetDir = getLocalDistDirectory();
  const extensionJsPath = join(targetDir, 'out', 'extension.js');

  const extensionJs = readFileSync(extensionJsPath, 'utf8');
  const isDevUnminified = /var IS_DEV = false;/;
  const logFileProd = buildLogFilename(false);
  const logFileDev = buildLogFilename(true);
  const logFilePattern = new RegExp(logFileProd.replace('.', '\\.'), 'g');
  const statusBarPattern = new RegExp(`${EXTENSION_DISPLAY_NAME}:`, 'g');

  let patchedExtensionJs = extensionJs;

  if (isDevUnminified.test(patchedExtensionJs)) {
    patchedExtensionJs = patchedExtensionJs.replace(isDevUnminified, 'var IS_DEV = true;');
  } else {
    patchedExtensionJs = patchedExtensionJs.replace(logFilePattern, logFileDev);
    patchedExtensionJs = patchedExtensionJs.replace(statusBarPattern, `${addDevLabel(EXTENSION_DISPLAY_NAME)}:`);
  }

  writeFileSync(extensionJsPath, patchedExtensionJs);
}

async function writePackageJson() {
  logger.log('[VSCode] Step 5/7 - Writing package.json...');
  const targetDir = getLocalDistDirectory();
  const packageJsonPath = join(EXTENSION_DIR, 'package.json');
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
  const modifiedPackageJson = applyDevTransformations(packageJson);

  writeFileSync(join(targetDir, 'package.json'), JSON.stringify(modifiedPackageJson, null, 2));
}

async function copyMetaFiles() {
  logger.log('[VSCode] Step 6/7 - Copying meta files...');
  const targetDir = getLocalDistDirectory();

  const licensePath = join(EXTENSION_DIR, 'LICENSE');
  if (existsSync(licensePath)) {
    copyFileSync(licensePath, join(targetDir, 'LICENSE'));
  }

  const readmePath = join(EXTENSION_DIR, 'README.md');
  if (existsSync(readmePath)) {
    copyFileSync(readmePath, join(targetDir, 'README.md'));
  }
}

async function copyToVSCodeExtensions() {
  logger.log('[VSCode] Step 7/7 - Installing to VSCode extensions...');
  const sourceDir = getLocalDistDirectory();
  const targetDir = getVSCodeExtensionsDirectory();

  if (existsSync(targetDir)) {
    rmSync(targetDir, { recursive: true });
  }

  mkdirSync(targetDir, { recursive: true });
  copyRecursive(sourceDir, targetDir);
}

async function printSuccessMessage() {
  const editor = detectCurrentEditor();
  const targetDir = getVSCodeExtensionsDirectory();
  const editorName = EDITOR_DISPLAY_NAMES[editor];

  logger.log(`\n[VSCode] ✅ Extension installed to: ${targetDir}`);
  logger.log(`[VSCode]    Extension ID: ${EXTENSION_ID_DEV}`);
  logger.log(`[VSCode]    Editor detected: ${editorName}`);
  logger.log(`\n[VSCode] 🔄 Reload ${editorName} to activate the extension:`);
  logger.log('[VSCode]    - Press Ctrl+Shift+P');
  logger.log(`[VSCode]    - Type "Reload Window" and press Enter\n`);
}

function getLocalDistDirectory(): string {
  return join(EXTENSION_DIR, 'dist-dev');
}

enum Editor {
  VSCode = 'vscode',
  Cursor = 'cursor',
  VSCodium = 'vscodium',
  Windsurf = 'windsurf',
}

const EDITOR_DISPLAY_NAMES: Record<Editor, string> = {
  [Editor.VSCode]: 'VSCode',
  [Editor.Cursor]: 'Cursor',
  [Editor.VSCodium]: 'VSCodium',
  [Editor.Windsurf]: 'Windsurf',
};

function detectCurrentEditor(): Editor {
  const vscodeEnv = process.env.VSCODE_PID || process.env.TERM_PROGRAM;
  if (vscodeEnv === 'vscode') return Editor.VSCode;

  const cursorPath = join(homedir(), '.cursor', 'extensions');
  const windsurfPath = join(homedir(), '.windsurf', 'extensions');
  const vscodiumPath =
    process.platform === 'darwin'
      ? join(homedir(), '.vscode-oss', 'extensions')
      : join(homedir(), '.config', 'VSCodium', 'extensions');
  const vscodePath = join(homedir(), '.vscode', 'extensions');

  if (existsSync(cursorPath) && !existsSync(vscodePath)) return Editor.Cursor;
  if (existsSync(windsurfPath) && !existsSync(vscodePath)) return Editor.Windsurf;
  if (existsSync(vscodiumPath) && !existsSync(vscodePath)) return Editor.VSCodium;

  return Editor.VSCode;
}

function getEditorExtensionsPath(editor: Editor): string {
  const paths: Record<Editor, string> = {
    [Editor.VSCode]: join(homedir(), '.vscode', 'extensions'),
    [Editor.Cursor]: join(homedir(), '.cursor', 'extensions'),
    [Editor.Windsurf]: join(homedir(), '.windsurf', 'extensions'),
    [Editor.VSCodium]:
      process.platform === 'darwin'
        ? join(homedir(), '.vscode-oss', 'extensions')
        : join(homedir(), '.config', 'VSCodium', 'extensions'),
  };
  return paths[editor];
}

function getVSCodeExtensionsDirectory(): string {
  const editor = detectCurrentEditor();
  return join(getEditorExtensionsPath(editor), EXTENSION_ID_DEV);
}

function getPlatformInfo(): { platform: string; npmPlatform: string } | null {
  const platform = process.platform;
  const arch = process.arch;
  const key = `${platform}-${arch}`;
  const npmPlatform = PLATFORM_TARGET_MAP[key];

  if (!npmPlatform) {
    return null;
  }

  return { platform, npmPlatform };
}

function copyRecursive(src: string, dest: string): void {
  const stat = statSync(src);

  if (stat.isDirectory()) {
    if (!existsSync(dest)) {
      mkdirSync(dest, { recursive: true });
    }

    const entries = readdirSync(src);
    for (const entry of entries) {
      copyRecursive(join(src, entry), join(dest, entry));
    }
  } else {
    copyFileSync(src, dest);
  }
}

function transformContextKey(text: string): string {
  return text
    .replace(new RegExp(`view\\s*==\\s*${VIEW_ID}\\b`, 'g'), `view == ${addDevSuffix(VIEW_ID)}`)
    .replace(/\b(\w+)(?=\s*==|\s*!=|\s|$)/g, (match) => {
      if (match.startsWith(CONTEXT_PREFIX) && !match.endsWith(DEV_SUFFIX)) {
        return addDevSuffix(match);
      }
      return match;
    });
}

function transformCommand(cmd: string): string {
  if (!cmd.startsWith(`${CONTEXT_PREFIX}.`)) return cmd;
  return cmd.replace(`${CONTEXT_PREFIX}.`, `${addDevSuffix(CONTEXT_PREFIX)}.`);
}

function transformTitle(title: string): string {
  if (title.startsWith(`${EXTENSION_DISPLAY_NAME}:`)) {
    return title.replace(`${EXTENSION_DISPLAY_NAME}:`, `${EXTENSION_DISPLAY_NAME} (Dev):`);
  }
  if (title.startsWith(`${CONTEXT_PREFIX}:`)) {
    return title.replace(`${CONTEXT_PREFIX}:`, `${CONTEXT_PREFIX} (Dev):`);
  }
  return title;
}

function applyDevTransformations(pkg: Record<string, unknown>): Record<string, unknown> {
  const transformed = { ...pkg };

  transformed.name = `${pkg.name}-dev`;
  transformed.displayName = addDevLabel(pkg.displayName as string);

  const contributes = transformed.contributes as Record<string, unknown>;
  if (!contributes) return transformed;

  if (contributes.viewsContainers) {
    const containers = contributes.viewsContainers as Record<string, unknown>;
    if (containers.activitybar) {
      containers.activitybar = (containers.activitybar as Array<{ id: string; title: string }>).map((container) => ({
        ...container,
        id: addDevSuffix(container.id),
        title: addDevLabel(container.title),
      }));
    }
  }

  if (contributes.views) {
    const views = contributes.views as Record<string, Array<{ id: string; name?: string }>>;
    const newViews: Record<string, unknown> = {};

    for (const [containerKey, viewList] of Object.entries(views)) {
      const newContainerKey = addDevSuffix(containerKey);
      newViews[newContainerKey] = viewList.map((view) => ({
        ...view,
        id: addDevSuffix(view.id),
        name: view.name ? addDevLabel(view.name) : undefined,
      }));
    }

    contributes.views = newViews;
  }

  if (contributes.viewsWelcome) {
    const viewsWelcome = contributes.viewsWelcome as Array<{ view: string; contents: string; when?: string }>;
    for (const welcome of viewsWelcome) {
      welcome.view = addDevSuffix(welcome.view);
    }
  }

  if (contributes.menus) {
    const menus = contributes.menus as Record<string, Array<{ when?: string; command?: string }>>;

    for (const menuList of Object.values(menus)) {
      for (const menu of menuList) {
        if (menu.when) {
          menu.when = transformContextKey(menu.when);
        }
        if (menu.command) {
          menu.command = transformCommand(menu.command);
        }
      }
    }
  }

  if (contributes.commands) {
    const commands = contributes.commands as Array<{ command: string; title?: string; enablement?: string }>;
    for (const cmd of commands) {
      cmd.command = transformCommand(cmd.command);
      if (cmd.title) {
        cmd.title = transformTitle(cmd.title);
      }
      if (cmd.enablement) {
        cmd.enablement = transformContextKey(cmd.enablement);
      }
    }
  }

  if (contributes.keybindings) {
    const keybindings = contributes.keybindings as Array<{ when?: string; command?: string }>;
    for (const binding of keybindings) {
      if (binding.when) {
        binding.when = transformContextKey(binding.when);
      }
      if (binding.command) {
        binding.command = transformCommand(binding.command);
      }
    }
  }

  return transformed;
}
