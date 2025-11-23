import esbuild, { type BuildOptions } from 'esbuild';

const isDev = !process.env.CI;

const logger = console;

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
  define: {
    __IS_DEV_BUILD__: isDev ? 'true' : 'false',
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
