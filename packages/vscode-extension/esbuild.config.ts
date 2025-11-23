import esbuild, { type BuildOptions } from 'esbuild';

const isDev = !process.env.CI;

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
  console.log('Build complete!');
}

build().catch((err) => {
  console.error(err);
  process.exit(1);
});
