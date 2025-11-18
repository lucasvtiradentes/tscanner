#!/usr/bin/env node
const { existsSync } = require('node:fs');
const { join } = require('node:path');

const postinstallPath = join(__dirname, 'dist', 'scripts', 'safe-postinstall.js');

if (existsSync(postinstallPath)) {
  require(postinstallPath);
} else {
  console.log('Skipping postinstall - package not built yet (development mode)');
}
