import * as fs from 'node:fs';
import * as path from 'node:path';
import esbuild, { type BuildOptions } from 'esbuild';

const isDev = !process.env.CI;

const logger = console;

const projectRoot = path.resolve(__dirname, '../..');
const aiFixPrompt = fs.readFileSync(path.resolve(projectRoot, 'assets/prompts/fix-tscanner-issues.prompt.md'), 'utf8');

const extensionBuildOptions: BuildOptions = {
  entryPoints: ['src/extension.ts'],
  bundle: true,
  outfile: 'out/extension.js',
  external: ['vscode'],
  format: 'cjs',
  platform: 'node',
  target: 'node18',
  sourcemap: false,
  minify: false,
  logLevel: 'info',
  mainFields: ['module', 'main'],
  define: {
    __IS_DEV_BUILD__: isDev ? 'true' : 'false',
    __AI_FIX_PROMPT__: JSON.stringify(aiFixPrompt),
  },
  alias: {
    zod: require.resolve('zod'),
  },
};

async function build() {
  await esbuild.build(extensionBuildOptions);
  logger.log('Build complete!');
}

build().catch((err) => {
  logger.error(err);
  process.exit(1);
});
